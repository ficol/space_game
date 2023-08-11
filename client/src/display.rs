use std::sync::mpsc::{Receiver, Sender};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use server::{DisplayInfo, DisplayType, Drawer, Space};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

struct CanvasDrawer {
    pub canvas: Canvas<Window>,
}

impl Drawer for CanvasDrawer {
    fn draw(&mut self, display_info: DisplayInfo, width: u32, height: u32) -> Result<(), String> {
        match display_info.display_type {
            DisplayType::Planet => self.canvas.set_draw_color(Color::RED),
            DisplayType::Ship => self.canvas.set_draw_color(Color::WHITE),
        }
        draw_circle(
            &mut self.canvas,
            Point::new(
                (display_info.x * width as f64) as i32,
                (display_info.y * height as f64) as i32,
            ),
            (display_info.radius * f64::from(width)) as i32,
        )
    }
}

fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32) -> Result<(), String> {
    let mut x = radius;
    let mut y = 0;

    let mut re = x * x + y * y - radius * radius;
    while x >= y {
        canvas.draw_point(Point::new(center.x() + x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() + y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() + x))?;

        canvas.draw_point(Point::new(center.x() - x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() - y, center.y() - x))?;

        canvas.draw_point(Point::new(center.x() + x, center.y() - y))?;
        canvas.draw_point(Point::new(center.x() + y, center.y() - x))?;

        if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0 {
            re += 1 - 2 * x;
            x -= 1;
        }
        re += 2 * y + 1;
        y += 1;
    }

    Ok(())
}

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

    let mut canvas_drawer = CanvasDrawer {
        canvas: window.into_canvas().build().map_err(|e| e.to_string())?,
    };

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

        canvas_drawer.canvas.set_draw_color(Color::BLACK);
        canvas_drawer.canvas.clear();

        let msg = state_recv.recv()?;
        let space: Space = bincode::deserialize(&msg)?;
        space.draw_all(&mut canvas_drawer, WIDTH, HEIGHT)?;
        canvas_drawer.canvas.present();

        // send move command
        let mut msg = vec![is_move.into()];
        msg.append(&mut f64::to_be_bytes(direction).to_vec());
        command_send.send(msg)?;
    }

    Ok(())
}
