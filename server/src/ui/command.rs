use crate::logic::space;

pub fn run_command(space: &mut space::Space, command: Vec<u8>) {
    match command[0] {
        0 => space.add_ship(command[1], space::gen_new_location()),
        1 => space.remove_ship(command[1]),
        2 => space.move_ship(
            command[1],
            if command[2] == 0 {
                None
            } else {
                Some(f64::from_be_bytes(command[3..].try_into().unwrap()))
            },
        ),
        _ => space.move_ship(1, None),
    }
}
