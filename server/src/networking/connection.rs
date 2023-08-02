use bus::{Bus, BusReader};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::constants;

struct Client<T: Write + Read> {
    stream: T,
    command_sender: Sender<Vec<u8>>,
    state_receiver: BusReader<Vec<u8>>,
}

pub fn handle_listen(
    addr: &str,
    command_sender: Sender<Vec<u8>>,
    state_bus: &Arc<Mutex<Bus<Vec<u8>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr)?;

    let mut connections: HashMap<u8, JoinHandle<()>> = HashMap::new();
    for mut stream in listener.incoming().flatten() {
        if let Some(id) = init_connection(&mut connections, &mut stream, constants::MAX_PLAYERS) {
            // thread for every connection
            let client = Client {
                stream,
                command_sender: command_sender.clone(),
                state_receiver: state_bus.lock().unwrap().add_rx(),
            };
            let handle = thread::spawn(move || {
                handle_connection(id, client);
            });
            connections.insert(id, handle);
        }
    }

    for (_, connection) in connections {
        connection.join().unwrap();
    }

    Ok(())
}

fn init_connection(
    connections: &mut HashMap<u8, JoinHandle<()>>,
    stream: &mut impl Write,
    max_players: u8,
) -> Option<u8> {
    connections.retain(|_, handle| !handle.is_finished());
    if connections.len() >= max_players as usize {
        stream.write_all(&constants::MAX_PLAYERS_MSG).unwrap();
        return None;
    }
    (1..max_players + 1).find(|&i| !connections.contains_key(&i))
}

fn handle_connection<T: Write + Read>(id: u8, mut client: Client<T>) {
    loop {
        let mut state_msg = client.state_receiver.recv().unwrap();
        while let Ok(msg) = client.state_receiver.try_recv() {
            state_msg = msg;
        }
        let mut msg = u32::to_be_bytes(state_msg.len() as u32).to_vec();
        msg.append(&mut state_msg);
        if client.stream.write_all(&msg).is_err() {
            break;
        }

        let mut buf_reader = BufReader::new(&mut client.stream);
        let mut command_msg = Vec::new();
        if buf_reader.read_until(0x04, &mut command_msg).is_err() {
            // TODO CATCH COMMANDS
            break;
        }
        command_msg.push(id + 48);
        client.command_sender.send(command_msg).unwrap();
    }
}
