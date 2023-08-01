mod display;

use crossbeam_channel::Sender;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

pub fn run(_path: &str, _port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("0.0.0.0:8888")?;

    let (state_send, state_recv) = crossbeam_channel::unbounded();

    thread::spawn(move || handle_connection(stream, state_send));

    display::print_game(state_recv)
}

fn handle_connection(mut stream: TcpStream, state_sender: Sender<Vec<u8>>) {
    // TODO
    loop {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut msg = Vec::new();
        buf_reader.read_until(b'%', &mut msg).unwrap();
        if state_sender.send(msg).is_err() {
            break;
        }
        let response_msg = b"Client%";
        if stream.write_all(response_msg).is_err() {
            break;
        }
    }
}
