use crate::coding::boolean::MinecraftBoolean;
use crate::coding::signed_byte::MinecraftSignedByte;
use crate::coding::string::MinecraftString;
use crate::coding::unsigned_byte::MinecraftUnsignedByte;

#[derive(Debug)]
pub struct ClientSettings {
    /// guarrented to always be lowercase
    pub locale: MinecraftString,
    pub render_distance: MinecraftSignedByte,
    pub chat_mode: ChatMode,
    pub chat_colors_enabled: MinecraftBoolean,
    pub skin_parts_displayed: MinecraftUnsignedByte,
    pub main_hand: MainHand,
}

#[derive(PartialEq, Debug)]
pub enum ChatMode {
    Enabled = 0x00,
    CommandsOnly = 0x01,
    Hidden = 0x02,
}

#[derive(PartialEq, Debug)]
pub enum MainHand {
    Left = 0x00,
    Right = 0x01,
}
