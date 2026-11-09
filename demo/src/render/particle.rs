//particels of the effects , basically 1x1 squares with a color

use macroquad::color::Color;
use simple_physics_engine_2d::math::Vec2;

use crate::timer::Timer;

pub const MAX_PARTICLES: usize = 1000;
pub struct Particles {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub colours: Vec<Color>,
    pub timers: Vec<Timer>,
}
