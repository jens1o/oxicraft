#[macro_use]
extern crate log;
extern crate serde_json;
extern crate simplelog;
#[macro_use]
extern crate serde_derive;

mod coding;
mod connection;
mod entity;
mod packet;
mod world;

use crate::connection::{handshake::HandshakeNextState, Connection};
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use std::io;
use std::net::{TcpListener, TcpStream};
use std::time::SystemTime;

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let mut connection = Connection::from_tcp_stream(stream)?;

    info!(
        "New connection from {} ({})!",
        connection.ip_address, connection.connection_id
    );

    let next_state = connection.do_handshake()?;

    match next_state {
        HandshakeNextState::Status => {
            connection.send_status()?;
        }
        HandshakeNextState::Login => {
            connection.login()?;
        }
    }

    let connect_duration = SystemTime::now()
        .duration_since(connection.start_time)
        .unwrap();

    info!(
        "Connection terminated; client {} was connected for {:?}.",
        connection.connection_id, connect_duration
    );

    Ok(())
}

fn main() -> io::Result<()> {
    SimpleLogger::init(LevelFilter::Trace, Config::default()).unwrap();

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
            use crate::coding::Encodeable;

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
