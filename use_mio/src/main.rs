use std::io;

use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    let mut client = TcpStream::connect(addr)?;
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let connection = server.accept()?;
                    println!("server recv a connection");
                    drop(connection);
                },
                CLIENT => {
                    if event.is_writable() {
                        println!("client write");
                    }  
                    if event.is_readable() {
                        println!("client read");
                    }

                    return Ok(());
                },
                _ => unreachable!()
            }
        }
    }

}
