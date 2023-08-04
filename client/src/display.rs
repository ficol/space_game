use std::sync::mpsc::{Receiver, Sender};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use server::Space;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

pub fn display_game(
    state_recv: Receiver<Vec<u8>>,
    command_send: Sender<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Space Game", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let (mut is_move, mut direction) = (false, 0.);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    (is_move, direction) = match keycode {
                        Keycode::Escape => break 'running,
                        Keycode::W => (true, -std::f64::consts::PI / 2.),
                        Keycode::A => (true, std::f64::consts::PI),
                        Keycode::S => (true, std::f64::consts::PI / 2.),
                        Keycode::D => (true, 0.),
                        _ => (false, 0.),
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let msg = state_recv.recv()?;
        let space: Space = bincode::deserialize(&msg)?;
        space.draw_all(&mut canvas, WIDTH, HEIGHT)?;
        canvas.present();

        // send move command
        let mut msg = vec![is_move.into()];
        msg.append(&mut f64::to_be_bytes(direction).to_vec());
        command_send.send(msg)?;
    }

    Ok(())
}
