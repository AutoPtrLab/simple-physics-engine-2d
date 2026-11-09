use simple_physics_engine_2d::{body_shapes::body::Body, math::Vec2};
use slotmap::DenseSlotMap;

//file contituying the individual behaviour of spells
// this includes beofres and after the timer
//
use crate::world::{
    level::PhysicsID,
    spell::{Spell, SpellKind},
};

pub fn update_spells(spells: &mut [Spell], bodies: &mut DenseSlotMap<PhysicsID, Body>, dt: f32) {
    for s in spells {
        //if the timer has already ended... (cleanear)

        if s.life_timer.update_and_check(dt) {
            //if the spell timer has reach the end..
            match s.kind {
                SpellKind::Arrow | SpellKind::EnergyBall | SpellKind::Explosion | SpellKind::WindGale { .. } => {
                    s.is_dead = true;
                }
                SpellKind::ExplosionBall => {
                    s.convert(SpellKind::Explosion, Vec2::ZERO, &mut bodies[s.body]);
                }
            }
        }
    }
}
