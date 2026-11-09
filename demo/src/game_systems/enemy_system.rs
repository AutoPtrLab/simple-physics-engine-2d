// updates the enemys, iots abilities and their status

use simple_physics_engine_2d::body_shapes::body::Body;
use slotmap::DenseSlotMap;

use crate::world::{enemy::Enemy, level::PhysicsID};

pub fn update_enemies(enemies: &mut [Enemy], bodies: &mut DenseSlotMap<PhysicsID, Body>, dt: f32) {
    for e in enemies {
        if e.hp <= 0 {
            e.convert_to_cadavers(&mut bodies[e.body]);
        }

        if let Some(s) = &mut e.status
            && s.timer.update_and_check(dt) {
                e.status = None;
            }
    }
}
