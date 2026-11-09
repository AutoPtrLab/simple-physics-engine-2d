//the funcitons that help the towers create spells


use simple_physics_engine_2d::{body_shapes::body::Body, math::Vec2};
use slotmap::DenseSlotMap;

use crate::{
    game_systems::creation_system::new_spell,
    world::{
        level::{PhysicsID, SpellID},
        spell::Spell,
        tower::Tower,
    },
};

pub fn update_towers(towers: &mut [Tower], spells: &mut DenseSlotMap<SpellID, Spell>, bodies: &mut DenseSlotMap<PhysicsID, Body>, dt: f32) {
    // we have to update depending of the kind of tower

    //update the timer
    for t in towers {
        //take into account that if the tower has not started shooting it has a little delay in teh timer, in rapid towers this does not matter but
        // if the cooldown is higher...
        if t.timer.update_and_check(dt) && t.dir != Vec2::ZERO {
            new_spell(bodies, spells, t.kind.spell(), t.pos, t.dir);

            t.dir = Vec2::ZERO;
        }
    } //we reset the t.dir to zero so is not shooting
}
