use bus::Bus;
use std::sync::{mpsc::Receiver, Arc, Mutex};
use std::time::Duration;

use crate::constants;
use crate::ui::command;

use super::space::Space;

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
            state = space.get_state_binary();
        }
        state_sender.lock().unwrap().broadcast(state);
        std::thread::sleep(Duration::from_secs_f64(constants::GAME_STATE_TICK_SECONDS));
    }
}
