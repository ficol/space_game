mod connection;
mod constants;
mod object;
pub mod space;

use bus::Bus;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use std::fs;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use connection::handle_connection;
use space::Space;

pub fn run(path: &str, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let space: Space = serde_json::from_slice(&fs::read(path)?)?;
    let space_counter = Arc::new(Mutex::new(space));

    let update_counter = Arc::clone(&space_counter);
    let (command_sender, command_receiver) = mpsc::channel();
    thread::spawn(move || run_game(&update_counter, command_receiver));

    let state_bus = Arc::new(Mutex::new(Bus::new(constants::MAX_PLAYERS)));

    let space_counter = Arc::clone(&space_counter);
    let broadcast = Arc::clone(&state_bus);
    thread::spawn(move || run_state_send(&space_counter, &broadcast));

    let listener = TcpListener::bind(addr)?;

    println!("listening started, ready to accept");

    let mut handles: Vec<JoinHandle<_>> = Vec::with_capacity(constants::MAX_PLAYERS);
    for mut stream in listener.incoming().flatten() {
        handles.retain(|handle| !handle.is_finished());
        if handles.len() >= constants::MAX_PLAYERS {
            let _ = stream.write_all(b"too many players%");
            let _ = stream.shutdown(std::net::Shutdown::Both);
            continue;
        }

        let command_sender = command_sender.clone();
        let state_receiver = state_bus.lock().unwrap().add_rx();
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

fn run_state_send(space_counter: &Arc<Mutex<Space>>, state_sender: &Arc<Mutex<Bus<Vec<u8>>>>) {
    loop {
        let state;
        {
            let space = space_counter.lock().unwrap();
            state = space.get_state().unwrap();
        }
        state_sender.lock().unwrap().broadcast(state);
        std::thread::sleep(Duration::from_secs_f64(constants::GAME_STATE_TICK_SECONDS));
    }
}
