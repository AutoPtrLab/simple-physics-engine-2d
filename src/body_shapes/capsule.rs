use crate::body_shapes::body::*;
use crate::math::Vec2;
use std::f32::consts::PI;

///Factory Functions of the capsule shape, A capsule is made from a line and a radius
/// so we can represent it with the lenght of the line and the radius
///
///
///  ------------------------
/// C    C        I       D   D
///C   ===C=============D===   D The equal sign means the length and the radius is just the rad of the circles ( or the I)
/// C    C        I       D   D
///  ------------------------
///Regular not rotable capsule
pub fn new_capsule(pos: Vec2, vel: Vec2, rad: f32, length: f32, mass: f32, ang: f32) -> Body {
    assert!(mass > 0.0, "mass cannot be 0 or negative");
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
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
        is_rotable: false,
    }
}
//Rotable capsule
pub fn new_rot_capsule(pos: Vec2, vel: Vec2, rad: f32, length: f32, mass: f32, ang: f32) -> Body {
    assert!(mass > 0.0, "mass cannot be 0 or negative");
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
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
        is_rotable: true,
    }
}
//Cinematic capsule(infinite mass)
pub fn new_static_capsule(pos: Vec2, vel: Vec2, rad: f32, length: f32, ang: f32) -> Body {
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
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
        is_rotable: false,
    }
}
//hitbox of a capsule , rotable(ang is in degrees)
pub fn new_hitbox_capsule(pos: Vec2, vel: Vec2, rad: f32, length: f32, ang: f32, ang_vel: f32) -> Body {
    assert!(rad > 0.0, "A capsule cannot have a null or negative radius");
    assert!(length > 0.0, "A capsule cannot have zero or negative length");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: ang * PI / 180.0,
        ang_vel,
        inv_mass: 0.0,
        inv_inert: 0.0,
        shape: Shape::Capsule {
            rad,
            half_len: (length * 0.5),
        },
        is_hitbox: true,
        is_rotable: true,
    }
}
