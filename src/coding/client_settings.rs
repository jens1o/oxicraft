use super::string::{MinecraftString, ReadString};
use super::Decodeable;
use super::{
    boolean::MinecraftBoolean, signed_byte::MinecraftSignedByte,
    unsigned_byte::MinecraftUnsignedByte,
};
use crate::client_settings::{ChatMode, ClientSettings, MainHand};
use std::collections::VecDeque;
use std::io;

impl Decodeable<ClientSettings, io::Error> for VecDeque<u8> {
    fn decode(&mut self) -> Result<ClientSettings, io::Error> {
        let locale: MinecraftString = self.read_string(16)?.to_lowercase();
        let render_distance: MinecraftSignedByte = self.decode()?;
        let chat_mode: ChatMode = self.decode()?;
        let chat_colors_enabled: MinecraftBoolean = self.decode()?;
        let skin_parts_displayed: MinecraftUnsignedByte = self.decode()?;
        let main_hand: MainHand = self.decode()?;

        Ok(ClientSettings {
            locale,
            render_distance,
            chat_mode,
            chat_colors_enabled,
            skin_parts_displayed,
            main_hand,
        })
    }
}

// TODO: Add tests
