use crossbeam_channel::{Receiver, Sender};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle_connection(
    mut stream: TcpStream,
    state_receiver: Receiver<Vec<u8>>,
    command_sender: Sender<Vec<u8>>,
) {
    loop {
        //TODO
        let mut state_msg = state_receiver.recv().unwrap();
        state_msg.push(b'%');
        if stream.write_all(&state_msg).is_err() {
            break;
        }
        let mut buf_reader = BufReader::new(&mut stream);
        let mut command_msg = Vec::new();
        if buf_reader.read_until(b'%', &mut command_msg).is_err() {
            break;
        }
        command_sender.send(command_msg).unwrap();
    }
}
