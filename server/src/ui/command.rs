use glam::DVec2;

use crate::logic::space::Space;

pub fn run_command(space: &mut Space, command: Vec<u8>) {
    // TODO HANDLE COMMANDS
    match command[..] {
        [3] => space.add_ship(1, DVec2::ZERO),
        [4] => space.remove_ship(1),
        [5] => space.shoot(1, 0.0),
        _ => space.move_ship(1, None),
    }
}
