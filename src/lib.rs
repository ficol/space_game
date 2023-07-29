mod display;
mod object;
mod space;

use display::print_game;
use space::Space;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, time::SystemTime};

pub fn run(path: &str, _port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let space: Space = serde_json::from_slice(&fs::read(path)?)?;
    let space_counter = Arc::new(Mutex::new(space));

    let update_counter = Arc::clone(&space_counter);
    thread::spawn(move || {
        run_game(&update_counter);
    });

    let print_counter = Arc::clone(&space_counter);
    print_game(&print_counter)?;

    Ok(())
}

fn run_game(space_mutex: &Arc<Mutex<Space>>) -> ! {
    loop {
        let start = SystemTime::now();
        std::thread::sleep(Duration::new(0, 1000));
        {
            if let Ok(duration) = start.elapsed() {
                let mut space = space_mutex.lock().unwrap();
                space.update(duration.as_secs_f64());
            }
        }
    }
}
