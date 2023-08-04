mod constants;
mod logic;
mod networking;
mod ui;

pub use logic::space::Space;
pub use ui::display::display_game;

use bus::Bus;
use std::fs;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use logic::update::{run_game, run_state_send};
use networking::connection::handle_listen;

pub fn run(path: &str, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    // create space
    let space: Space = serde_json::from_slice(&fs::read(path)?)?;
    let space_counter = Arc::new(Mutex::new(space));

    // update space logic thread
    let update_counter = Arc::clone(&space_counter);
    let (command_sender, command_receiver) = mpsc::channel();
    let update_handle = thread::spawn(move || run_game(&update_counter, command_receiver));

    // communication space thread
    let state_bus = Arc::new(Mutex::new(Bus::new(constants::MAX_PLAYERS.into())));
    let space_counter = Arc::clone(&space_counter);
    let broadcast = Arc::clone(&state_bus);
    let state_handle = thread::spawn(move || run_state_send(&space_counter, &broadcast));

    // listen for new connections
    handle_listen(addr, command_sender, &state_bus)?;

    update_handle.join().unwrap();
    state_handle.join().unwrap();

    Ok(())
}
