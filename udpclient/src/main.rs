use std::{io, net::UdpSocket};

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    socket.connect("127.0.0.1:8080")?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;

        let mut buf = [0u8; 1500];
        let (_size, _src) = socket.recv_from(&mut buf)?;
        println!("recv: {}", String::from_utf8(Vec::from(buf)).expect("Can not write buf as string"));
    }
}
