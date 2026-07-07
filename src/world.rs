use std::f32::consts::PI;

use crate::body::{self, Body, Shape};
use crate::math::Vec2;
#[derive(Default, Debug)]
pub struct World {
    pub bodies: Vec<body::Body>, //vector holding all the bodies
}

impl World {
    ///creates a new world with the defautl value of 1000 bodies
    pub fn new() -> Self {
        Self {
            bodies: Vec::with_capacity(1000),
        }
    }
    ///creates a new world with a certain capacity
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            bodies: Vec::with_capacity(capacity),
        }
    }

    pub fn add_circle(&mut self, pos: Vec2, vel: Vec2, rad: f32, mass: f32) {
        assert!(mass > 0.0, "mass cannot be 0 or negative");
        self.bodies.push(Body {
            pos,
            vel,
            accel: Vec2::ZERO,
            ang: 0.0,
            ang_vel: 0.0,
            inv_mass: 1.0 / mass,
            inert: 0.5 * mass * rad * rad, //I= 1/2 * M *r²
            shape: Shape::Circle { rad },
        });
    }
    ///new rect, all rect are cinematic bodies they can be moved but are not affected by the other bodies(thought they can affect the others)
    pub fn add_rect(&mut self, pos: Vec2, vel: Vec2, width: f32, height: f32) {
        self.bodies.push(Body {
            pos,
            vel,
            accel: Vec2::ZERO,
            ang: 0.0,
            ang_vel: 0.0,
            inv_mass: 0.0,
            inert: 0.0,
            shape: Shape::Rectangle { width, height },
        });
    }
    ///new line, static line , recieves two bodies
    pub fn add_line(&mut self, p1: Vec2, p2: Vec2) {
        assert!(p1 - p2 != Vec2::ZERO, "Cant make a line with legth zero");
        self.bodies.push(Body {
            pos: p1,
            vel: Vec2::ZERO,
            accel: Vec2::ZERO,
            ang: 0.0,
            ang_vel: 0.0,
            inv_mass: 0.0,
            inert: 0.0,
            shape: Shape::Line { p: (p2) },
        });
    }
    ///adds a capsuel to the world, the lenght here does not count the radius of the two circles,angles in DEG
    pub fn add_capsule(&mut self, pos: Vec2, vel: Vec2, rad: f32, length: f32, mass: f32, ang: f32) {
        assert!(mass > 0.0, "mass cannot be 0 or negative");
        assert!(length > 0.0, "A capsule cannot have zero or negative length");
        let density = mass / (2.0 * length * rad + PI * rad * rad);
        let i = (length * length * length) * rad * 0.166667
            + 2.0 * length * rad * rad * rad
            + PI * rad * rad * length * length * 0.25
            + PI * rad * rad * rad * rad * 0.5;
        self.bodies.push(Body {
            pos,
            vel,
            accel: Vec2::ZERO,
            ang: ang * PI / 180.0,
            ang_vel: 0.0,
            inv_mass: 1.0 / mass,
            inert: density * i,
            shape: Shape::Capsule {
                rad,
                half_len: (length * 0.5),
            },
        });
    }
}
