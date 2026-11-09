
use crate::{
    timer::Timer,
    world::{
        CADAVER_LAYER, ENEMY_LAYER, EntityKind, HERO_LAYER, OBSTACLE_LAYER, SPELL_LAYER, TERRAIN_LAYER, TOWER_LAYER,
        body_data::to_body_data,
        enemy::{Enemy, EnemyKind},
        level::{EnemyID, ObstacleID, PhysicsID, SpellID, Terrain, TerrainID, TerrainKind, TowerID},
        obstacle::{Obstacle, ObstacleKind},
        spell::{Spell, SpellKind},
        tower::{Tower, TowerKind},
    },
};
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::{
    body_shapes::body::{Body, Shape},
    math::Vec2,
};
use slotmap::{DenseSlotMap, Key};

//this files takes care of the creation of all the WorldEntities
pub fn new_tower(bodies: &mut DenseSlotMap<PhysicsID, Body>, towers: &mut DenseSlotMap<TowerID, Tower>, kind: TowerKind, pos: Vec2) {
    let mut body = Tower::create_range_circle(kind, pos);
    towers.insert_with_key(|tower_key| {
        body.user_data = to_body_data(EntityKind::Tower, tower_key.data().as_ffi());

        Tower {
            kind,
            timer: Timer::new(kind.stats().cooldown),
            dir: Vec2::ZERO,
            pos,
            range_body: bodies.insert(body),
        }
    });
}

pub fn new_spell(
    bodies: &mut DenseSlotMap<PhysicsID, Body>,
    spells: &mut DenseSlotMap<SpellID, Spell>,
    kind: SpellKind,
    pos: Vec2,
    dir: Vec2,
) {
    let mut stats = kind.stats();
    stats.body.pos = pos;
    stats.body.vel = dir * stats.speed;
    spells.insert_with_key(|spell_key| {
        stats.body.user_data = to_body_data(EntityKind::Spell, spell_key.data().as_ffi());
        Spell {
            body: bodies.insert(stats.body),
            kind,
            damage: stats.damage,
            life_timer: Timer::new(stats.life_cooldown),
            is_dead: false,
        }
    });
}

pub fn new_obstacle(
    bodies: &mut DenseSlotMap<PhysicsID, Body>,
    obstacles: &mut DenseSlotMap<ObstacleID, Obstacle>,
    kind: ObstacleKind,
    pos: Vec2,
    ang: Option<f32>,
) {
    let mut stats = kind.stats();
    stats.body.pos = pos;
    stats.body.set_layer_bit(OBSTACLE_LAYER);
    stats
        .body
        .set_mask_bits(OBSTACLE_LAYER | ENEMY_LAYER | CADAVER_LAYER | TERRAIN_LAYER);
    if let Some(ang) = ang {
        stats.body.ang = ang;
    }
    obstacles.insert_with_key(|obs_key| {
        stats.body.user_data = to_body_data(EntityKind::Obstacle, obs_key.data().as_ffi());
        Obstacle {
            damage: stats.damage,
            durability: stats.durability,
            body: bodies.insert(stats.body),
            kind,
        }
    });
}

pub fn new_terrain(
    bodies: &mut DenseSlotMap<PhysicsID, Body>,
    terrain: &mut DenseSlotMap<TerrainID, Terrain>,
    mut body: Body,
    kind: TerrainKind,
) {
    body.set_layer_bit(TERRAIN_LAYER);
    body.set_mask_bits(ENEMY_LAYER | OBSTACLE_LAYER | HERO_LAYER | CADAVER_LAYER);

    let size: Vec2mq = match body.shape {
        Shape::Circle { rad } => Vec2mq {
            x: rad * 2.0,
            y: rad * 2.0,
        },
        Shape::Line { p } => Vec2mq {
            x: (body.pos - p).len(),
            y: 3.0,
        },
        Shape::Rectangle { half_width, half_height } => Vec2mq {
            x: half_width * 2.0,
            y: half_height * 2.0,
        },
        Shape::Capsule { rad, half_len } => Vec2mq {
            x: half_len + rad * 2.0,
            y: rad * 2.0,
        },
    };

    terrain.insert_with_key(|t_key| {
        body.user_data = to_body_data(EntityKind::Terrain, t_key.data().as_ffi());
        Terrain {
            body: bodies.insert(body),
            size,
            kind,
        }
    });
}

pub fn new_enemy(bodies: &mut DenseSlotMap<PhysicsID, Body>, enemies: &mut DenseSlotMap<EnemyID, Enemy>, kind: EnemyKind, pos: Vec2) {
    let mut stats = kind.stats();
    stats.body.pos = pos;
    stats.body.set_layer_bit(ENEMY_LAYER);
    stats
        .body
        .set_mask_bits(OBSTACLE_LAYER | TERRAIN_LAYER | ENEMY_LAYER | SPELL_LAYER | TOWER_LAYER | CADAVER_LAYER);
    //stats.body.user_data = to_body_data(EntityKind::Enemy, self.enemies.len());
    enemies.insert_with_key(|enemy_key| {
        stats.body.user_data = to_body_data(EntityKind::Enemy, enemy_key.data().as_ffi());
        Enemy {
            body: bodies.insert(stats.body),
            kind,
            hp: stats.hp,
            damage: stats.damage,
            status: None,
        }
    });
}
