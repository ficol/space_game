mod object;
mod space;
mod space_builder;

use glam::DVec2;

use crate::{object::Updatable, space::ShipConfig};

fn main() {
    let mut space = space::Space::new(DVec2::new(100.0, 100.0));
    let ship_config = ShipConfig::new(1.0, 1.0, 1.0, 10.0);
    space.add_planet(DVec2::new(10.0, 10.0), 1.0, 100.0, DVec2::ZERO);
    let id1 = space.add_ship(DVec2::new(20.0, 20.0), 1.0, 100.0, ship_config);
    space.move_ship(id1.unwrap(), None);
    space.shoot(id1.unwrap(), 0.0);
    space.update(10.0);
    space.remove_ship(id1.unwrap());
    let state = space.get_state().unwrap();
    println!("{}", state.len());
}
