use crate::math::Vec2;

///enum representing each shape, each field holding its own data
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { rad: f32 },
    Rectangle { width: f32, height: f32 }, //the coords of the rect are in the center of gravity
    Line { p: Vec2 },                      //this is the second point of the line ,being the pos the first one
}
///Representation of every rigid particle
#[derive(Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub accel: Vec2,
    pub inv_mass: f32, //better computong
    pub shape: Shape,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, accel: Vec2, mass: f32, shape: Shape) -> Self {
        assert!(mass > 0.0, "Not a valid mass!");
        Self {
            pos,
            vel,
            accel,
            inv_mass: 1.0 / mass,
            shape,
        }
    }
    //an object that is not affected by the other dynamics
    pub fn new_static(pos: Vec2, shape: Shape) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            accel: Vec2::ZERO,
            //mass = 1/inv_mass  => 1/0.0 =inf in this case
            inv_mass: 0.0,
            shape,
        }
    }
}
