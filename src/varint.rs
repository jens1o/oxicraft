use std::io::{self, Read};
use std::net::TcpStream;

type Varint = i32;

pub trait ReadVarint<E> {
    fn read_varint(&mut self) -> Result<Varint, E>;
}

pub trait ToVarint {
    fn to_varint(&self) -> Vec<Varint>;
}

impl ReadVarint<io::Error> for TcpStream {
    fn read_varint(&mut self) -> Result<Varint, io::Error> {
        // see https://wiki.vg/Protocol#VarInt_and_VarLong
        let mut num_reads: u8 = 0;
        let mut result: Varint = 0;

        loop {
            let mut buffer = [0; 1];
            self.read_exact(&mut buffer)?;
            num_reads += 1;

            let value: i32 = (buffer[0] as i32) & 0b01111111;
            result |= value << (7 * num_reads);

            if buffer[0] & 0b10000000 == 0 {
                break;
            }
        }

        Ok(result)
    }
}

impl ReadVarint<io::Error> for Vec<u8> {
    fn read_varint(&mut self) -> Result<Varint, io::Error> {
        // see https://wiki.vg/Protocol#VarInt_and_VarLong
        let mut num_reads: u8 = 0;
        let mut result: Varint = 0;

        let mut vector = self.clone();

        vector.reverse();

        loop {
            let item = vector.pop().unwrap();

            num_reads += 1;

            let value: i32 = (item as i32) & 0b01111111;
            result |= value << (7 * num_reads);

            if item & 0b10000000 == 0 {
                break;
            }
        }

        trace!("Number of reads: {}", num_reads);

        Ok(result)
    }
}

impl ToVarint for i32 {
    fn to_varint(&self) -> Vec<Varint> {
        let mut remaining = self.clone();
        let mut result = Vec::with_capacity(7);

        loop {
            let part = remaining & 0x7F; // First 7 bits
            remaining >>= 7;
            if remaining == 0 {
                result.push(part);
                break;
            } else {
                result.push(part | 0x80);
            }
        }

        result
    }
}
