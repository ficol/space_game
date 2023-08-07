use glam::f64::DVec2;
use serde::{Deserialize, Serialize};

pub trait Update {
    fn update(&mut self, time: f64, total_field: DVec2) {
        self.object_mut().update(time, total_field);
    }

    fn collision(&self, other: &impl Update) -> bool {
        self.object().collision(other.object())
    }

    fn get_field(&self, other: &impl Update) -> DVec2 {
        self.object().get_field(other.object())
    }

    fn object(&self) -> &Object;

    fn object_mut(&mut self) -> &mut Object;
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Object {
    pub location: DVec2,
    pub radius: f64,
    pub mass: f64,
    pub field: f64,
    pub velocity: DVec2,
    pub acceleration: DVec2,
}

impl Object {
    pub fn new(location: DVec2, radius: f64, mass: f64, field: f64, velocity: DVec2) -> Object {
        Object {
            location,
            radius,
            mass,
            field,
            velocity,
            acceleration: DVec2::ZERO,
        }
    }

    pub fn update(&mut self, time: f64, total_field: DVec2) {
        if self.mass != 0. {
            self.acceleration = total_field / self.mass;
        }
        self.velocity += self.acceleration * time;
        self.location += self.velocity * time;
        self.bounce();
    }

    fn collision(&self, other: &Self) -> bool {
        self.location.distance(other.location) <= self.radius + other.radius
    }

    fn get_field(&self, other: &Self) -> DVec2 {
        if self.location.distance(other.location) < 1e-3 {
            return DVec2::ZERO;
        }
        let effective_field_ratio =
            self.location.distance(other.location).min(other.radius) / other.radius;
        let value = (other.field * effective_field_ratio)
            / (self.location.distance(other.location).powf(2.));
        (other.location - self.location).normalize() * value
    }

    fn bounce(&mut self) {
        if self.location.x < 0. || self.location.x > 1. {
            self.velocity.x = -self.velocity.x;
            self.acceleration.x = 0.;
        }
        if self.location.y < 0. || self.location.y > 1. {
            self.velocity.y = -self.velocity.y;
            self.acceleration.y = 0.;
        }
        self.location = self.location.clamp(DVec2::ZERO, DVec2::new(1., 1.));
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
        let object1 = Object::new(DVec2::new(0., 0.), 0.1, 0., 0., DVec2::ZERO);
        let object2 = Object::new(DVec2::new(0.2, 0.2), 0.1, 0., 0., DVec2::ZERO);
        assert!(!object1.collision(&object2));
    }

    #[test]
    fn collision_of_two_objects() {
        let object1 = Object::new(DVec2::new(0., 0.), 1., 0., 0., DVec2::ZERO);
        let object2 = Object::new(DVec2::new(1., 1.), 1., 0., 0., DVec2::ZERO);
        assert!(object1.collision(&object2));
    }

    #[test]
    fn update_object_location_no_mass() {
        let mut object = Object::new(DVec2::ZERO, 1., 0., 0., DVec2::new(0.1, 0.));
        object.update(1., DVec2::new(0., 1.));
        assert!(object.velocity.is_finite());
        assert_vectors!(DVec2::new(0.1, 0.), object.location, 1e-6);
    }

    #[test]
    fn update_accelerated_object_location() {
        let mut object = Object::new(DVec2::new(0., 0.), 1., 1., 1., DVec2::ZERO);
        object.update(1., DVec2::new(0., 0.1)); //change location
        assert_vectors!(DVec2::new(0., 0.1), object.location, 1e-6);
    }

    #[test]
    fn calculate_field() {
        let object1 = Object::new(DVec2::new(0., 0.), 1., 1., 1., DVec2::ZERO);
        let object2 = Object::new(DVec2::new(0., 1.), 1., 1., 2., DVec2::ZERO);
        let field = object1.get_field(&object2);
        assert_vectors!(DVec2::new(0., 2.), field, 1e-6);
    }

    #[test]
    fn bounce_max() {
        let mut object = Object::new(DVec2::new(100., 80.), 1., 1., 1., DVec2::new(10., 10.));
        object.update(0., DVec2::ZERO);
        assert_vectors!(DVec2::new(1., 1.), object.location, 1e-6);
        assert_vectors!(DVec2::new(-10., -10.), object.velocity, 1e-6);
        assert_vectors!(DVec2::ZERO, object.acceleration, 1e-6);
    }

    #[test]
    fn bounce_min() {
        let mut object = Object::new(DVec2::new(-10., -20.), 1., 1., 1., DVec2::new(-10., -10.));
        object.update(0., DVec2::ZERO);
        assert_vectors!(DVec2::ZERO, object.location, 1e-6);
        assert_vectors!(DVec2::new(10., 10.), object.velocity, 1e-6);
        assert_vectors!(DVec2::ZERO, object.acceleration, 1e-6);
    }
}
