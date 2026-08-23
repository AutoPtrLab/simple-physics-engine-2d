use crate::body_shapes::body::BodyType::{Dynamic, Kinematic, Static};
use crate::body_shapes::body::*;
use crate::math::Vec2;
///creates a Dynamic circle (non-rotable)
pub fn new_circle(pos: Vec2, vel: Vec2, rad: f32, mass: f32, restitution_coef: f32, friction_coef: f32) -> Body {
    assert!(mass > 0.0, "mass cannot be 0 or negative");
    assert!(rad > 0.0, "Cannot have a negative or zero  radius circle");
    assert!(
        restitution_coef >= 0.0,
        "Cannot implement a negative restitution coefficient"
    );
    assert!(friction_coef >= 0.0, "Cannot implement a negative friction coefficient");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 1.0 / mass,
        inv_inert: 0.0, //I= 1/2 * M *r²
        shape: Shape::Circle { rad },
        is_hitbox: false,
        body_type: Dynamic,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
///creates a rotable circle
pub fn new_rot_circle(pos: Vec2, vel: Vec2, rad: f32, mass: f32, restitution_coef: f32, friction_coef: f32) -> Body {
    assert!(mass > 0.0, "mass cannot be 0 or negative");
    assert!(rad > 0.0, "Cannot have a negative or zero  radius circle");
    assert!(
        restitution_coef >= 0.0,
        "Cannot implement a negative restitution coefficient"
    );
    assert!(friction_coef >= 0.0, "Cannot implement a negative friction coefficient");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 1.0 / mass,
        inv_inert: 0.5 * mass * rad * rad, //I= 1/2 * M *r²
        shape: Shape::Circle { rad },
        is_hitbox: false,
        body_type: Dynamic,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
///Creates a static circle
pub fn new_static_circle(pos: Vec2, rad: f32, restitution_coef: f32, friction_coef: f32) -> Body {
    assert!(rad > 0.0, "Cannot have a negative or zero  radius circle");
    assert!(
        restitution_coef >= 0.0,
        "Cannot implement a negative restitution coefficient"
    );
    assert!(friction_coef >= 0.0, "Cannot implement a negative friction coefficient");
    Body {
        pos,
        vel: Vec2::ZERO,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0, //I= 1/2 * M *r²
        shape: Shape::Circle { rad },
        is_hitbox: false,
        body_type: Static,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
//Creates a kinematic circle
pub fn new_kinematic_circle(pos: Vec2, vel: Vec2, rad: f32, restitution_coef: f32, friction_coef: f32) -> Body {
    assert!(rad > 0.0, "Cannot have a negative or zero  radius circle");
    assert!(
        restitution_coef >= 0.0,
        "Cannot implement a negative restitution coefficient"
    );
    assert!(friction_coef >= 0.0, "Cannot implement a negative friction coefficient");
    Body {
        pos,
        vel,
        accel: Vec2::ZERO,
        ang: 0.0,
        ang_vel: 0.0,
        inv_mass: 0.0,
        inv_inert: 0.0, //I= 1/2 * M *r²
        shape: Shape::Circle { rad },
        is_hitbox: false,
        body_type: Kinematic,
        restitution_coef,
        friction_coef,
        layer_bits: Layer::L0,
        mask_bits: Layer::L0,
    }
}
