use glam::DVec2;
use serde::{Deserialize, Serialize};

use crate::object::Object;
use crate::object::Updatable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    size: DVec2,
    planets: Vec<Object>,
    ships: Vec<Ship>,
    bullets: Vec<Bullet>,
}

impl Space {
    pub fn new(size: DVec2) -> Space {
        Space {
            size,
            planets: vec![],
            ships: vec![],
            bullets: vec![],
        }
    }

    pub fn add_planet(&mut self, location: DVec2, radius: f64, mass: f64, velocity: DVec2) -> bool {
        self.planets
            .push(Object::new(location, radius, mass, velocity));
        true
    }

    pub fn add_ship(
        &mut self,
        location: DVec2,
        radius: f64,
        mass: f64,
        ship_config: ShipConfig,
    ) -> Option<u8> {
        let new_id = self.ships.iter().map(|x| x.id).max().unwrap_or(0) + 1;
        self.ships.push(Ship {
            id: new_id,
            object: Object::new(location, radius, mass, DVec2::ZERO),
            ship_config,
            score: Score {
                kills: 0,
                deaths: 0,
            },
            direction: None,
        });
        Some(new_id)
    }

    pub fn remove_ship(&mut self, id: u8) -> bool {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            self.ships.remove(index);
            return true;
        }
        false
    }

    pub fn move_ship(&mut self, id: u8, direction: Option<f64>) -> bool {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            self.ships[index].direction = direction;
            return true;
        }
        false
    }

    pub fn shoot(&mut self, id: u8, direction: f64) -> bool {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            let config = &self.ships[index].ship_config;
            let bullet_velocity = DVec2::from_angle(direction) * config.bullet_speed;
            self.bullets.push(Bullet {
                id,
                object: self.ships[index].object.spawn_from(
                    config.bullet_radius,
                    config.bullet_mass,
                    bullet_velocity,
                ),
            });
            return true;
        }
        false
    }

    fn get_ship_index(&self, id: u8) -> Option<usize> {
        self.ships.iter().position(|x| x.id == id)
    }
}

impl Updatable for Space {
    fn update(&mut self, time: f64) {
        let total_planet_fields: Vec<DVec2> = self
            .planets
            .iter()
            .map(|planet| {
                self.planets
                    .iter()
                    .map(|other| planet.get_field(other))
                    .sum()
            })
            .collect();
        for (i, planet) in self.planets.iter_mut().enumerate() {
            planet.update(time);
            planet.bound(&self.size);
            planet.change_acceleration(&total_planet_fields[i]); //TODO
        }
        for bullet in self.bullets.iter_mut() {
            bullet.object.update(time);
            bullet.object.bound(&self.size);
            let total_field = self
                .planets
                .iter()
                .map(|other| bullet.object.get_field(other))
                .sum();
            bullet.object.change_acceleration(&total_field);
        }
        for ship in self.ships.iter_mut() {
            ship.object.update(time);
            ship.object.bound(&self.size);
            let total_field = self
                .planets
                .iter()
                .map(|other| ship.object.get_field(other))
                .sum();
            ship.object.change_acceleration(&total_field);
            if self
                .planets
                .iter()
                .any(|planet| ship.object.collision(planet))
            {
                //TODO kill by nobody
            }
            if self
                .planets
                .iter()
                .any(|bullet| ship.object.collision(bullet))
            {
                //TODO kill by player
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipConfig {
    bullet_speed: f64,
    bullet_radius: f64,
    bullet_mass: f64,
    force: f64,
}

impl ShipConfig {
    pub fn new(bullet_speed: f64, bullet_radius: f64, bullet_mass: f64, force: f64) -> Self {
        ShipConfig {
            bullet_speed,
            bullet_radius,
            bullet_mass,
            force,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Ship {
    id: u8,
    object: Object,
    direction: Option<f64>,
    ship_config: ShipConfig,
    score: Score,
}

#[derive(Debug, Serialize, Deserialize)]
struct Score {
    kills: u32,
    deaths: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bullet {
    id: u8,
    object: Object,
}

#[cfg(test)]
mod tests {
    use super::*;
    const DEFAULT_SHIP_CONFIG: ShipConfig = ShipConfig {
        bullet_speed: 10.0,
        bullet_radius: 1.0,
        bullet_mass: 10.0,
        force: 100.0,
    };

    #[test]
    fn add_ship_when_no_ships() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        let id = space
            .add_ship(DVec2::new(10.0, 10.0), 10.0, 100.0, DEFAULT_SHIP_CONFIG)
            .unwrap();
        assert_eq!(1, id);
        assert_eq!(1, space.ships.len());
    }

    #[test]
    fn add_ship_when_other_ship_exists() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        space.add_ship(DVec2::new(10.0, 10.0), 10.0, 100.0, DEFAULT_SHIP_CONFIG);
        let id = space
            .add_ship(DVec2::new(10.0, 10.0), 10.0, 100.0, DEFAULT_SHIP_CONFIG)
            .unwrap();
        assert_eq!(2, id);
        assert_eq!(2, space.ships.len());
    }

    #[test]
    fn remove_existing_ship() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        let id = space
            .add_ship(DVec2::new(10.0, 10.0), 10.0, 100.0, DEFAULT_SHIP_CONFIG)
            .unwrap();
        assert!(space.remove_ship(id));
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn remove_non_existing_ship() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        let id = space
            .add_ship(DVec2::new(10.0, 10.0), 10.0, 100.0, DEFAULT_SHIP_CONFIG)
            .unwrap();
        space.remove_ship(id);
        assert!(!space.remove_ship(id));
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn move_existing_ship() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        let id = space
            .add_ship(DVec2::new(10.0, 10.0), 10.0, 100.0, DEFAULT_SHIP_CONFIG)
            .unwrap();
        assert!(space.move_ship(id, Some(0.0)));
        assert!(space.move_ship(id, None));
    }

    #[test]
    fn move_non_existing_ship() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        assert!(!space.move_ship(1, None));
    }

    #[test]
    fn shoot_existing_ship() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        let id = space
            .add_ship(DVec2::new(10.0, 10.0), 10.0, 100.0, DEFAULT_SHIP_CONFIG)
            .unwrap();
        assert!(space.shoot(id, 0.0));
        assert_eq!(1, space.bullets.len());
    }

    #[test]
    fn shoot_non_existing_ship() {
        let mut space = Space::new(DVec2::new(100.0, 100.0));
        assert!(!space.shoot(1, 0.0));
        assert_eq!(0, space.bullets.len());
    }
}
