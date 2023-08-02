use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use server::display_game;

pub fn run(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(ip)?;

    let (state_send, state_recv) = mpsc::channel();
    let (command_send, command_recv) = mpsc::channel();

    thread::spawn(move || handle_connection(stream, state_send, command_recv));

    display_game(state_recv, command_send)
}

fn handle_connection<T: Write + Read>(
    mut stream: T,
    state_sender: Sender<Vec<u8>>,
    command_receiver: Receiver<Vec<u8>>,
) {
    loop {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut length_bytes = [0; std::mem::size_of::<u32>()];
        buf_reader.read_exact(&mut length_bytes).unwrap();
        let mut msg_buf = buf_reader.take(u32::from_be_bytes(length_bytes) as u64);
        let mut msg = Vec::new();
        msg_buf.read_to_end(&mut msg).unwrap();
        if state_sender.send(msg).is_err() {
            break;
        }
        let response_msg = [0x04]; // TODO SEND COMMANDS
        if stream.write_all(&response_msg).is_err() {
            break;
        }
        // if let Ok(command) = command_receiver.try_recv().unwrap() {
        //     length_bytes = u32::to_be_bytes(command.len() as u32);
        //     let mut response_msg =
        //     let response_msg = [constants::MSG_END];
        //     if stream.write_all(&response_msg).is_err() {
        //         break;
        //     }
        // }
    }
}
