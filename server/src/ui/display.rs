use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum DisplayType {
    Planet,
    Ship,
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
    fn draw(&self, canvas: &mut Canvas<Window>, width: u32, height: u32) -> Result<(), String> {
        let display_info = self.get_display_info();
        match display_info.display_type {
            DisplayType::Planet => canvas.set_draw_color(Color::RED),
            DisplayType::Ship => canvas.set_draw_color(Color::WHITE),
        }
        draw_circle(
            canvas,
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
