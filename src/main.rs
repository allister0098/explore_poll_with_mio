use std::error::Error;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> Result<(), Box<dyn Error>>{
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let connection = server.accept();
                    drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        // ブロック無しでソケットに書き込み可能
                    }
                    if event.is_readable() {
                        // ブロックなしでソケットを読み込み可能
                    }

                    return Ok(());
                }
                _ => unreachable!(),
            }
        }
    }
}
