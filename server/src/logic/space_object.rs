use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::ui::display::{DisplayInfo, DisplayType, Drawable};

use super::object::{Object, Update};
use super::space::{BulletConfig, ShipConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    object: Object,
}

impl Update for Planet {
    fn object(&self) -> &Object {
        &self.object
    }

    fn object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
}

impl Drawable for Planet {
    fn get_display_info(&self) -> DisplayInfo {
        DisplayInfo {
            display_type: DisplayType::Planet,
            id: None,
            x: self.object.location.x,
            y: self.object.location.y,
            radius: self.object.radius,
        }
    }
}

impl Planet {
    #[cfg(test)]
    pub fn new(location: DVec2, mass: f64, field: f64, radius: f64, velocity: DVec2) -> Planet {
        Planet {
            object: Object::new(location, radius, mass, field, velocity),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    id: u8,
    object: Object,
    direction: Option<f64>,
    force: f64,
}

impl Update for Ship {
    fn update(&mut self, time: f64, total_field: DVec2) {
        if let Some(direction) = self.direction {
            self.object.update(
                time,
                total_field + self.force * DVec2::from_angle(direction),
            );
        } else {
            self.object.update(time, total_field);
        }
    }

    fn object(&self) -> &Object {
        &self.object
    }

    fn object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
}

impl Drawable for Ship {
    fn get_display_info(&self) -> DisplayInfo {
        DisplayInfo {
            display_type: DisplayType::Ship,
            id: Some(self.id),
            x: self.object.location.x,
            y: self.object.location.y,
            radius: self.object.radius,
        }
    }
}

impl Ship {
    pub fn new(id: u8, location: DVec2, ship_config: &ShipConfig) -> Self {
        Ship {
            id,
            object: Object::new(
                location,
                ship_config.radius,
                ship_config.mass,
                ship_config.field,
                DVec2::ZERO,
            ),
            direction: None,
            force: ship_config.force,
        }
    }

    pub fn shoot(&self, direction: f64, bullet_config: &BulletConfig) -> Bullet {
        Bullet {
            id: self.id,
            object: Object::new(
                self.object.location,
                bullet_config.radius,
                bullet_config.mass,
                bullet_config.field,
                DVec2::from_angle(direction) * bullet_config.speed,
            ),
        }
    }

    pub fn respawn(&mut self, new_location: DVec2) {
        self.object.location = new_location;
        self.object.velocity = DVec2::ZERO;
        self.object.acceleration = DVec2::ZERO;
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn change_direction(&mut self, direction: Option<f64>) {
        self.direction = direction;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bullet {
    id: u8,
    object: Object,
}

impl Update for Bullet {
    fn object(&self) -> &Object {
        &self.object
    }

    fn object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
}

impl Drawable for Bullet {
    fn get_display_info(&self) -> DisplayInfo {
        DisplayInfo {
            display_type: DisplayType::Bullet,
            id: Some(self.id),
            x: self.object.location.x,
            y: self.object.location.y,
            radius: self.object.radius,
        }
    }
}
