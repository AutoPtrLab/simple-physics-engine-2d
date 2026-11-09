//physical obstacles of the game , they hold the physical id of the obstacle itself
use crate::world::{
    CADAVER_LAYER, ENEMY_LAYER, OBSTACLE_LAYER, TERRAIN_LAYER,
    level::PhysicsID,
};
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::{body_shapes::body::Body, math::Vec2};

pub const NUMBER_OF_OBSTACLES: usize = 3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObstacleKind {
    Bumper, // a circle that contact adding ekinectic energy ly the pinball ones,
    GravityZone,
    Trampoline,
}
//this gives the stats depending on the enum
impl ObstacleKind {
    pub fn stats(&self) -> Stats {
        match self {
            Self::Bumper => {
                let rad = 20.0;
                Stats {
                    price: 50,
                    durability: 10,
                    body: Body::new_static_circle(Vec2::ZERO, rad)
                        .with_layer_bit(OBSTACLE_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER)
                        .with_friction(0.3)
                        .with_restitution(1.0),
                    damage: 0,
                    size: Vec2mq {
                        x: rad * 2.0,
                        y: rad * 2.0,
                    },
                }
            }
            Self::GravityZone => {
                let rad = 40.0;
                let len = 200.0;
                Stats {
                    durability: 10000,
                    price: 50,
                    body: Body::new_static_capsule(Vec2::ZERO, rad, len, 0.0)
                        .with_layer_bit(OBSTACLE_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER)
                        .with_hitbox(),
                    damage: 0,
                    size: Vec2mq {
                        x: rad * 2.0 + len,
                        y: rad * 2.0,
                    },
                }
            }
            Self::Trampoline => {
                let rad = 10.0;
                let len = 20.0;
                Stats {
                    durability: 50,
                    price: 250,
                    body: Body::new_static_capsule(Vec2::ZERO, rad, len, 0.0)
                        .with_layer_bit(OBSTACLE_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER | CADAVER_LAYER)
                        .with_restitution(2.0),
                    damage: 0,
                    size: Vec2mq {
                        x: len + rad * 2.0,
                        y: rad * 2.0,
                    },
                }
            }
        }
    }
    pub fn size(&self) -> Vec2mq {
        self.stats().size
    }
}

pub struct Stats {
    pub durability: i16,
    pub damage: i16,
    pub body: Body,
    pub size: Vec2mq,
    pub price: u16,
}
#[derive(Clone, Copy)]
pub struct Obstacle {
    pub damage: i16,     //damage the obstacle deals to the enemies, is i16 to make operations easier
    pub durability: i16, //how many uses has left
    pub body: PhysicsID,
    pub kind: ObstacleKind,
}

impl Obstacle {
    pub fn size(&self) -> Vec2mq {
        self.kind.size()
    }
    pub fn restore_durability(&mut self) {
        self.durability = self.kind.stats().durability;
    }
}
