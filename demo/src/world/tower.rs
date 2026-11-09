//towers that shot certain proyectiles or troops

use crate::{
    timer::Timer,
    world::{
        ENEMY_LAYER, TOWER_LAYER,
        level::PhysicsID,
        spell::SpellKind,
    },
};
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::{body_shapes::body::Body, math::Vec2};

pub const NUMBER_OF_TOWERS: usize = 4;

#[derive(Clone, Copy, Debug)]
pub enum TowerKind {
    Wizard, //good damage but not knockback
    Archer, //the archer bullets has little knowback and almost no damage
    Bomber, // bombs, maybe make the trajectory parabolical?
    TowerRange,
}
impl TowerKind {
    pub fn stats(&self) -> Stats {
        match self {
            Self::Wizard => Stats {
                price: 100,
                cooldown: 2.0,
                size: Vec2mq::new(30.0, 30.0),
                spell_kind: SpellKind::EnergyBall,
                range: 400.0,
            },
            Self::Archer => Stats {
                price: 50,
                cooldown: 0.3,
                size: Vec2mq::new(30.0, 30.0),
                spell_kind: SpellKind::Arrow,
                range: 300.0,
            },
            Self::Bomber => Stats {
                price: 150,
                cooldown: 1.0,
                size: Vec2mq::new(30.0, 30.0),
                spell_kind: SpellKind::ExplosionBall,
                range: 300.0,
            },
            Self::TowerRange => {
                panic!("que hace my G");
            }
        }
    }
    pub fn size(&self) -> Vec2mq {
        self.stats().size
    }
    pub fn damage(&self) -> i16 {
        self.stats().spell_kind.stats().damage
    }
    pub fn spell(&self) -> SpellKind {
        self.stats().spell_kind
    }
}
pub struct Stats {
    pub cooldown: f32,
    pub size: Vec2mq,
    pub spell_kind: SpellKind,
    pub range: f32, //range of the tower in the radius of its circle of action
    pub price: u16,
}
#[derive(Clone, Copy)]
pub struct Tower {
    pub kind: TowerKind,
    pub timer: Timer, //time between shots
    pub dir: Vec2,    // direction where the spell is launched
    pub pos: Vec2,
    pub range_body: PhysicsID,
}

impl Tower {
    pub fn size(&self) -> Vec2mq {
        self.kind.size()
    }
    pub fn create_range_circle(kind: TowerKind, pos: Vec2) -> Body {
        Body::new_static_circle(pos, kind.stats().range)
            .with_layer_bit(TOWER_LAYER)
            .with_mask_bits(ENEMY_LAYER)
            .with_hitbox()
    }
}
