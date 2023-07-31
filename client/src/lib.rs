mod display;

use std::io::{BufRead, BufReader};
use std::net::TcpStream;

//use display::print_game;

pub fn run(_path: &str, _port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:8888")?;

    handle_connection(stream);

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut line = Vec::new();
        buf_reader.read_until(b'%', &mut line).unwrap();

        println!("Response: {:?}", String::from_utf8(line));
    }
}
