use super::varint::Varint;
use super::Decodeable;
use crate::client_status::ClientStatus;
use std::collections::VecDeque;
use std::io;

impl Decodeable<ClientStatus, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<ClientStatus, io::Error> {
        let raw_value: Varint = self.decode()?;

        if raw_value == 0x00 {
            Ok(ClientStatus::PerformRespawn)
        } else if raw_value == 0x01 {
            Ok(ClientStatus::RequestStats)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Client status is either 0 or 1, garbage given!",
            ))
        }
    }
}

// TODO: Add tests
