use crate::body::{self, Body};
#[derive(Default, Debug)]
pub struct World {
    pub bodies: Vec<body::Body>,
    pub collisions: Vec<(usize, usize)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: Vec::with_capacity(1000),
            collisions: Vec::with_capacity(200),
        }
    }
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body)
    }
    /* pub fn add_bodies(&mut self, bodies: [Body]) {
    for b in bodies {
        self.add_body(b);
    } */
    //}
}
