use crate::packet::{Packet, PacketData};
use crate::short::ReadUnsignedShort;
use crate::string::ReadString;
use crate::varint::ReadVarint;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::net::{AddrParseError, IpAddr, SocketAddr, TcpStream};
use std::u16;

pub enum ConnectionState {
    Unknown,
    Handshaking,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Unknown
    }
}

pub struct Connection {
    pub ip_address: SocketAddr,
    pub tcp_stream: TcpStream,
    pub state: ConnectionState,
    /// the protocol version specified by the client
    pub protocol_version: Option<u16>,
    /// the server address used to connect to this specified by the client
    pub server_address: Option<SocketAddr>,
}

impl Connection {
    pub fn from_tcp_stream(stream: TcpStream) -> io::Result<Connection> {
        Ok(Connection {
            ip_address: stream.peer_addr()?,
            tcp_stream: stream,
            state: Default::default(),
            protocol_version: Default::default(),
            server_address: Default::default(),
        })
    }

    pub fn do_handshake(&mut self) -> io::Result<()> {
        let data_packet = self.read_data_packet()?;

        // ensure it is the Handshake packet that was sent by the client
        if data_packet.packet_id != 0x0 {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Client indicates it is not looking for a handshake, rather than an existing connection."
            ));
        }

        self.state = ConnectionState::Handshaking;

        if let PacketData::Data(mut packet_data) = data_packet.data {
            // read protocol version
            let protocol_version = packet_data.read_varint()?;

            if protocol_version > u16::max_value() as i32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Too great protocol version supplied.",
                ));
            } else if protocol_version < u16::min_value() as i32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Too tiny protocol version supplied.",
                ));
            }

            trace!("Protocol version: {:?}", protocol_version);

            self.protocol_version = Some(protocol_version as u16);

            let server_address = packet_data.read_string(255)?;

            let ip_addr: Result<IpAddr, AddrParseError> = server_address.parse();

            if ip_addr.is_err() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid ip address given.",
                ));
            }

            let ip_addr = ip_addr.unwrap();
            let port = packet_data.read_unsigned_short()?;
            let socket_addr = SocketAddr::new(ip_addr, port);

            self.server_address = Some(socket_addr);

            trace!("Client used {} to connect.", &socket_addr);

            let next_state = match packet_data.read_varint()? {
                1 => HandshakeNextState::Status,
                2 => HandshakeNextState::Login,
                x => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("The next state may only be 1 or 2, {} given.", x),
                    ));
                }
            };

            info!("Next state: {:?}", next_state);

            trace!("Rest of data of handshake packet: {:?}", packet_data);
        }

        Ok(())
    }

    pub fn read_data_packet(&mut self) -> io::Result<Packet> {
        let length = self.tcp_stream.read_varint()?;
        ensure_data_size(length)?;

        // we now can ensure it is a positive number, thus cast it
        let length: u32 = length as u32;

        let packet_id = self.tcp_stream.read_varint()?;

        let mut buffer = vec![0; length as usize];
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

#[derive(Debug)]
pub enum HandshakeNextState {
    Status = 1,
    Login = 2,
}

#[inline]
pub fn ensure_data_size(size: i32) -> io::Result<()> {
    if size <= 0 {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Received packet with data size of zero (or less).",
        ))
    } else {
        Ok(())
    }
}
