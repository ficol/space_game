use glam::DVec2;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{
    object::Update,
    space_object::{Planet, Ship},
};
use crate::ui::display::Drawer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    ship_config: ShipConfig,
    planets: Vec<Planet>,
    ships: Vec<Ship>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShipConfig {
    pub force: f64,
    pub radius: f64,
    pub mass: f64,
    pub field: f64,
}

impl Space {
    #[cfg(test)]
    pub fn new(ship_config: ShipConfig) -> Space {
        Space {
            ship_config,
            planets: vec![],
            ships: vec![],
        }
    }

    pub fn get_state_binary(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn draw_all(
        &self,
        drawer: &mut impl Drawer,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        for planet in self.planets.iter() {
            drawer.draw(planet.get_display_info(), width, height)?;
        }
        for ship in self.ships.iter() {
            drawer.draw(ship.get_display_info(), width, height)?;
        }
        Ok(())
    }

    pub fn update(&mut self, time: f64) {
        self.update_planets(time);
        self.update_ships(time);
    }

    pub fn add_ship(&mut self, id: u8, location: DVec2) {
        if self.ships.iter().all(|ship| ship.get_id() != id) {
            self.ships.push(Ship::new(id, location, &self.ship_config));
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
            planet.update(time, total_planet_fields[i]);
        }
    }

    fn update_ships(&mut self, time: f64) {
        for ship in self.ships.iter_mut() {
            let total_field = self.planets.iter().map(|other| ship.get_field(other)).sum();
            ship.update(time, total_field);
            if self.planets.iter().any(|planet| ship.collision(planet)) {
                ship.respawn(gen_new_location());
            }
        }
    }

    fn get_ship_index(&self, id: u8) -> Option<usize> {
        self.ships.iter().position(|x| x.get_id() == id)
    }

    #[cfg(test)]
    fn add_planet(&mut self, location: DVec2, mass: f64, field: f64, radius: f64, velocity: DVec2) {
        self.planets
            .push(Planet::new(location, mass, field, radius, velocity));
    }
}

pub fn gen_new_location() -> DVec2 {
    let mut rng = rand::thread_rng();
    DVec2::new(rng.gen_range(0. ..1.), rng.gen_range(0. ..1.))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_space() -> Space {
        let ship_config = ShipConfig {
            force: 0.1,
            radius: 0.1,
            mass: 2.,
            field: 1.,
        };
        Space::new(ship_config)
    }

    #[test]
    fn add_ship_no_ships() {
        let mut space = basic_space();
        space.add_ship(1, DVec2::ZERO);
        assert_eq!(1, space.ships.len());
    }

    #[test]
    fn add_ship_other_ship_exists() {
        let mut space = basic_space();
        space.add_ship(1, DVec2::ZERO);
        space.add_ship(2, DVec2::ZERO);
        assert_eq!(2, space.ships.len());
    }

    #[test]
    fn add_ship_same_ship_exists() {
        let mut space = basic_space();
        space.add_ship(1, DVec2::ZERO);
        space.add_ship(1, DVec2::ZERO);
        assert_eq!(1, space.ships.len());
    }

    #[test]
    fn remove_ship_ship_exists() {
        let mut space = basic_space();
        space.add_ship(1, DVec2::ZERO);
        space.remove_ship(1);
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn remove_ship_no_ship() {
        let mut space = basic_space();
        space.remove_ship(1);
        assert_eq!(0, space.ships.len());
    }

    #[test]
    fn move_non_existing_ship_no_panic() {
        let mut space = basic_space();
        space.move_ship(1, None);
    }

    #[test]
    fn update_space_ship_no_collision() {
        let mut space = basic_space();
        space.add_planet(DVec2::new(1., 0.), 0., 1., 0.1, DVec2::ZERO);
        space.add_ship(1, DVec2::new(0., 0.));
        space.update(1.);
        assert_eq!(1, space.ships.len());
        let (_, velocity, _) = space.ships[0].get_motion();
        assert_eq!(DVec2::new(0.5, 0.), velocity);
    }

    #[test]
    fn update_space_ship_collision_planet() {
        let mut space = basic_space();
        space.add_planet(DVec2::new(0.5, 0.5), 0., 0.1, 0.1, DVec2::ZERO);
        space.add_ship(1, DVec2::new(0.5, 0.5));
        space.update(1.);
        assert_eq!(1, space.ships.len());
        let (_, velocity, _) = space.ships[0].get_motion();
        assert_eq!(DVec2::ZERO, velocity);
    }
}
