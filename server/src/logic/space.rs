use glam::DVec2;
use rand::Rng;
use sdl2::{render::Canvas, video::Window};
use serde::{Deserialize, Serialize};

use super::{
    object::Update,
    space_object::{Bullet, Planet, Ship},
};
use crate::ui::display::Drawable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    ship_config: ShipConfig,
    bullet_config: BulletConfig,
    planets: Vec<Planet>,
    ships: Vec<Ship>,
    bullets: Vec<Bullet>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShipConfig {
    pub force: f64,
    pub radius: f64,
    pub mass: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulletConfig {
    pub speed: f64,
    pub radius: f64,
    pub mass: f64,
}

impl Space {
    #[cfg(test)]
    pub fn new(ship_config: ShipConfig, bullet_config: BulletConfig) -> Space {
        Space {
            ship_config,
            bullet_config,
            planets: vec![],
            ships: vec![],
            bullets: vec![],
        }
    }

    pub fn get_state_binary(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn draw_all(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for planet in self.planets.iter() {
            planet.draw(canvas)?;
        }
        for ship in self.ships.iter() {
            ship.draw(canvas)?;
        }
        for bullet in self.bullets.iter() {
            bullet.draw(canvas)?;
        }
        Ok(())
    }

    pub fn update(&mut self, time: f64) {
        self.update_planets(time);
        self.update_bullets(time);
        self.update_ships(time);
    }

    #[cfg(test)]
    pub fn add_planet(&mut self, location: DVec2, mass: f64, radius: f64, velocity: DVec2) {
        self.planets
            .push(Planet::new(location, mass, radius, velocity));
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

    pub fn shoot(&mut self, id: u8, direction: f64) {
        let index = self.get_ship_index(id);
        if let Some(index) = index {
            self.bullets
                .push(self.ships[index].shoot(direction, &self.bullet_config));
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

    fn update_bullets(&mut self, time: f64) {
        self.bullets.retain(|bullet| bullet.in_space());
        for bullet in self.bullets.iter_mut() {
            let total_field = self
                .planets
                .iter()
                .map(|other| bullet.get_field(other))
                .sum();
            bullet.update(time, total_field);
        }
        self.bullets
            .retain(|bullet| self.planets.iter().all(|planet| !bullet.collision(planet)));
    }

    fn update_ships(&mut self, time: f64) {
        for ship in self.ships.iter_mut() {
            let total_field = self.planets.iter().map(|other| ship.get_field(other)).sum();
            ship.update(time, total_field);
            if self.planets.iter().any(|planet| ship.collision(planet)) {
                let new_location = DVec2::new(
                    rand::thread_rng().gen_range(0. ..1.),
                    rand::thread_rng().gen_range(0. ..1.),
                );
                ship.respawn(new_location);
            }
            if self.bullets.iter().any(|bullet| ship.collision(bullet)) {
                let new_location = DVec2::new(
                    rand::thread_rng().gen_range(0. ..1.),
                    rand::thread_rng().gen_range(0. ..1.),
                );
                ship.respawn(new_location);
            }
        }
    }

    fn get_ship_index(&self, id: u8) -> Option<usize> {
        self.ships.iter().position(|x| x.get_id() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_space() -> Space {
        let ship_config = ShipConfig {
            force: 0.1,
            radius: 0.1,
            mass: 1.,
        };
        let bullet_config = BulletConfig {
            speed: 0.1,
            radius: 0.1,
            mass: 1.,
        };
        Space::new(ship_config, bullet_config)
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
    fn shoot_ship_exists() {
        let mut space = basic_space();
        space.add_ship(1, DVec2::ZERO);
        space.shoot(1, 0.);
        assert_eq!(1, space.bullets.len());
    }

    #[test]
    fn shoot_no_ship() {
        let mut space = basic_space();
        space.shoot(1, 0.);
        assert_eq!(0, space.bullets.len());
    }

    #[test]
    fn update_space_bullet_no_collision() {
        let mut space = basic_space();
        space.add_planet(DVec2::new(0.5, 0.5), 0., 0.1, DVec2::ZERO);
        space.add_ship(1, DVec2::new(0.7, 0.7));
        space.shoot(1, 0.);
        space.update(0.);
        assert_eq!(1, space.bullets.len());
    }

    #[test]
    fn update_space_bullet_out_of_space() {
        let mut space = basic_space();
        space.add_planet(DVec2::new(0.5, 0.5), 0., 0.1, DVec2::ZERO);
        space.add_ship(1, DVec2::ZERO);
        space.shoot(1, 0.);
        space.update(0.);
        assert_eq!(0, space.bullets.len());
    }
}
