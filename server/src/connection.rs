use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::space::Space;

pub fn run_connection(space_mutex: &Arc<Mutex<Space>>, _port: u32) {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        let connection_mutex = Arc::clone(space_mutex);
        thread::spawn(move || handle_connection(stream.unwrap(), &connection_mutex).expect("Aaa"));
    }
}

fn handle_connection(
    mut stream: TcpStream,
    space_mutex: &Arc<Mutex<Space>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 60));
        let mut state;
        {
            let space = space_mutex.lock().unwrap();
            state = space.get_state()?;
        }
        state.push(b'%');
        stream.write_all(&state)?;
    }
}
