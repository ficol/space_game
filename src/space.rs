use glam::DVec2;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::object::Object;
use crate::object::{self, Planet, Properties, Ship, ShipConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    size: DVec2,
    planets: Vec<object::Planet>,
    ships: Vec<object::Ship>,
    bullets: Vec<object::Bullet>,
}

#[allow(dead_code)]
impl Space {
    pub fn new(size: DVec2) -> Space {
        Space {
            size,
            planets: vec![],
            ships: vec![],
            bullets: vec![],
        }
    }

    // tmp
    pub fn get_objects_location(&self) -> Vec<(f64, f64)> {
        let mut locations = vec![];
        for planet in self.planets.iter() {
            let location = planet.get_location();
            locations.push((location[0], location[1]));
        }
        for ship in self.ships.iter() {
            let location = ship.get_location();
            locations.push((location[0], location[1]));
        }
        for bullet in self.bullets.iter() {
            let location = bullet.get_location();
            locations.push((location[0], location[1]));
        }
        locations
    }

    pub fn update(&mut self, time: f64) {
        self.update_planets(time);
        self.update_bullets(time);
        self.update_ships(time);
    }

    fn update_planets(&mut self, time: f64) {
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
            planet.update(time, &total_planet_fields[i], &self.size);
        }
    }

    fn update_bullets(&mut self, time: f64) {
        for bullet in self.bullets.iter_mut() {
            let total_field = self
                .planets
                .iter()
                .map(|other| bullet.get_field(other))
                .sum();
            bullet.update(time, &total_field, &self.size);
        }
    }

    fn update_ships(&mut self, time: f64) {
        let mut kill_ids = Vec::new();
        let mut death_ids = Vec::new();
        for ship in self.ships.iter_mut() {
            let total_field = self.planets.iter().map(|other| ship.get_field(other)).sum();
            ship.update(time, &total_field, &self.size);
            if self.planets.iter().any(|planet| ship.collision(planet)) {
                death_ids.push(ship.get_id());
            }
            for bullet_hit in self
                .bullets
                .iter()
                .filter(|bullet| ship.get_id() != bullet.get_id() && ship.collision(*bullet))
            {
                death_ids.push(ship.get_id());
                kill_ids.push(bullet_hit.get_id());
            }
        }
        for id in kill_ids.iter() {
            if let Some(i) = self.get_ship_index(*id) {
                self.ships[i].kill();
            }
        }
        let mut rng = rand::thread_rng();
        for id in death_ids.iter() {
            if let Some(i) = self.get_ship_index(*id) {
                let new_location = DVec2::new(
                    rng.gen_range(0. ..self.size[0]),
                    rng.gen_range(0. ..self.size[1]),
                );
                self.ships[i].respawn(new_location);
            }
        }
    }

    pub fn add_planet(&mut self, properties: Properties) -> bool {
        self.planets.push(Planet::new(properties));
        true
    }

    pub fn add_ship(&mut self, properties: Properties, ship_config: ShipConfig) -> Option<u8> {
        let new_id = self.ships.iter().map(|x| x.get_id()).max().unwrap_or(0) + 1;
        self.ships.push(Ship::new(new_id, properties, ship_config));
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
            self.ships[index].change_direction(direction);
            return true;
        }
        false
    }

    pub fn shoot(&mut self, id: u8, direction: f64) -> bool {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            self.bullets.push(self.ships[index].shoot(direction));
            return true;
        }
        false
    }

    fn get_ship_index(&self, id: u8) -> Option<usize> {
        self.ships.iter().position(|x| x.get_id() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ship_when_no_ships() {
        let mut space = Space::new(DVec2::new(100., 100.));
        let id = space
            .add_ship(Properties::default(), ShipConfig::default())
            .unwrap();
        assert_eq!(1, id);
        assert_eq!(1, space.ships.len());
    }

    #[test]
    fn add_ship_when_other_ship_exists() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.add_ship(Properties::default(), ShipConfig::default());
        let id = space
            .add_ship(Properties::default(), ShipConfig::default())
            .unwrap();
        assert_eq!(2, id);
        assert_eq!(2, space.ships.len());
    }

    #[test]
    fn remove_existing_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        let id = space
            .add_ship(Properties::default(), ShipConfig::default())
            .unwrap();
        assert!(space.remove_ship(id));
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn remove_non_existing_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        let id = space
            .add_ship(Properties::default(), ShipConfig::default())
            .unwrap();
        space.remove_ship(id);
        assert!(!space.remove_ship(id));
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn move_existing_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        let id = space
            .add_ship(Properties::default(), ShipConfig::default())
            .unwrap();
        assert!(space.move_ship(id, Some(0.)));
        assert!(space.move_ship(id, None));
    }

    #[test]
    fn move_non_existing_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        assert!(!space.move_ship(1, None));
    }

    #[test]
    fn shoot_existing_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        let id = space
            .add_ship(Properties::default(), ShipConfig::default())
            .unwrap();
        assert!(space.shoot(id, 0.));
        assert_eq!(1, space.bullets.len());
    }

    #[test]
    fn shoot_non_existing_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        assert!(!space.shoot(1, 0.));
        assert_eq!(0, space.bullets.len());
    }
}
