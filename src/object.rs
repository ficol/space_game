use glam::f64::DVec2;
use serde::Serialize;
pub trait Updatable {
    fn update(&mut self, time: f64);
}

#[derive(Debug, Serialize)]
pub struct Object {
    location: DVec2,
    radius: f64,
    mass: f64,
    velocity: DVec2,
    acceleration: DVec2,
}

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

    pub fn spawn_from(&self, radius: f64, mass: f64, velocity: DVec2) -> Object {
        Object {
            location: self.location,
            radius,
            mass,
            velocity,
            acceleration: DVec2::ZERO,
        }
    }

    pub fn change_acceleration(&mut self, total_field: &DVec2) {
        if self.mass != 0.0 {
            self.acceleration = *total_field / self.mass;
        }
    }

    pub fn collision(&self, other: &Self) -> bool {
        self.location.distance(other.location) <= self.radius + other.radius
    }

    pub fn get_field(&self, other: &Self) -> DVec2 {
        if self.location == other.location {
            return DVec2::ZERO;
        }
        let value = self.mass / (self.location.distance(other.location).powf(2.0));
        (self.location - other.location).normalize() * value
    }

    pub fn bound(&mut self, size: &DVec2) {
        self.location = self.location.clamp(DVec2::MIN, *size);
    }
}

impl Updatable for Object {
    fn update(&mut self, time: f64) {
        self.velocity += self.acceleration * time;
        self.location += self.velocity * time;
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
        let object1 = Object::new(DVec2::new(0.0, 0.0), 1.0, 0.0, DVec2::ZERO);
        let object2 = Object::new(DVec2::new(2.0, 2.0), 1.0, 0.0, DVec2::ZERO);
        assert!(!object1.collision(&object2));
    }

    #[test]
    fn collision_of_two_objects() {
        let object1 = Object::new(DVec2::new(0.0, 0.0), 1.0, 0.0, DVec2::ZERO);
        let object2 = Object::new(DVec2::new(1.0, 1.0), 1.0, 0.0, DVec2::ZERO);
        assert!(object1.collision(&object2));
    }

    #[test]
    fn update_object_location_no_mass() {
        let mut object = Object::new(DVec2::new(0.0, 0.0), 1.0, 0.0, DVec2::new(1.0, 0.0));
        object.change_acceleration(&DVec2::new(0.0, 1.0));
        object.update(1.0);
        assert!(object.velocity.is_finite());
        assert_vectors!(DVec2::new(1.0, 0.0), object.location, 1e-6);
    }

    #[test]
    fn update_accelerated_object_location() {
        let mut object = Object::new(DVec2::new(0.0, 0.0), 1.0, 1.0, DVec2::new(1.0, 0.0));
        object.change_acceleration(&DVec2::new(0.0, 1.0));
        object.update(1.0);
        assert_vectors!(DVec2::new(1.0, 1.0), object.location, 1e-6);
    }

    #[test]
    fn calculate_field() {
        let object = Object::new(DVec2::new(0.0, 0.0), 1.0, 1.0, DVec2::ZERO);
        let object2 = Object::new(DVec2::new(0.0, 1.0), 1.0, 1.0, DVec2::ZERO);
        let field = object.get_field(&object2);
        assert_vectors!(DVec2::new(0.0, -1.0), field, 1e-6);
    }

    #[test]
    fn bound() {
        let mut object = Object::new(DVec2::new(100.0, 80.0), 1.0, 1.0, DVec2::ZERO);
        let size = DVec2::new(50.0, 40.0);
        object.bound(&size);
        assert_vectors!(size, object.location, 1e-6);
    }
}
