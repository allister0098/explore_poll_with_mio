use std::error::Error;

use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token, Registry};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;

const SERVER: Token = Token(0);

const DATA: &[u8] = b"Hello world\n";

fn main() -> Result<(), Box<dyn Error>>{
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the TCP server socket.
    let addr = "127.0.0.1:13265".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Map of `Token` -> `TcpStream`.
    let mut connections = HashMap::new();
    // Unique token for each incoming connection.
    let mut unique_token = Token(SERVER.0 + 1);

    println!("You can connect to the server using `nc`:");
    println!(" $ nc 127.0.0.1 9000");
    println!("You'll see our welcome message and anything you type we'll be printed here.");

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let mut (connection, address) = match server.accept() {
                        Ok((connection, address)) => (connection, address),
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our listner has no more incoming connections queued,
                            // so we can return to polling and wait for some more.
                        }
                        Err(e) => {
                            // If it was any other kind of error, something went wrong and we terminate with an error.
                            return Err(e);
                        }
                    };

                    println!("Accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE)
                    )?;

                    connections.insert(token, connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        println!("socket is writable");
                    }
                    if event.is_readable() {
                        println!("socket is readable");
                    }

                    return Ok(());
                }
                _ => unreachable!(),
            }
        }
    }
}
