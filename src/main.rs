#[macro_use]
extern crate log;
extern crate simplelog;

mod connection;
mod packet;
mod varint;

use crate::connection::Connection;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use std::io;
use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let mut connection = Connection::from_tcp_stream(stream)?;

    info!("New connection from {}!", connection.ip_address);

    connection.do_handshake()?;

    Ok(())
}

fn main() -> io::Result<()> {
    SimpleLogger::init(LevelFilter::Trace, Config::default()).unwrap();

    info!("Started logging");

    let listener = TcpListener::bind("127.0.0.1:25565")?;

    info!("Started listening on {}", listener.local_addr()?);

    for incoming_stream in listener.incoming() {
        handle_connection(incoming_stream?)?;
    }

    Ok(())
}
