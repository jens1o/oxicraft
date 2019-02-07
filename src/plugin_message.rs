use crate::coding::string::{MinecraftString, ReadString};
use std::collections::VecDeque;
use std::io;

#[derive(Debug)]
pub struct PluginMessage {
    /// The channel this plugin message has been sent to, e.g. `minecraft:brand`.
    /// It is either starting with `minecraft:` or with a custom namespace.
    /// It is guarenteed to start with a namespace(so `thing` is saved as `minecraft:thing`).
    channel: MinecraftString,
    /// Holding the raw data
    data: VecDeque<u8>,
    origin: PluginMessageOrigin,
}

impl PluginMessage {
    pub fn new(
        channel: MinecraftString,
        data: VecDeque<u8>,
        origin: PluginMessageOrigin,
    ) -> PluginMessage {
        let mut channel = channel;
        if !channel.contains(':') {
            channel = "minecraft:".to_owned() + &channel;
        }

        PluginMessage {
            channel,
            data,
            origin,
        }
    }

    #[inline(always)]
    pub fn channel(&self) -> &MinecraftString {
        &self.channel
    }

    #[inline(always)]
    pub fn data(&self) -> &VecDeque<u8> {
        &self.data
    }

    pub fn data_stringify(&mut self) -> io::Result<MinecraftString> {
        self.data.read_string(32767)
    }

    #[inline(always)]
    pub fn origin(&self) -> &PluginMessageOrigin {
        &self.origin
    }

    #[inline(always)]
    pub fn is_minecraft(&self) -> bool {
        self.channel.starts_with("minecraft:")
    }

    #[inline(always)]
    pub fn get_channel_namespace(&self) -> &str {
        self.channel
            .split_at(
                self.channel
                    .find(':')
                    .expect("Channel does not contain namespace!"),
            )
            .0
    }
}

/// Clarifies where this plugin message comes from
#[derive(Debug)]
pub enum PluginMessageOrigin {
    /// Plugin Message is originating from the server
    Server,
    /// This Plugin Message has been received by the client.
    Client,
}
