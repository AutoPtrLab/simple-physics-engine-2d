//! A line is a geometrical figure represented with two points, is static and cannot have neither velocity nor rotation
//!
//!p1-> o====================o ->p2
//!
use crate::body_shapes::body::BodyType::Static;
use crate::body_shapes::body::Layer;
use crate::body_shapes::body::{Body, Shape};
use crate::math::Vec2;

impl Body {
    pub fn new_line(p1: Vec2, p2: Vec2) -> Body {
        assert!(p1 - p2 != Vec2::ZERO, "Cant make a line with legth zero");

        let vec = p2 - p1;
        let ang = -vec.y.atan2(vec.x);
        Body {
            pos: p1,
            vel: Vec2::ZERO,
            accel: Vec2::ZERO,
            ang,
            ang_vel: 0.0,
            inv_mass: 0.0,
            inv_inert: 0.0,
            shape: Shape::Line { p: (p2) },
            is_hitbox: false,
            body_type: Static,
            restitution_coef: 0.0,
            friction_coef: 0.0,
            layer_bits: Layer::L0,
            mask_bits: Layer::L0,
        }
    }
}
