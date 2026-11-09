//ui object that shows the select sprite and doesnt do anything else

use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::math::Vec2;

use crate::world::EntityKind;

// the drawing position starts up left
pub struct Drawing {
    pub pos: Vec2,
    pub size: Vec2mq,
    pub kind: EntityKind,
    pub id: usize, // this comes from the EnemyKind or TowerKind enum transform
}

impl Drawing {
    //the id is the specific kind
    pub fn new(pos: Vec2, size: Vec2mq, kind: EntityKind, id: usize) -> Self {
        Drawing { pos, size, kind, id }
    }
}
