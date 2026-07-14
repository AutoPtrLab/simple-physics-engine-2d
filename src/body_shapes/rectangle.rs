use crate::body_shapes::body::*;
use crate::math::Vec2;

///new rect, all rect are cinematic bodies they can move but are not affected by the other bodies(they have special interactions with others static bodies),
/// the position is in the center of the rectangle
///<----------------> width
///  ______________           4
/// |              |          |
/// |              |          | heigth
/// |       º ->pos|          |
/// |              |          |
/// |______________|          7
///
pub fn new_rect(pos: Vec2, vel: Vec2, width: f32, height: f32) -> Body {
    assert!(width > 0.0, "cannot make a negative width rect");
    assert!(height > 0.0, "cannot make a negative height rect");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0,
        shape: Shape::Rectangle { width, height },
        is_hitbox: false,
        is_rotable: false,
    }
}
pub fn new_hitbox_rect(pos: Vec2, vel: Vec2, width: f32, height: f32) -> Body {
    assert!(width > 0.0, "cannot make a negative width rect");
    assert!(height > 0.0, "cannot make a negative height rect");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0,
        shape: Shape::Rectangle { width, height },
        is_hitbox: true,
        is_rotable: false,
    }
}
