mod display;
mod object;
mod space;

use clap::Parser;
use display::print_game;
use object::Updatable;
use space::Space;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, time::SystemTime};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Space Game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to json file with map configuration
    #[arg(long, default_value_t = String::from("maps/example.json"))]
    path: String,

    /// Port to bind server
    #[arg(long, default_value_t = 8888)]
    port: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let space: Space = serde_json::from_slice(&fs::read(args.path)?)?;
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
    let now = SystemTime::now();
    let mut prev = now.elapsed().unwrap().as_secs_f64();
    loop {
        std::thread::sleep(Duration::new(0, 1000));
        {
            let mut space = space_mutex.lock().unwrap();
            if let Ok(elapsed) = now.elapsed() {
                let curr = elapsed.as_secs_f64();
                space.update(curr - prev);
                prev = curr;
            }
        }
    }
}
