#![feature(integer_atomics)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate log;
extern crate serde_json;
extern crate simplelog;
#[macro_use]
extern crate serde_derive;

mod client_settings;
mod coding;
mod connection;
mod difficulty;
mod dimension;
mod entity;
mod location;
mod packet;
mod player;
mod plugin_message;
mod world;

use crate::connection::{handshake::HandshakeNextState, Connection};
use crate::location::Location;
use crate::player::Player;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use std::io;
use std::net::{TcpListener, TcpStream};
use std::time::SystemTime;

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let mut connection = Connection::from_tcp_stream(stream)?;
    let start_time = connection.start_time;
    let connection_id = connection.connection_id;

    info!(
        "New connection from {} ({})!",
        connection.ip_address, connection_id
    );

    let next_state = connection.do_handshake()?;

    match next_state {
        HandshakeNextState::Status => {
            connection.send_status()?;
        }
        HandshakeNextState::Login => {
            let (username, uuid) = connection.prepare_login()?;

            let mut player = Player::from_basic_data(connection, username, uuid);

            player.send_login_success()?;
            player.send_join_game()?;
            player.broadcast_server_name()?;
            player.set_spawn_location(Location::default())?;
            // TODO: Find better fitting values
            player.set_player_abilities(0b1101 /* flying and creative */, 0.05, 0.1)?;
            player.read_client_settings()?;

            // TODO: Make this dynamic, as this is an optional package.
            let mut plugin_message = player.receive_plugin_message()?;

            debug!("Received plugin message: {:?}", plugin_message);

            if plugin_message.channel() == "minecraft:brand" {
                info!("Client is called \"{}\".", plugin_message.data_stringify()?);
            } else {
                warn!(
                    "Unknown plugin message channel {}!",
                    plugin_message.channel()
                );
            }

            // Tell client they're ready to spawn.
            let teleport_id = player.set_location(&Location::default(), 0.0, 0.0, 0b0)?;

            // C->S Teleport Confirm
            player.expect_teleport_confirm(teleport_id)?;

            // Send position once again to confirm
            player.set_location(&Location::default(), 0.0, 0.0, 0b0)?;

            info!("Client login successfully done.");
        }
    }

    let connect_duration = SystemTime::now().duration_since(start_time).unwrap();

    info!(
        "Connection terminated; client {} was connected for {:?}.",
        connection_id, connect_duration
    );

    Ok(())
}

fn main() -> io::Result<()> {
    SimpleLogger::init(
        if cfg!(debug_assertions) {
            LevelFilter::Trace
        } else {
            LevelFilter::Info
        },
        Config::default(),
    )
    .unwrap();

    info!("Started logging.");

    let listener = TcpListener::bind("0.0.0.0:25565")?;

    info!("Started listening on {}.", listener.local_addr()?);

    for incoming_stream in listener.incoming() {
        handle_connection(incoming_stream?)?;
    }

    Ok(())
}

#[macro_export]
macro_rules! build_package_data {
    ( $( $x: expr ),* ) => {
        {
            #[allow(unused_imports)]
            use crate::coding::Encodeable; // wrongly claimed for not being used.
            use std::collections::VecDeque;

            let mut package_data: VecDeque<u8> = VecDeque::with_capacity(
                (
                    $(
                        $x.byte_length() +
                    )*
                0) as usize
            );

            $(
                package_data.extend($x.encode());
            )*

            package_data
        }
    }
}
