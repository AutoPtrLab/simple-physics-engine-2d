use simple_physics_engine_2d::math::Vec2;

use crate::timer::Timer;

pub struct ParticlesText {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub timers: Vec<Timer>,

    pub values: Vec<i16>,
    pub kinds: Vec<ParticleTextKind>,
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ParticleTextKind {
    Money,
    Damage,
}
