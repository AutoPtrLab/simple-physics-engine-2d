//!new rect, all rect are cinematic bodies they can move but are not affected by the other bodies(they have special interactions with others static bodies),
//! the position is in the center of the rectangle
//!<----------------> width
//!  ______________           4
//! |              |          |
//! |              |          | heigth
//! |       º ->pos|          |
//! |              |          |
//! |______________|          7
use crate::body_shapes::body::BodyType::{Dynamic, Kinematic, Static};
use crate::body_shapes::body::*;
use crate::math::Vec2;
///regular dynamic non rotbale rect
pub fn new_rect(pos: Vec2, vel: Vec2, width: f32, height: f32, mass: f32) -> Body {
    assert!(mass > 0.0, "cannot make a null or negative mass rect");
    assert!(width > 0.0, "cannot make a negative 2 width rect");
    assert!(height > 0.0, "cannot make a negative height rect");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 1.0 / mass,
        inv_inert: 0.0,
        shape: Shape::Rectangle { width, height },
        is_hitbox: false,
        body_type: Dynamic,
    }
}
///Creates a kinematic non rotable kinematic rect
pub fn new_kinematic_rect(pos: Vec2, vel: Vec2, width: f32, height: f32) -> Body {
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
        body_type: Kinematic,
    }
}
///Creates a statuc rect
pub fn new_static_rect(pos: Vec2, width: f32, height: f32) -> Body {
    assert!(width > 0.0, "cannot make a negative width rect");
    assert!(height > 0.0, "cannot make a negative height rect");
    Body {
        pos,
        vel: Vec2::ZERO,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0,
        shape: Shape::Rectangle { width, height },
        is_hitbox: false,
        body_type: Static,
    }
}
