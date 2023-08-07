mod display;

use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn run(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(ip)?;

    let (state_send, state_recv) = mpsc::channel();
    let (command_send, command_recv) = mpsc::channel();

    thread::spawn(move || handle_connection(stream, state_send, command_recv));

    display::display_game(state_recv, command_send)
}

fn handle_connection<T: Write + Read>(
    mut stream: T,
    state_sender: Sender<Vec<u8>>,
    command_receiver: Receiver<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let mut buf_reader = BufReader::new(&mut stream);
        // read msg length
        let mut length_bytes = [0; std::mem::size_of::<u32>()];
        buf_reader.read_exact(&mut length_bytes)?;

        // read state
        let length = u32::from_be_bytes(length_bytes);
        let mut msg_buf = buf_reader.take(length.into());
        let mut msg = Vec::new();
        msg_buf.read_to_end(&mut msg)?;

        // send state to display
        state_sender.send(msg)?;

        // receive last command from display
        let mut response_msg = command_receiver.recv()?;
        while let Ok(msg) = command_receiver.try_recv() {
            response_msg = msg;
        }

        // send command to server
        stream.write_all(&response_msg)?;
    }
}
