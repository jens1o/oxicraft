use super::string::ReadString;
use super::{Decodeable, Encodeable};
use crate::plugin_message::{PluginMessage, PluginMessageOrigin};
use std::collections::VecDeque;
use std::io;

impl Decodeable<PluginMessage, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<PluginMessage, io::Error> {
        let channel = self.read_string(32767)?;
        let data: VecDeque<u8> = self.decode()?;
        let origin = PluginMessageOrigin::Client;

        Ok(PluginMessage::new(channel, data, origin))
    }
}

impl Encodeable for PluginMessage {
    fn encode(&self) -> VecDeque<u8> {
        crate::build_package_data!(self.channel(), self.data())
    }

    fn byte_length(&self) -> u8 {
        self.channel().byte_length() + self.data().byte_length()
    }
}
