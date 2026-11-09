use crate::timer::Timer;


use crate::world::level::PhysicsID;
use crate::world::{CADAVER_LAYER, ENEMY_LAYER, OBSTACLE_LAYER, SPELL_LAYER, TERRAIN_LAYER};
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::body_shapes::body::Body;
use simple_physics_engine_2d::math::Vec2;

pub const NUMBER_OF_ENEMIES: usize = 3;
//Circular enemie
#[derive(Clone, Copy)]
pub enum EnemyKind {
    Slime,
    Golem,
    Mini,
}
#[derive(Clone, Copy)]
pub enum StatusKind {
    Damaged,
}
pub struct Status {
    pub timer: Timer, // how much does the effect lasts
    pub kind: StatusKind,
}
impl EnemyKind {
    pub fn stats(&self) -> Stats {
        match self {
            Self::Slime => {
                let rad = 15.0;
                Stats {
                    damage: 1,
                    hp: 100,
                    money: 100,
                    body: Body::new_rot_circle(Vec2::ZERO, Vec2::ZERO, rad, 15.0)
                        .with_layer_bit(ENEMY_LAYER)
                        .with_mask_bits(OBSTACLE_LAYER | SPELL_LAYER | ENEMY_LAYER | TERRAIN_LAYER | CADAVER_LAYER)
                        .with_friction(0.3)
                        .with_restitution(1.0),
                    size: Vec2mq::new(rad * 2.0, rad * 2.0),
                }
            }
            Self::Golem => {
                let rad = 25.0;
                Stats {
                    damage: 4,
                    hp: 300,
                    money: 200,
                    body: Body::new_rot_circle(Vec2::ZERO, Vec2::ZERO, rad, 20.0)
                        .with_layer_bit(ENEMY_LAYER)
                        .with_mask_bits(OBSTACLE_LAYER | SPELL_LAYER | ENEMY_LAYER)
                        .with_friction(0.1)
                        .with_restitution(0.6)
                        .with_gravity_scale(2.0),
                    size: Vec2mq::new(rad * 2.0, rad * 2.0),
                }
            }
            Self::Mini => {
                let rad = 7.0;
                Stats {
                    damage: 1,
                    hp: 55,
                    money: 5,
                    body: Body::new_rot_circle(Vec2::ZERO, Vec2::ZERO, rad, 10.0)
                        .with_layer_bit(ENEMY_LAYER)
                        .with_mask_bits(OBSTACLE_LAYER | SPELL_LAYER | ENEMY_LAYER)
                        .with_friction(0.1)
                        .with_restitution(0.9)
                        .with_gravity_scale(0.5),
                    size: Vec2mq::new(rad * 2.0, rad * 2.0),
                }
            }
        }
    }
    fn size(&self) -> Vec2mq {
        self.stats().size
    }
}
pub struct Stats {
    pub damage: i16,
    pub hp: i16,
    pub body: Body,
    pub size: Vec2mq,
    pub money: u16,
}
pub struct Enemy {
    pub body: PhysicsID,
    pub kind: EnemyKind,
    pub hp: i16, //health amount of the Enemy
    // pub is_dead: bool, //determines if the enemie is alive or dead at that moment
    pub damage: i16, //damage the enemy deals to the core and to the other obstacles , maybe this changes
    pub status: Option<Status>,
}

impl Enemy {
    pub fn size(&self) -> Vec2mq {
        self.kind.size()
    }
    pub fn convert_to_cadavers(&mut self, body: &mut Body) {
        body.set_layer_bit(CADAVER_LAYER);
    }
}
