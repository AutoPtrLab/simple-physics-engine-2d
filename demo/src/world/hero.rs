//struct of the controlable character
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::{body_shapes::body::Body, math::Vec2};

use crate::timer::Timer;
use crate::world::level::{Level, PhysicsID};
use crate::world::{EntityKind, HERO_LAYER, TERRAIN_LAYER};
use simple_physics_engine_2d::v2;
pub struct Hero {
    pub body: PhysicsID,
    pub jump_area: PhysicsID,
    pub left_climb_area: PhysicsID,
    pub right_climb_area: PhysicsID,
    pub can_jump: bool,
    pub can_l_climb: bool,
    pub can_r_climb: bool,
    pub timer: Timer,
    pub dash_timer: Timer,
    pub can_dash: bool,
    pub is_dashing: bool,
}

impl Level {
    pub fn new_hero(&mut self, pos: Vec2) {
        let data = (EntityKind::to_u64(EntityKind::Hero) as u128) << 64;
        self.hero = Some(Hero {
            body: self.bodies.insert(
                Body::new_rot_circle(pos, Vec2::ZERO, Hero::radius(), 10.0)
                    .with_data(data)
                    .with_layer_bit(HERO_LAYER)
                    .with_mask_bits(TERRAIN_LAYER)
                    .with_gravity_scale(4.5),
            ),
            jump_area: self.bodies.insert(
                Body::new_kinematic_rect(pos + v2!(0.0, Hero::radius()), Vec2::ZERO, Hero::radius() * 0.001, 10.0)
                    .with_hitbox()
                    .with_layer_bit(HERO_LAYER)
                    .with_mask_bits(TERRAIN_LAYER)
                    .with_data(data + 1),
            ),
            left_climb_area: self.bodies.insert(
                Body::new_kinematic_rect(pos - v2!(Hero::radius(), 0.0), Vec2::ZERO, 10.0, Hero::radius() * 0.5)
                    .with_hitbox()
                    .with_layer_bit(HERO_LAYER)
                    .with_mask_bits(TERRAIN_LAYER)
                    .with_data(data + 2),
            ),
            right_climb_area: self.bodies.insert(
                Body::new_kinematic_rect(pos + v2!(Hero::radius(), 0.0), Vec2::ZERO, 10.0, Hero::radius() * 0.5)
                    .with_hitbox()
                    .with_layer_bit(HERO_LAYER)
                    .with_mask_bits(TERRAIN_LAYER)
                    .with_data(data + 3),
            ),
            can_jump: false,
            can_l_climb: false,
            can_r_climb: false,
            is_dashing: false,
            can_dash: true,
            timer: Timer::new(0.2),
            dash_timer: Timer::new(1.0),
        });
    }
}
impl Hero {
    #[inline]
    pub const fn size() -> Vec2mq {
        Vec2mq {
            x: Hero::radius() * 2.0,
            y: Hero::radius() * 2.0,
        }
    }
    #[inline]
    pub const fn radius() -> f32 {
        20.0
    }
}
