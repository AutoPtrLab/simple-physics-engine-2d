use std::f32::consts::PI;

//spells that the towers or the player can summon
use crate::timer::Timer;
use crate::world::level::PhysicsID;
use crate::world::{ENEMY_LAYER, SPELL_LAYER, TERRAIN_LAYER};
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::body_shapes::body::Body;
use simple_physics_engine_2d::math::Vec2;

pub const NUMBER_OF_SPELLS: usize = 5;

#[derive(Clone, Copy)]
pub enum SpellKind {
    Arrow,
    EnergyBall,    // hitbox, no pushback and is automatically shot to the target (it can actually mise)
    ExplosionBall, // spell that launchea an explosion ball,it generates the explosion when it collides with the enemies
    Explosion,     //the Explosion Spell
    WindGale { ang: f32 },
}

impl From<SpellKind> for usize {
    fn from(spell: SpellKind) -> Self {
        match spell {
            SpellKind::Arrow => 0,
            SpellKind::EnergyBall => 1,
            SpellKind::ExplosionBall => 2,
            SpellKind::Explosion => 3,
            SpellKind::WindGale { .. } => 4,
        }
    }
}
impl SpellKind {
    pub fn stats(&self) -> Stats {
        match self {
            Self::Arrow => {
                let rad = 2.0;
                Stats {
                    body: Body::new_circle(Vec2::ZERO, Vec2::ZERO, rad, 0.5)
                        .with_layer_bit(SPELL_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER)
                        .with_restitution(1.0)
                        .with_gravity_scale(0.0),
                    damage: 20,
                    size: Vec2mq::new(rad * 2.0, rad * 2.0),
                    life_cooldown: 1.0,
                    speed: 400.0,
                }
            }
            Self::EnergyBall => {
                let rad: f32 = 3.0;
                Stats {
                    body: Body::new_kinematic_circle(Vec2::ZERO, Vec2::ZERO, rad)
                        .with_layer_bit(SPELL_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER)
                        .with_hitbox(),
                    damage: 100,
                    size: Vec2mq::new(rad * 2.0, rad * 2.0),
                    life_cooldown: 1.0,
                    speed: 600.0,
                }
            }
            Self::Explosion => {
                let rad: f32 = 30.0;
                Stats {
                    body: Body::new_static_circle(Vec2::ZERO, rad)
                        .with_layer_bit(SPELL_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER)
                        .with_hitbox(),
                    damage: 50,
                    size: Vec2mq::new(rad * 2.0, rad * 2.0),
                    life_cooldown: 0.1,
                    speed: 0.0,
                }
            }
            Self::ExplosionBall => {
                let rad: f32 = 3.0;
                Stats {
                    body: Body::new_kinematic_circle(Vec2::ZERO, Vec2::ZERO, rad)
                        .with_layer_bit(SPELL_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER)
                        .with_hitbox(),
                    damage: 0,
                    size: Vec2mq::new(rad * 2.0, rad * 2.0),
                    life_cooldown: 2.0,
                    speed: 200.0,
                }
            }
            Self::WindGale { ang } => {
                let rad = 30.0;
                let len = 200.0;
                Stats {
                    body: Body::new_static_capsule(Vec2::ZERO, rad, len, *ang * 180.0 / PI)
                        .with_layer_bit(SPELL_LAYER)
                        .with_mask_bits(ENEMY_LAYER | TERRAIN_LAYER)
                        .with_hitbox(),
                    damage: 10,
                    size: Vec2mq::new(len + 2.0 * rad, rad),
                    life_cooldown: 0.0,
                    speed: 0.0,
                }
            }
        }
    }
    pub fn size(&self) -> Vec2mq {
        self.stats().size
    }
}

pub struct Stats {
    pub body: Body,
    pub damage: i16,
    size: Vec2mq,
    pub life_cooldown: f32, //how much the spell last
    pub speed: f32,
}
pub struct Spell {
    pub body: PhysicsID,
    pub kind: SpellKind,
    pub damage: i16,       //damage of the spell, is a signed integer to interact with the health
    pub life_timer: Timer, // how much does the spell last alive
    pub is_dead: bool,     //variable describing the status of the spell
}

impl Spell {
    pub fn size(&self) -> Vec2mq {
        self.kind.size()
    }

    //converts the spell and resets the timer so it doesnt die
    pub fn convert(&mut self, new_kind: SpellKind, _new_dir: Vec2, body: &mut Body) {
        self.is_dead = false;
        let mut stats = new_kind.stats();
        self.damage = stats.damage;
        self.kind = new_kind;
        stats.body.pos = body.pos;
        stats.body.user_data = body.user_data;
        self.life_timer = Timer::new(stats.life_cooldown);

        *body = stats.body;
    }
}
