use glam::f64::DVec2;
use serde::{Deserialize, Serialize};

pub trait Object {
    fn update(&mut self, time: f64, total_field: &DVec2, size: &DVec2) {
        self.properties_mut().update(time, total_field, size);
    }

    fn collision(&self, other: &impl Object) -> bool {
        self.properties().collision(other.properties())
    }

    fn get_field(&self, other: &impl Object) -> DVec2 {
        self.properties().get_field(other.properties())
    }

    fn properties(&self) -> &Properties;

    fn properties_mut(&mut self) -> &mut Properties;
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Properties {
    location: DVec2,
    radius: f64,
    mass: f64,
    velocity: DVec2,
    acceleration: DVec2,
}

#[allow(dead_code)]
impl Properties {
    pub fn new(location: DVec2, radius: f64, mass: f64, velocity: DVec2) -> Properties {
        Properties {
            location,
            radius,
            mass,
            velocity,
            acceleration: DVec2::ZERO,
        }
    }

    fn update(&mut self, time: f64, total_field: &DVec2, size: &DVec2) {
        self.location += self.velocity * time;
        self.velocity += self.acceleration * time;
        if self.mass != 0. {
            self.acceleration = *total_field / self.mass;
        }
        self.bound(size);
    }

    fn collision(&self, other: &Self) -> bool {
        self.location.distance(other.location) <= self.radius + other.radius
    }

    fn get_field(&self, other: &Self) -> DVec2 {
        if self.location.distance(other.location) < other.radius {
            return DVec2::ZERO;
        }
        let value = other.mass / (self.location.distance(other.location).powf(2.));
        (other.location - self.location).normalize() * value
    }

    fn bound(&mut self, size: &DVec2) {
        self.location = self.location.clamp(DVec2::ZERO, *size);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    properties: Properties,
}

impl Object for Planet {
    fn properties(&self) -> &Properties {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

impl Planet {
    pub fn new(properties: Properties) -> Self {
        Planet { properties }
    }

    // tmp
    pub fn get_location(&self) -> DVec2 {
        self.properties.location
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShipConfig {
    bullet_speed: f64,
    bullet_radius: f64,
    bullet_mass: f64,
    force: f64,
}

#[allow(dead_code)]
impl ShipConfig {
    pub fn new(bullet_speed: f64, bullet_radius: f64, bullet_mass: f64, force: f64) -> ShipConfig {
        ShipConfig {
            bullet_speed,
            bullet_radius,
            bullet_mass,
            force,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Score {
    kills: u32,
    deaths: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    id: u8,
    properties: Properties,
    direction: Option<f64>,
    ship_config: ShipConfig,
    score: Score,
}

impl Object for Ship {
    fn properties(&self) -> &Properties {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

impl Ship {
    pub fn new(id: u8, properties: Properties, ship_config: ShipConfig) -> Self {
        Ship {
            id,
            properties,
            direction: None,
            ship_config,
            score: Score::default(),
        }
    }

    pub fn shoot(&self, direction: f64) -> Bullet {
        Bullet {
            id: self.id,
            properties: Properties {
                radius: self.ship_config.bullet_radius,
                mass: self.ship_config.bullet_mass,
                velocity: DVec2::from_angle(direction) * self.ship_config.bullet_speed,
                acceleration: DVec2::ZERO,
                ..self.properties
            },
        }
    }

    pub fn respawn(&mut self, new_location: DVec2) {
        self.score.deaths += 1;
        self.properties.location = new_location;
    }

    pub fn kill(&mut self) {
        self.score.kills += 1;
    }

    // tmp
    pub fn get_location(&self) -> DVec2 {
        self.properties.location
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
    properties: Properties,
}

impl Object for Bullet {
    fn properties(&self) -> &Properties {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

#[allow(dead_code)]
impl Bullet {
    pub fn new(id: u8, properties: Properties) -> Self {
        Bullet { id, properties }
    }

    // tmp
    pub fn get_location(&self) -> DVec2 {
        self.properties.location
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_vectors {
        ($x:expr, $y:expr, $d:expr) => {
            if $x.distance($y) > $d {
                panic!();
            }
        };
    }

    #[test]
    fn no_collision_of_two_objects() {
        let properties1 = Properties::new(DVec2::new(0., 0.), 1., 0., DVec2::ZERO);
        let object1 = Planet::new(properties1);
        let properties2 = Properties::new(DVec2::new(2., 2.), 1., 0., DVec2::ZERO);
        let object2 = Bullet::new(1, properties2);
        assert!(!object1.collision(&object2));
    }

    #[test]
    fn collision_of_two_objects() {
        let properties1 = Properties::new(DVec2::new(0., 0.), 1., 0., DVec2::ZERO);
        let object1 = Bullet::new(4, properties1);
        let properties2 = Properties::new(DVec2::new(1., 1.), 1., 0., DVec2::ZERO);
        let object2 = Ship::new(6, properties2, ShipConfig::default());
        assert!(object1.collision(&object2));
    }

    #[test]
    fn update_object_location_no_mass() {
        let properties = Properties::new(DVec2::new(0., 0.), 1., 0., DVec2::new(1., 0.));
        let mut object = Planet::new(properties);
        object.update(1., &DVec2::new(0., 1.), &DVec2::MAX);
        assert!(object.properties.velocity.is_finite());
        assert_vectors!(DVec2::new(1., 0.), object.properties.location, 1e-6);
    }

    #[test]
    fn update_accelerated_object_location() {
        let properties = Properties::new(DVec2::new(0., 0.), 1., 1., DVec2::ZERO);
        let mut object = Bullet::new(4, properties);
        object.update(0., &DVec2::new(0., 1.), &DVec2::MAX); //set acceleration
        object.update(1., &DVec2::new(0., 1.), &DVec2::MAX); //gain velocity
        object.update(1., &DVec2::new(0., 1.), &DVec2::MAX); //change location
        assert_vectors!(DVec2::new(0., 1.), object.properties.location, 1e-6);
    }

    #[test]
    fn calculate_field() {
        let properties1 = Properties::new(DVec2::new(0., 0.), 1., 1., DVec2::ZERO);
        let object1 = Ship::new(12, properties1, ShipConfig::default());
        let properties2 = Properties::new(DVec2::new(0., 1.), 1., 2., DVec2::ZERO);
        let object2 = Planet::new(properties2);
        let field = object1.get_field(&object2);
        assert_vectors!(DVec2::new(0., 2.), field, 1e-6);
    }

    #[test]
    fn bound_max() {
        let properties = Properties::new(DVec2::new(100., 80.), 1., 1., DVec2::ZERO);
        let mut object = Planet::new(properties);
        object.update(0., &DVec2::ZERO, &DVec2::new(50., 40.));
        assert_vectors!(DVec2::new(50., 40.), object.properties.location, 1e-6);
    }

    #[test]
    fn bound_min() {
        let properties = Properties::new(DVec2::new(-10., -20.), 1., 1., DVec2::ZERO);
        let mut object = Bullet::new(2, properties);
        object.update(0., &DVec2::ZERO, &DVec2::new(50., 40.));
        assert_vectors!(DVec2::ZERO, object.properties.location, 1e-6);
    }
}
