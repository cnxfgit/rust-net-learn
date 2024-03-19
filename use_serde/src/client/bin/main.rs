use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fatiled to read from stdin");

        let parts: Vec<&str> = input.trim_matches('\n').split(",").collect();
        let point = Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        };

        stream
            .write_all(serde_json::to_string(&point).unwrap().as_bytes())
            .expect("Fatiled to write");
        stream.write_all(b"\n")?;

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer)?;
        let input = String::from_utf8(Vec::from(buffer)).unwrap();

        if input == "" {
            eprintln!("empty response");
        } else {
            println!("response: {}", input);
        }
    }
}
