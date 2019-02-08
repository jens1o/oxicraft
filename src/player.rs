use crate::client_settings::ClientSettings;
use crate::coding::float::MinecraftFloat;
use crate::coding::signed_byte::MinecraftSignedByte;
use crate::coding::varint::Varint;
use crate::coding::Decodeable;
use crate::connection::{Connection, ConnectionState};
use crate::entity::{get_new_eid, get_new_teleport_id};
use crate::location::Location;
use crate::packet::{Packet, PacketData};
use crate::plugin_message::{PluginMessage, PluginMessageOrigin};
use crate::world::World;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::VecDeque;
use std::f64;
use std::io;

pub struct Player {
    connection: Connection,
    username: String,
    // TODO: Create own struct for this
    uuid: String,
    client_settings: Option<ClientSettings>,
    entitity_id: usize,
    spawn_location: Location,
    current_location: Location,
    current_world: World,
}

impl Player {
    pub fn from_basic_data(connection: Connection, username: String, uuid: String) -> Player {
        Player {
            connection,
            username,
            uuid,
            client_settings: None,
            entitity_id: get_new_eid(),
            spawn_location: Location::default(),
            current_location: Location::default(),
            current_world: World::default(),
        }
    }

    pub fn spawn_location(&self) -> &Location {
        &self.spawn_location
    }

    /// Helper function for sending a package to this player.
    #[inline(always)]
    fn send_packet(&mut self, packet: &mut Packet) -> io::Result<()> {
        packet.send(&mut self.connection.tcp_stream)
    }

    /// S->C Login Success Packet
    ///
    /// This is immediately send after trying to log-in when online-mode is disabled.
    ///
    /// Sets the connection state to `Play` after sending.
    pub fn send_login_success(&mut self) -> io::Result<()> {
        trace!(
            "Sending login success packet to {} ({}).",
            self.username,
            self.connection.connection_id
        );

        let mut login_success_packet = Packet::from_id_and_data(
            Varint(0x02),
            PacketData::Data(super::build_package_data!(self.uuid, self.username)),
        );

        self.send_packet(&mut login_success_packet)?;

        // set the connection state to `Play` as we sent this package.
        self.connection.state = ConnectionState::Play;

        Ok(())
    }

    /// S->C Join Game
    ///
    /// Informs the client about the general overview on the world they will be joining.
    pub fn send_join_game(&mut self) -> io::Result<()> {
        let mut join_game_packet = Packet::from_id_and_data(
            Varint(0x25),
            PacketData::Data(super::build_package_data!(
                self.entitity_id as i32,
                self.current_world.gamemode,
                self.current_world.dimension,
                self.current_world.difficulty,
                20_u8, // max players TODO: Make this global
                self.current_world.level_type,
                false // reduced debug info?
            )),
        );

        self.send_packet(&mut join_game_packet)
    }

    pub fn broadcast_server_name(&mut self) -> io::Result<()> {
        let plugin_message = PluginMessage::new(
            "minecraft:brand".to_owned(),
            VecDeque::from(b"oxicraft".to_vec()),
            PluginMessageOrigin::Server,
        );

        self.send_plugin_message(plugin_message)
    }

    pub fn send_plugin_message(&mut self, plugin_message: PluginMessage) -> io::Result<()> {
        let mut packet = Packet::from_id_and_data(
            Varint(0x19),
            PacketData::Data(super::build_package_data!(plugin_message)),
        );

        self.send_packet(&mut packet)
    }

    /// S->C Spawn Location
    ///
    /// Notify the client where their spawn location will be(and also set the compass point to this location).
    pub fn set_spawn_location(&mut self, spawn_location: Location) -> io::Result<()> {
        self.spawn_location = spawn_location;

        // Send spawn location to client
        let mut packet = Packet::from_id_and_data(
            Varint(0x49),
            PacketData::Data(super::build_package_data!(self.spawn_location)),
        );

        self.send_packet(&mut packet)
    }

    /// S->C Player Abilities
    pub fn set_player_abilities(
        &mut self,
        flags: MinecraftSignedByte,
        flying_speed: MinecraftFloat,
        walking_speed: MinecraftFloat,
    ) -> io::Result<()> {
        let mut packet = Packet::from_id_and_data(
            Varint(0x2E),
            PacketData::Data(super::build_package_data!(
                flags,
                flying_speed,
                walking_speed
            )),
        );

        trace!(
            "Sending player abilities to connection {}…",
            self.connection.connection_id
        );

        self.send_packet(&mut packet)
    }

    /// C->S Client settings
    pub fn read_client_settings(&mut self) -> io::Result<()> {
        let client_settings_packet = self.connection.read_data_packet()?;

        // FIXME: Accept this, even when not send in ("right") order
        assert_eq!(client_settings_packet.packet_id, 0x04);

        if let PacketData::Data(mut packet_data) = client_settings_packet.data {
            let client_settings: ClientSettings = packet_data.decode()?;

            info!("Received client settings.");

            debug!("Client settings: {:?}", client_settings);

            self.client_settings = Some(client_settings);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Client settings packet does not contain any data?!",
            ))
        }
    }

    pub fn receive_plugin_message(&mut self) -> io::Result<PluginMessage> {
        let plugin_message_packet = self.connection.read_data_packet()?;

        assert_eq!(plugin_message_packet.packet_id, 0x0A);

        if let PacketData::Data(mut packet_data) = plugin_message_packet.data {
            let plugin_message: PluginMessage = packet_data.decode()?;

            Ok(plugin_message)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Client settings packet does not contain any data?!",
            ))
        }
    }

    pub fn set_location(
        &mut self,
        new_location: &Location,
        yaw: MinecraftFloat,
        pitch: MinecraftFloat,
        flags: MinecraftSignedByte,
    ) -> io::Result<Varint> {
        let teleport_id = get_new_teleport_id();

        let mut packet = Packet::from_id_and_data(
            Varint(0x32),
            PacketData::Data(super::build_package_data!(
                // need to be converted to MinecraftDouble.
                f64::from(new_location.x),
                f64::from(new_location.y),
                f64::from(new_location.z),
                yaw,
                pitch,
                flags,
                teleport_id
            )),
        );

        self.send_packet(&mut packet)?;

        Ok(teleport_id)
    }

    pub fn expect_teleport_confirm(&mut self, teleport_id: Varint) -> io::Result<()> {
        let teleport_confirm_package = self.connection.read_data_packet()?;

        if teleport_confirm_package.packet_id != 0x00 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Client didn't send an teleport confirm package.",
            ));
        }

        if let PacketData::Data(mut packet_data) = teleport_confirm_package.data {
            let given_teleport_id: Varint = packet_data.decode()?;

            if teleport_id == given_teleport_id {
                debug!("Teleport {:?} was confirmed by client!", teleport_id);
                return Ok(());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Teleport-ID by client didn't matched.",
        ))
    }
}

/// Implemented to match https://wiki.vg/Server_List_Ping#Response (sample values)
impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Player", 2)?;
        state.serialize_field("name", &self.username)?;
        state.serialize_field("id", &self.uuid)?;
        state.end()
    }
}
