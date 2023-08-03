use std::sync::mpsc::{Receiver, Sender};

use glam::DVec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::logic::space::Space;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

pub enum DisplayType {
    Planet,
    Ship,
    Bullet,
}

pub struct DisplayInfo {
    pub display_type: DisplayType,
    pub id: Option<u8>,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

pub trait Drawable {
    fn get_display_info(&self) -> DisplayInfo;
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let display_info = self.get_display_info();
        match display_info.display_type {
            DisplayType::Planet => canvas.set_draw_color(Color::RED),
            DisplayType::Ship => canvas.set_draw_color(Color::BLUE),
            DisplayType::Bullet => canvas.set_draw_color(Color::WHITE),
        }
        draw_circle(
            canvas,
            Point::new(
                (display_info.x * WIDTH as f64) as i32,
                (display_info.y * HEIGHT as f64) as i32,
            ),
            (display_info.radius * WIDTH as f64) as i32,
        )
    }
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

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let mut direction_vec = DVec2::ZERO;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => break 'running,
                    Keycode::W => direction_vec.y = 1.,
                    Keycode::A => direction_vec.x = -1.,
                    Keycode::S => direction_vec.y = -1.,
                    Keycode::D => direction_vec.x = 1.,
                    _ => (),
                },
                _ => {}
            }
        }
        let direction = if direction_vec == DVec2::ZERO {
            0.
        } else {
            direction_vec.angle_between(DVec2::new(1., 0.))
        };

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let msg = state_recv.recv().unwrap();
        let space: Space = bincode::deserialize(&msg).unwrap();

        let mut msg = if direction_vec == DVec2::ZERO {
            vec![0]
        } else {
            vec![1]
        };
        msg.append(&mut f64::to_be_bytes(direction).to_vec());
        msg.append(&mut vec![0]);
        msg.append(&mut vec![0_u8; 8]);
        command_send.send(msg)?;

        space.draw_all(&mut canvas)?;
        canvas.present();
    }

    Ok(())
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
