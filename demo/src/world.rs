pub mod body_data;
pub mod enemy;
pub mod hero;
pub mod level;

pub mod obstacle;
pub mod player_info;
pub mod spell;
pub mod tower;
pub mod wave_system;
//file containing all the world entities
use level::Level;
use obstacle::ObstacleKind;

use simple_physics_engine_2d::body_shapes::body::Layer::{L0, L1, L2, L3, L4, L5, L6, L7};
use simple_physics_engine_2d::collision_resolver::CollisionEvent;
use simple_physics_engine_2d::math::Vec2;
use tower::TowerKind;

use crate::world::level::{ObstacleID, TowerID};
use crate::world::spell::SpellKind;

pub const ENEMY_LAYER: u16 = L0;
pub const OBSTACLE_LAYER: u16 = L1;
pub const SPELL_LAYER: u16 = L2;
pub const TERRAIN_LAYER: u16 = L3;
pub const TOWER_LAYER: u16 = L4; // the layer of the tower range hitbox body
pub const HERO_LAYER: u16 = L5;
pub const MISC_LAYER: u16 = L6;
pub const CADAVER_LAYER: u16 = L7;

//Enum for dispatching the different kind of event the world needs to acommodate , this event are thing that signal sent by functinos which cannot
//change the overall scheme of the World
#[derive(Clone, Copy)]
pub enum WorldEvent {
    DamagePlayer(i16), // damages the player, this happens when an enemy enters to a sewer core
    BuyTower(TowerKind),
    BuyObstacle(ObstacleKind),
    UpgradeTower(TowerID), //this holds the id of the tower
    UpgradeObstacle(ObstacleID),
    CursorAimState(TowerID),
    MoneyPlayer(Vec2, u16),
    DamageEnemy { pos: Vec2, damage: i16 },
    CastSpell { kind: SpellKind, pos: Vec2, dir: Vec2 }, //the dir in the static spells is the angle
    DeleteTower(TowerID),
    DeleteObstacle(ObstacleID),
}

#[derive(Default)]
pub struct World {
    pub events: Vec<WorldEvent>, //Vector that holds all the events to execute by the game that dont really need to be in the system of each entity
    //physical interactions
    pub collision_events: Vec<CollisionEvent>, //vector that holds the collisions events
    pub level: Level,                          // the current level
}
#[derive(Debug)]
pub enum EntityKind {
    Enemy,
    Obstacle,
    Spell,
    Terrain,
    Tower,
    Hero,
}
impl EntityKind {
    pub fn from_u64(num: u64) -> Option<Self> {
        match num {
            0 => Some(EntityKind::Enemy),
            1 => Some(EntityKind::Obstacle),
            2 => Some(EntityKind::Spell),
            3 => Some(EntityKind::Terrain),
            4 => Some(EntityKind::Tower),
            5 => Some(EntityKind::Hero),

            _ => None,
        }
    }
    pub fn to_u64(kind: Self) -> u64 {
        match kind {
            EntityKind::Enemy => 0,
            EntityKind::Obstacle => 1,
            EntityKind::Spell => 2,
            EntityKind::Terrain => 3,
            EntityKind::Tower => 4,
            EntityKind::Hero => 5,
        }
    }
}
