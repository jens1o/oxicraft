use crate::packet::{Packet, PacketData};
use crate::varint::{ReadVarint, ToVarint};
use std::char;
use std::io::{self, Read};
use std::net::{SocketAddr, TcpStream};

pub enum ConnectionState {
    Unknown,
    Handshaking,
}

pub struct Connection {
    pub ip_address: SocketAddr,
    pub tcp_stream: TcpStream,
    pub state: ConnectionState,
}

impl Connection {
    pub fn from_tcp_stream(stream: TcpStream) -> io::Result<Connection> {
        Ok(Connection {
            ip_address: stream.peer_addr()?,
            tcp_stream: stream,
            state: ConnectionState::Unknown,
        })
    }

    pub fn do_handshake(&mut self) -> io::Result<()> {
        let data_packet = self.read_data_packet()?;
        // ensure it is the Handshake packet that was sent by the client
        assert!(data_packet.packet_id == 0x0);

        self.state = ConnectionState::Handshaking;

        if let PacketData::Data(mut packet_data) = data_packet.data {
            info!("{:?}", packet_data);

            // read protocol version
            // FIXME: This somehow returns tooo great values
            let protocol_version = packet_data.read_varint()?;

            info!("Protocol version: {:?}", protocol_version);
            info!(
                "{:?}",
                packet_data
                    .iter()
                    .map(|x| char::from_u32(*x as u32).unwrap())
                    .collect::<Vec<_>>()
            );
        }

        Ok(())
    }

    pub fn read_data_packet(&mut self) -> io::Result<Packet> {
        let length = self.tcp_stream.read_varint()?;
        ensure_data_size(length)?;

        // we now can ensure it is a positive number, thus cast it
        let length: u32 = length as u32;

        let packet_id = self.tcp_stream.read_varint()?;

        let mut buffer = vec![0; (length as usize) - 2032];
        self.tcp_stream.read_exact(&mut buffer)?;

        let packet = Packet {
            length,
            packet_id,
            data: PacketData::Data(buffer),
        };

        trace!("Received data packet: {:?}", packet);

        Ok(packet)
    }
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
