pub mod handshake;

use crate::coding::short::UnsignedShort;
use crate::coding::string::ReadString;
use crate::coding::varint::Varint;
use crate::coding::{Decodeable, Encodeable};
use crate::entity::get_new_eid;
use crate::packet::{Packet, PacketData};
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, Read};
use std::net::{AddrParseError, IpAddr, SocketAddr, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;
use std::u16;

static CONNECTION_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct ConnectionId {
    pub value: usize,
}

impl ConnectionId {
    pub fn new() -> ConnectionId {
        ConnectionId {
            value: CONNECTION_COUNTER.fetch_add(1, Ordering::SeqCst),
        }
    }
}

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.value)
    }
}

#[derive(PartialEq)]
pub enum ConnectionState {
    Unknown,
    Handshaking,
    Play,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Unknown
    }
}

pub struct Connection {
    pub connection_id: ConnectionId,
    pub start_time: SystemTime,
    pub ip_address: SocketAddr,
    pub tcp_stream: TcpStream,
    pub state: ConnectionState,
    /// the protocol version specified by the client
    pub protocol_version: Option<u16>,
    /// the server address used to connect to this specified by the client
    pub server_address: Option<SocketAddr>,
    pub username: Option<String>,
}

impl Connection {
    pub fn from_tcp_stream(stream: TcpStream) -> io::Result<Connection> {
        Ok(Connection {
            connection_id: ConnectionId::new(),
            start_time: SystemTime::now(),
            ip_address: stream.peer_addr()?,
            tcp_stream: stream,
            state: Default::default(),
            protocol_version: Default::default(),
            server_address: Default::default(),
            username: Default::default(),
        })
    }

    pub fn do_handshake(&mut self) -> io::Result<handshake::HandshakeNextState> {
        info!(
            "Processing handshake for connection {}.",
            self.connection_id
        );

        let benchmark_start = SystemTime::now();
        let data_packet = self.read_data_packet()?;

        // ensure it is the Handshake packet that was sent by the client
        if data_packet.packet_id != 0x00 {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Client indicates it is not looking for a handshake, rather than an existing connection."
            ));
        }

        self.state = ConnectionState::Handshaking;

        if let PacketData::Data(mut packet_data) = data_packet.data {
            // read protocol version
            let protocol_version: Varint = packet_data.decode()?;

            if protocol_version.0 > i32::from(u16::max_value()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Too great protocol version supplied.",
                ));
            } else if protocol_version.0 < i32::from(u16::min_value()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Too tiny protocol version supplied.",
                ));
            }

            trace!("Protocol version: {:?}", protocol_version);

            self.protocol_version = Some(protocol_version.0 as u16);

            let mut server_address = packet_data.read_string(255)?;

            if server_address == "localhost" {
                // bugfix to avoid unnecessary error
                server_address = "127.0.0.1".to_owned();
            }

            let ip_addr: Result<IpAddr, AddrParseError> = server_address.parse();

            if ip_addr.is_err() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid ip address given.",
                ));
            }

            let ip_addr = ip_addr.unwrap();
            let port: UnsignedShort = packet_data.decode()?;
            let socket_addr = SocketAddr::new(ip_addr, port);

            self.server_address = Some(socket_addr);

            trace!("Client used {} to connect.", &socket_addr);

            let next_state = match packet_data.decode()? {
                Varint(1) => handshake::HandshakeNextState::Status,
                Varint(2) => handshake::HandshakeNextState::Login,
                x => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("The next state may only be 1 or 2, {} given.", x.0),
                    ));
                }
            };

            info!("Next state of {}: {:?}", self.connection_id, next_state);

            if !packet_data.is_empty() {
                warn!("The handshake packet sent by the client contains more data than expected. Rest of data: {:?}", packet_data);
            }

            let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

            trace!(
                "Handling of handshake package took {:?}.",
                benchmark_duration
            );

            Ok(next_state)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Handshake packet does not contain any data?!",
            ))
        }
    }

    pub fn send_status(&mut self) -> io::Result<()> {
        assert!(self.state == ConnectionState::Handshaking);
        let benchmark_start = SystemTime::now();

        info!("Sending status to connection {}.", self.connection_id);

        // the package id for this(empty) package is 0x00.
        assert!(self.read_data_packet()?.packet_id == 0x00);

        let response = serde_json::to_string(&handshake::mock_slp())?.to_owned();

        let response_packet: Packet =
            Packet::from_id_and_data(Varint(0x00), PacketData::Data(response.encode()));

        response_packet.send(&mut self.tcp_stream)?;

        let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();
        info!(
            "Sent status to {} (took {:?}).",
            self.connection_id, benchmark_duration
        );

        // now, the client sends a data packet (basically to ping us), with a long we need to pong back.
        if let PacketData::Data(packet_data) = self.read_data_packet()?.data {
            let pong_packet = Packet::from_id_and_data(Varint(0x01), PacketData::Data(packet_data));

            pong_packet.send(&mut self.tcp_stream)?;
        } else {
            unreachable!();
        }

        Ok(())
    }

    /// Performs the necessary step to let the user spawn on a flat-map with grass all over the place.
    pub fn login(&mut self) -> io::Result<()> {
        // C->S Login Start
        let login_packet = self.read_data_packet()?;

        assert!(login_packet.packet_id == 0x00);
        if let PacketData::Data(mut packet_data) = login_packet.data {
            let username = packet_data.read_string(16)?;

            info!("New login from {} ({})!", &username, self.connection_id);

            // UUID-4 (with hyphens) of jens1o
            let player_uuid = "8e383e9f-608e-4556-97c9-61312c741ea0".to_owned();

            // S->C Login Success Packet

            info!(
                "Sending login success packet to {} ({}).",
                &username, self.connection_id
            );

            let login_success_packet = Packet::from_id_and_data(
                Varint(0x02),
                PacketData::Data(super::build_package_data!(player_uuid, username)),
            );

            self.username = Some(username);

            login_success_packet.send(&mut self.tcp_stream)?;
            info!("Sent login success package.");

            self.state = ConnectionState::Play;

            // S->C Join Game

            // TODO: refactor

            let entity_id = get_new_eid();
            let gamemode: u8 = 1; // Creative
            let dimension: i32 = 0; // Overworld
            let difficulty: u8 = 0; // Peaceful
            let max_players: u8 = 20;
            let level_type: String = String::from("flat");
            let reduced_debug_info: bool = false;

            let packet = Packet::from_id_and_data(
                Varint(0x25),
                PacketData::Data(super::build_package_data!(
                    entity_id,
                    gamemode,
                    dimension,
                    difficulty,
                    max_players,
                    level_type,
                    reduced_debug_info
                )),
            );

            packet.send(&mut self.tcp_stream)?;
        }

        Ok(())
    }

    pub fn read_data_packet(&mut self) -> io::Result<Packet> {
        let length: Varint = self.tcp_stream.decode()?;
        ensure_data_size(length.0)?;

        // we now can ensure it is a positive number, thus cast it
        let length: usize = length.0 as usize;

        let packet_id: Varint = self.tcp_stream.decode()?;

        // read the package contents. We need to read the amount that was given, without the package id.
        let data_length = length - packet_id.encode().len();

        trace!(
            "Reading {} bytes to read content of package with id {:#X}.",
            data_length,
            packet_id.0
        );

        let mut buffer = vec![0; data_length];
        self.tcp_stream.read_exact(&mut buffer)?;

        let packet = Packet {
            length,
            packet_id,
            data: PacketData::Data(VecDeque::from(buffer)),
        };

        trace!("Received data packet: {:?}", packet);

        Ok(packet)
    }
}

#[inline]
pub fn ensure_data_size(size: i32) -> io::Result<()> {
    if size <= 0 {
        warn!(r#"Received "data" with size of {}."#, size);
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Received packet with data size of zero (or less).",
        ))
    } else {
        Ok(())
    }
}
