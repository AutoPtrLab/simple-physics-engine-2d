//file describing the text part of the UI

use crate::world::{obstacle::Obstacle, tower::Tower};

pub enum TextKind {
    TowerDescriptor(Tower),
    ObstacleDescriptor(Obstacle),
    GameOver,
}
