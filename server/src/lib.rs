mod connection;
mod constants;
mod object;
pub mod space;

use crossbeam_channel::{unbounded, Receiver, Sender};
use space::Space;
use std::fs;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use connection::handle_connection;

pub fn run(path: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let space: Space = serde_json::from_slice(&fs::read(path)?)?;
    let space_counter = Arc::new(Mutex::new(space));

    let (command_sender, command_receiver) = unbounded();
    let update_counter = Arc::clone(&space_counter);
    thread::spawn(move || run_game(&update_counter, command_receiver));

    let (state_sender, state_receiver) = unbounded();
    let state_counter = Arc::clone(&space_counter);
    thread::spawn(move || run_state_send(&state_counter, state_sender));

    let listener = TcpListener::bind(SocketAddr::new(constants::IP, port))?;

    println!("listening started, ready to accept");

    let mut handles = Vec::with_capacity(5);
    for stream in listener.incoming().take(5).flatten() {
        let state_receiver = state_receiver.clone();
        let command_sender = command_sender.clone();
        handles.push(thread::spawn(move || {
            handle_connection(stream, state_receiver, command_sender);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

fn run_game(space_counter: &Arc<Mutex<Space>>, command_receiver: Receiver<Vec<u8>>) {
    loop {
        std::thread::sleep(Duration::from_secs_f64(constants::GAME_UPDATE_TICK_SECONDS));
        let mut space = space_counter.lock().unwrap();
        space.update(constants::GAME_UPDATE_TICK_SECONDS);
        for command in command_receiver.try_iter() {
            println!("{}", String::from_utf8(command).unwrap()); //TODO
        }
    }
}

fn run_state_send(space_counter: &Arc<Mutex<Space>>, state_sender: Sender<Vec<u8>>) {
    loop {
        let state;
        {
            let space = space_counter.lock().unwrap();
            state = space.get_state().unwrap();
        }
        state_sender.send(state).unwrap();
        std::thread::sleep(Duration::from_secs_f64(constants::GAME_STATE_TICK_SECONDS));
    }
}
