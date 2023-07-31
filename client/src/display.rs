// use std::time::Duration;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::video::Window;

// pub fn print_game(json: &str) -> Result<(), String> {
// let sdl_context = sdl2::init()?;
// let video_subsystem = sdl_context.video()?;

// let window = video_subsystem
//     .window("rust-sdl2 demo: Video", 1000, 1000)
//     .position_centered()
//     .opengl()
//     .build()
//     .map_err(|e| e.to_string())?;

// let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

// let mut event_pump = sdl_context.event_pump()?;

// 'running: loop {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit { .. }
//             | Event::KeyDown {
//                 keycode: Some(Keycode::Escape),
//                 ..
//             } => break 'running,
//             _ => {}
//         }
//     }

//     canvas.set_draw_color(Color::BLACK);
//     canvas.clear();

//     let locations: Vec<(f64, f64)>;
//     {
//         let space = space_mutex.lock().unwrap();
//         locations = space.get_objects_location();
//     }

//     canvas.set_draw_color(Color::WHITE);
//     for location in locations {
//         draw_dot(&mut canvas, location.0 as i32, location.1 as i32)?;
//     }
//     canvas.present();
//     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
// }

//     Ok(())
// }

// fn draw_dot(canvas: &mut Canvas<Window>, x: i32, y: i32) -> Result<(), String> {
//     canvas.fill_rect(Rect::new(x, y, 10, 10))?;

//     Ok(())
// }
