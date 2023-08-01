use crossbeam_channel::Receiver;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn print_game(state_recv: Receiver<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 1000, 1000)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        if let Ok(msg) = state_recv.recv() {
            println!("{}", String::from_utf8(msg).unwrap()); //TODO
        } else {
            break;
        }

        canvas.set_draw_color(Color::WHITE);
        draw_dot(&mut canvas, 10_i32, 10_i32, 10)?;
        canvas.present();
    }

    Ok(())
}

fn draw_dot(canvas: &mut Canvas<Window>, x: i32, y: i32, radius: u32) -> Result<(), String> {
    canvas.fill_rect(Rect::new(x, y, radius, radius))?;

    Ok(())
}
