use std::collections::VecDeque;
use std::fmt;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Packet {
    pub length: u32,
    pub packet_id: i32,
    pub data: PacketData,
}

impl Packet {
    pub fn new(packet_id: i32, data: PacketData) -> Packet {
        Packet {
            length: 0,
            packet_id,
            data,
        }
    }

    pub fn send(&mut self, connection: TcpStream) {
        // connection.write_all(mut buf: &[u8])
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Packet {{ length: {:?}, packet_id: 0x{:X?}, data: {:?} }}",
            self.length, self.packet_id, self.data
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum PacketData {
    Command,
    Data(VecDeque<u8>),
    Message(String),
}
