use bus::{Bus, BusReader};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use super::command;
use crate::constants;
use crate::game::space::Space;

struct Client {
    stream: TcpStream,
    command_sender: Sender<Vec<u8>>,
    state_receiver: BusReader<Vec<u8>>,
}

pub fn run_game(space_counter: &Arc<Mutex<Space>>, command_receiver: Receiver<Vec<u8>>) {
    loop {
        std::thread::sleep(Duration::from_secs_f64(constants::GAME_UPDATE_TICK_SECONDS));
        let mut space = space_counter.lock().unwrap();
        for command in command_receiver.try_iter() {
            command::run_command(&mut space, command);
        }
        space.update(constants::GAME_UPDATE_TICK_SECONDS);
    }
}

pub fn run_state_send(space_counter: &Arc<Mutex<Space>>, state_sender: &Arc<Mutex<Bus<Vec<u8>>>>) {
    loop {
        let state;
        {
            let space: std::sync::MutexGuard<'_, Space> = space_counter.lock().unwrap();
            state = space.get_state();
        }
        state_sender.lock().unwrap().broadcast(state);
        std::thread::sleep(Duration::from_secs_f64(constants::GAME_STATE_TICK_SECONDS));
    }
}

pub fn handle_listen(
    addr: &str,
    command_sender: Sender<Vec<u8>>,
    state_bus: &Arc<Mutex<Bus<Vec<u8>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr)?;

    let mut connections: HashMap<u8, JoinHandle<()>> = HashMap::new();
    for mut stream in listener.incoming().flatten() {
        if let Some(id) = init_connection(&mut connections, &mut stream) {
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
    stream: &mut TcpStream,
) -> Option<u8> {
    connections.retain(|_, handle| !handle.is_finished());
    if connections.len() >= constants::MAX_PLAYERS as usize {
        stream.write_all(constants::MAX_PLAYERS_MSG).unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        return None;
    }
    (1..constants::MAX_PLAYERS + 1).find(|&i| !connections.contains_key(&i))
}

fn handle_connection(id: u8, mut client: Client) {
    loop {
        let mut state_msg = client.state_receiver.recv().unwrap();
        while let Ok(msg) = client.state_receiver.try_recv() {
            state_msg = msg;
        }
        state_msg.push(constants::MSG_END);
        if client.stream.write_all(&state_msg).is_err() {
            break;
        }
        let mut buf_reader = BufReader::new(&mut client.stream);
        let mut command_msg = Vec::new();
        if buf_reader
            .read_until(constants::MSG_END, &mut command_msg)
            .is_err()
        {
            break;
        }
        command_msg.push(id + 48);
        client.command_sender.send(command_msg).unwrap();
    }
    client.command_sender.send(vec![b'a', 48 + id]).unwrap()
}
