use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct Packet {
    pub length: u32,
    pub packet_id: i32,
    pub data: PacketData,
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

#[derive(Debug)]
pub enum PacketData {
    Command,
    Data(VecDeque<u8>),
    Message(String),
}
