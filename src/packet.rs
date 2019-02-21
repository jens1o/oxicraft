use crate::coding::varint::Varint;
use crate::coding::Encodeable;
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Packet {
    pub length: usize,
    pub packet_id: Varint,
    pub data: PacketData,
}

impl Packet {
    pub fn from_id_and_data(packet_id: Varint, data: PacketData) -> Packet {
        Packet {
            length: 0,
            packet_id,
            data,
        }
    }

    pub fn send(&mut self, connection: &mut TcpStream) -> io::Result<()> {
        let packet_id_varint = self.packet_id.encode();

        let length = packet_id_varint.len() + self.data.len();
        self.length = length;

        let length_varint: VecDeque<u8> = Varint(length as i32).encode();

        // acquiring more capacity, because we also need to have enough space for our varint
        let mut write_buffer: Vec<u8> = Vec::with_capacity(length + length_varint.len());

        // The packet consists of a Varint that represents the size of this package, the package id and the data.

        write_buffer.extend(length_varint);
        write_buffer.extend(packet_id_varint);

        write_buffer.extend(self.data.to_bytes());

        connection.write_all(&write_buffer)?;

        debug!("Sent data {:?}", self);

        Ok(())
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

impl PacketData {
    pub fn len(&self) -> usize {
        if let PacketData::Data(packet_data) = self {
            return packet_data.len();
        }

        unimplemented!();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if let PacketData::Data(packet_data) = self {
            // TODO: Optimize this.
            return packet_data.iter().cloned().collect::<Vec<_>>();
        }

        unimplemented!();
    }
}
