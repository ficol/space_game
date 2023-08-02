use glam::DVec2;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::object::{Bullet, Object, Planet, Ship, ShipConfig, Update};
use crate::ui::display::DisplayInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    size: DVec2,
    planets: Vec<Planet>,
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

    pub fn get_state(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn get_display_info(&self) -> Vec<DisplayInfo> {
        let mut display_infos = Vec::new();
        for planet in self.planets.iter() {
            display_infos.push(planet.get_display_info(self.size));
        }
        for ship in self.ships.iter() {
            display_infos.push(ship.get_display_info(self.size));
        }
        for bullet in self.bullets.iter() {
            display_infos.push(bullet.get_display_info(self.size));
        }
        display_infos
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
            planet.update(time, total_planet_fields[i], self.size);
        }
    }

    fn update_bullets(&mut self, time: f64) {
        for bullet in self.bullets.iter_mut() {
            let total_field = self
                .planets
                .iter()
                .map(|other| bullet.get_field(other))
                .sum();
            bullet.update(time, total_field, self.size);
        }
        self.bullets
            .retain(|bullet| self.planets.iter().all(|planet| !bullet.collision(planet)));
    }

    fn update_ships(&mut self, time: f64) {
        let mut kill_ids = Vec::new();
        let mut death_ids = Vec::new();
        for ship in self.ships.iter_mut() {
            let total_field = self.planets.iter().map(|other| ship.get_field(other)).sum();
            ship.update(time, total_field, self.size);
            if self.planets.iter().any(|planet| ship.collision(planet)) {
                death_ids.push(ship.get_id());
            }
            for bullet_hit in self
                .bullets
                .iter()
                .filter(|bullet| ship.get_id() != bullet.get_id() && bullet.collision(ship))
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

    pub fn add_planet(&mut self, object: Object) {
        let planet = Planet::new(object);
        if planet.fit_in(self.size) {
            self.planets.push(planet);
        }
    }

    pub fn add_ship(&mut self, id: u8, object: Object, ship_config: ShipConfig) {
        if self.ships.iter().all(|ship| ship.get_id() != id) {
            self.ships.push(Ship::new(id, object, ship_config));
        }
    }

    pub fn remove_ship(&mut self, id: u8) {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            self.ships.remove(index);
        }
    }

    pub fn move_ship(&mut self, id: u8, direction: Option<f64>) {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            self.ships[index].change_direction(direction);
        }
    }

    pub fn shoot(&mut self, id: u8, direction: f64) {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            self.bullets.push(self.ships[index].shoot(direction));
        }
    }

    fn get_ship_index(&self, id: u8) -> Option<usize> {
        self.ships.iter().position(|x| x.get_id() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ship_no_ships() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.add_ship(1, Object::default(), ShipConfig::default());
        assert_eq!(1, space.ships.len());
    }

    #[test]
    fn add_ship_other_ship_exists() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.add_ship(1, Object::default(), ShipConfig::default());
        space.add_ship(2, Object::default(), ShipConfig::default());
        assert_eq!(2, space.ships.len());
    }

    #[test]
    fn add_ship_same_ship_exists() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.add_ship(1, Object::default(), ShipConfig::default());
        space.add_ship(1, Object::default(), ShipConfig::default());
        assert_eq!(1, space.ships.len());
    }

    #[test]
    fn remove_ship_ship_exists() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.add_ship(1, Object::default(), ShipConfig::default());
        space.remove_ship(1);
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn remove_ship_no_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.remove_ship(1);
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn move_non_existing_ship_no_panic() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.move_ship(1, None);
    }

    #[test]
    fn shoot_ship_exists() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.add_ship(1, Object::default(), ShipConfig::default());
        space.shoot(1, 0.);
        assert_eq!(1, space.bullets.len());
    }

    #[test]
    fn shoot_no_ship() {
        let mut space = Space::new(DVec2::new(100., 100.));
        space.shoot(1, 0.);
        assert_eq!(0, space.bullets.len());
    }
}
