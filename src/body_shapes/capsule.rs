//!Factory Functions of the capsule shape, A capsule is made from a line and a radius
//! so we can represent it with the lenght of the line and the radius
//!
//!
//!  ------------------------
//! C    C        I       D   D
//!C   ===C=============D===   D The equal sign means the length and the radius is just the rad of the circles ( or the I)
//! C    C        I       D   D
//!  ------------------------
use crate::body_shapes::body::BodyType::{Dynamic, Kinematic, Static};
use crate::body_shapes::body::*;
use crate::math::Vec2;
use std::f32::consts::PI;

///Regular not rotable capsule
pub fn new_capsule(
    pos: Vec2,
    vel: Vec2,
    rad: f32,
    length: f32,
    mass: f32,
    ang: f32,
    restitution_coef: f32,
    friction_coef: f32,
) -> Body {
    assert!(mass > 0.0, "mass cannot be 0 or negative");
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
    assert!(
        restitution_coef >= 0.0,
        "Cannot implement a negative restitution coefficient"
    );
    assert!(friction_coef >= 0.0, "Cannot implement a negative friction coefficient");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: ang * PI / 180.0,
        ang_vel: 0.0,
        inv_mass: 1.0 / mass,
        inv_inert: 0.0, //non rotable
        shape: Shape::Capsule {
            rad,
            half_len: (length * 0.5),
        },
        is_hitbox: false,
        body_type: Dynamic,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
///Rotable capsule
pub fn new_rot_capsule(
    pos: Vec2,
    vel: Vec2,
    rad: f32,
    length: f32,
    mass: f32,
    ang: f32,
    restitution_coef: f32,
    friction_coef: f32,
) -> Body {
    assert!(mass > 0.0, "mass cannot be 0 or negative");
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
    assert!(
        restitution_coef >= 0.0,
        "Cannot implement a negative restitution coefficient"
    );
    assert!(friction_coef >= 0.0, "Cannot implement a negative friction coefficient");
    let density = mass / (2.0 * length * rad + PI * rad * rad);
    let i = (length * length * length) * rad * 0.166667
        + 2.0 * length * rad * rad * rad
        + PI * rad * rad * length * length * 0.25
        + PI * rad * rad * rad * rad * 0.5;
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: ang * PI / 180.0,
        ang_vel: 0.0,
        inv_mass: 1.0 / mass,
        inv_inert: 1.0 / (density * i),
        shape: Shape::Capsule {
            rad,
            half_len: (length * 0.5),
        },
        is_hitbox: false,
        body_type: Dynamic,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
///new static capsule
pub fn new_static_capsule(
    pos: Vec2,
    rad: f32,
    length: f32,
    ang: f32,
    restitution_coef: f32,
    friction_coef: f32,
) -> Body {
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
    Body {
        pos,
        vel: Vec2::ZERO,
        accel: Vec2::ZERO,
        ang: ang * PI / 180.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0,
        shape: Shape::Capsule {
            rad,
            half_len: (length * 0.5),
        },
        is_hitbox: false,
        body_type: Static,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
///kinematic capsule(infinite mass)
pub fn new_kinematic_capsule(
    pos: Vec2,
    vel: Vec2,
    rad: f32,
    length: f32,
    ang: f32,
    restitution_coef: f32,
    friction_coef: f32,
) -> Body {
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
    assert!(
        restitution_coef >= 0.0,
        "Cannot implement a negative restitution coefficient"
    );
    assert!(friction_coef >= 0.0, "Cannot implement a negative friction coefficient");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: ang * PI / 180.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0,
        shape: Shape::Capsule {
            rad,
            half_len: (length * 0.5),
        },
        is_hitbox: false,
        body_type: Kinematic,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
