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

pub trait Drawer {
    fn draw(&mut self, display_info: DisplayInfo, width: u32, height: u32) -> Result<(), String>;
}
