pub mod cleaning_system;
pub mod collision_system;
pub mod creation_system;
pub mod cursor_system;
pub mod enemy_system;
pub mod hero_system;
pub mod spell_system;
pub mod tower_system;
pub mod wave_system;
pub mod world_event_system;

use crate::{game_systems::collision_system::update_physics, input::Input};

use super::world::World;

impl World {
    pub fn update_world(&mut self, input: &mut Input, dt: f32) {
        update_physics(self, dt);
        if let Some(hero) = &mut self.level.hero {
            hero_system::update_hero(input, hero, &mut self.level.bodies, &mut self.events, dt);
        }

        enemy_system::update_enemies(self.level.enemies.values_as_mut_slice(), &mut self.level.bodies, dt);
        spell_system::update_spells(self.level.spells.values_as_mut_slice(), &mut self.level.bodies, dt);
        tower_system::update_towers(
            self.level.towers.values_as_mut_slice(),
            &mut self.level.spells,
            &mut self.level.bodies,
            dt,
        );
        wave_system::update_waves(&mut self.level, dt);
    }
}
