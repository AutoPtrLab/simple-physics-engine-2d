use crate::body_shapes::body::*;
use crate::math::Vec2;
///creates a circle
pub fn new_circle(pos: Vec2, vel: Vec2, rad: f32, mass: f32) -> Body {
    assert!(mass > 0.0, "mass cannot be 0 or negative");
    assert!(rad > 0.0, "Cannot have a negative or zero  radius circle");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 1.0 / mass,
        inv_inert: 1.0 / (0.5 * mass * rad * rad), //I= 1/2 * M *r²
        shape: Shape::Circle { rad },
        is_hitbox: false,
        is_rotable: false,
    }
}
///creates a circle hitbox
pub fn new_hitbox_circle(pos: Vec2, vel: Vec2, rad: f32) -> Body {
    assert!(rad > 0.0, "Cannot have a negative or zero radius circle");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0, //I= 1/2 * M *r²
        shape: Shape::Circle { rad },
        is_hitbox: true,
        is_rotable: false,
    }
}
