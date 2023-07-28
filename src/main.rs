mod object;
mod space;

use glam::DVec2;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use std::{thread, time::SystemTime};

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
    let ship_config2 = ShipConfig::new(1.0, 1.0, 1.0, 10.0);
    let _ = space.add_ship(DVec2::new(20.0, 20.0), 1.0, 100.0, ship_config2);

    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        run_space(&mut space, tx);
    });
    let handle2 = thread::spawn(move || {
        print_space_state(rx);
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}

fn run_space(space: &mut space::Space, tx: Sender<String>) -> ! {
    let now = SystemTime::now();
    loop {
        std::thread::sleep(Duration::new(1, 0));
        if let Ok(elapsed) = now.elapsed() {
            space.update(elapsed.as_secs_f64());
        }
        tx.send(serde_json::to_string_pretty(&space).unwrap())
            .expect("send failed");
    }
}

fn print_space_state(rx: Receiver<String>) -> ! {
    loop {
        println!("{}", rx.recv().unwrap());
    }
}
