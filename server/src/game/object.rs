use glam::f64::DVec2;
use serde::{Deserialize, Serialize};

pub trait Update {
    fn update(&mut self, time: f64, total_field: DVec2, size: DVec2) {
        self.object_mut().update(time, total_field, size);
    }

    fn collision(&self, other: &impl Update) -> bool {
        self.object().collision(other.object())
    }

    fn get_field(&self, other: &impl Update) -> DVec2 {
        self.object().get_field(other.object())
    }

    fn fit_in(&self, size: DVec2) -> bool {
        self.object().fit_in(size)
    }

    fn get_params(&self) -> (f64, f64, f64) {
        self.object().get_params()
    }

    fn object(&self) -> &Object;

    fn object_mut(&mut self) -> &mut Object;
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Object {
    location: DVec2,
    radius: f64,
    mass: f64,
    velocity: DVec2,
    acceleration: DVec2,
}

#[allow(dead_code)]
impl Object {
    pub fn new(location: DVec2, radius: f64, mass: f64, velocity: DVec2) -> Object {
        Object {
            location,
            radius,
            mass,
            velocity,
            acceleration: DVec2::ZERO,
        }
    }

    fn update(&mut self, time: f64, total_field: DVec2, size: DVec2) {
        self.location += self.velocity * time;
        self.velocity += self.acceleration * time;
        if self.mass != 0. {
            self.acceleration = total_field / self.mass;
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

    fn bound(&mut self, size: DVec2) {
        self.location = self.location.clamp(DVec2::ZERO, size);
    }

    fn fit_in(&self, size: DVec2) -> bool {
        self.location.cmple(size).all()
    }

    fn get_params(&self) -> (f64, f64, f64) {
        (self.location.x, self.location.y, self.radius)
    }
}

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

impl Planet {
    pub fn new(object: Object) -> Self {
        Planet { object }
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
    object: Object,
    direction: Option<f64>,
    ship_config: ShipConfig,
    score: Score,
}

impl Update for Ship {
    fn object(&self) -> &Object {
        &self.object
    }

    fn object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
}

impl Ship {
    pub fn new(id: u8, object: Object, ship_config: ShipConfig) -> Self {
        Ship {
            id,
            object,
            direction: None,
            ship_config,
            score: Score::default(),
        }
    }

    pub fn shoot(&self, direction: f64) -> Bullet {
        Bullet::new(
            self.id,
            Object {
                radius: self.ship_config.bullet_radius,
                mass: self.ship_config.bullet_mass,
                velocity: DVec2::from_angle(direction) * self.ship_config.bullet_speed,
                acceleration: DVec2::ZERO,
                ..self.object
            },
        )
    }

    pub fn respawn(&mut self, new_location: DVec2) {
        self.score.deaths += 1;
        self.object.location = new_location;
    }

    pub fn kill(&mut self) {
        self.score.kills += 1;
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

impl Bullet {
    pub fn new(id: u8, object: Object) -> Self {
        Bullet { id, object }
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
        let object1 = Object::new(DVec2::new(0., 0.), 1., 0., DVec2::ZERO);
        let planet1 = Planet::new(object1);
        let object2 = Object::new(DVec2::new(2., 2.), 1., 0., DVec2::ZERO);
        let bullet2 = Bullet::new(1, object2);
        assert!(!planet1.collision(&bullet2));
    }

    #[test]
    fn collision_of_two_objects() {
        let object1 = Object::new(DVec2::new(0., 0.), 1., 0., DVec2::ZERO);
        let bullet1 = Bullet::new(4, object1);
        let object2 = Object::new(DVec2::new(1., 1.), 1., 0., DVec2::ZERO);
        let ship2 = Ship::new(6, object2, ShipConfig::default());
        assert!(bullet1.collision(&ship2));
    }

    #[test]
    fn update_object_location_no_mass() {
        let object = Object::new(DVec2::new(0., 0.), 1., 0., DVec2::new(1., 0.));
        let mut object = Planet::new(object);
        object.update(1., DVec2::new(0., 1.), DVec2::MAX);
        assert!(object.object.velocity.is_finite());
        assert_vectors!(DVec2::new(1., 0.), object.object.location, 1e-6);
    }

    #[test]
    fn update_accelerated_object_location() {
        let object = Object::new(DVec2::new(0., 0.), 1., 1., DVec2::ZERO);
        let mut object = Bullet::new(4, object);
        object.update(0., DVec2::new(0., 1.), DVec2::MAX); //set acceleration
        object.update(1., DVec2::new(0., 1.), DVec2::MAX); //gain velocity
        object.update(1., DVec2::new(0., 1.), DVec2::MAX); //change location
        assert_vectors!(DVec2::new(0., 1.), object.object.location, 1e-6);
    }

    #[test]
    fn calculate_field() {
        let object1 = Object::new(DVec2::new(0., 0.), 1., 1., DVec2::ZERO);
        let object1 = Ship::new(12, object1, ShipConfig::default());
        let object2 = Object::new(DVec2::new(0., 1.), 1., 2., DVec2::ZERO);
        let object2 = Planet::new(object2);
        let field = object1.get_field(&object2);
        assert_vectors!(DVec2::new(0., 2.), field, 1e-6);
    }

    #[test]
    fn bound_max() {
        let object = Object::new(DVec2::new(100., 80.), 1., 1., DVec2::ZERO);
        let mut object = Planet::new(object);
        object.update(0., DVec2::ZERO, DVec2::new(50., 40.));
        assert_vectors!(DVec2::new(50., 40.), object.object.location, 1e-6);
    }

    #[test]
    fn bound_min() {
        let object = Object::new(DVec2::new(-10., -20.), 1., 1., DVec2::ZERO);
        let mut object = Bullet::new(2, object);
        object.update(0., DVec2::ZERO, DVec2::new(50., 40.));
        assert_vectors!(DVec2::ZERO, object.object.location, 1e-6);
    }
}
