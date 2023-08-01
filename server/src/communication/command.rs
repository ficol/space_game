use crate::game::{
    object::{Object, ShipConfig},
    space::Space,
};

pub fn run_command(space: &mut Space, command: Vec<u8>) {
    match command[..] {
        [2] => space.add_planet(Object::default()),
        [3] => space.add_ship(1, Object::default(), ShipConfig::default()),
        [4] => space.remove_ship(1),
        [5] => space.shoot(1, 0.0),
        _ => space.move_ship(1, None),
    }
}
