// struct defining the behaviour of the mouse in the game

use simple_physics_engine_2d::math::Vec2;

use crate::world::{
    level::TowerID,
    obstacle::ObstacleKind,
    spell::SpellKind,
    tower::TowerKind,
};

#[derive(Default, Clone, Copy)]
pub enum CursorState {
    #[default]
    Idle, // not doing anything in particular, is able to interact with anything
    PlacingObstacle(ObstacleKind, Option<Vec2>),
    PlacingTower(TowerKind),
    ChangingTowerDirection(TowerID),
    CastSpell(SpellKind),
}
