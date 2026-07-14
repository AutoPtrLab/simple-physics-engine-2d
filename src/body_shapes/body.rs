use crate::math::Vec2;

///enum representing each shape, each field holding its own data
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { rad: f32 },
    Rectangle { width: f32, height: f32 }, //the coords of the rect are in the center of gravity
    Line { p: Vec2 },                      //this is the second point of the line ,being the pos the first one
    Capsule { rad: f32, half_len: f32 },   //
}
///Representation of every rigid particle
#[derive(Debug)]
pub struct Body {
    //Cinematic linear vars
    pub pos: Vec2,
    pub vel: Vec2,
    pub accel: Vec2,
    //Cinematic rotacional vars (they are scalars beacuse they are always referencing the z axis)
    pub ang: f32,     //radians, is clockwise
    pub ang_vel: f32, //angular velocity rad/s
    //there is no need for the angular acceleration
    //Dinamic properties
    pub inv_mass: f32,  //1/mass ,better computing and no division / 0
    pub inv_inert: f32, //inertia value, maybe chacnge to the inverse
    pub shape: Shape,
    //variables referencing the behaviour of the body
    pub is_rotable: bool, //
    pub is_hitbox: bool,
}

impl Body {
    //helper
    pub fn is_static(&self) -> bool {
        self.inv_mass == 0.0
    }
}
