use crate::math::Vec2;

///enum representing each shape, each field holding its own data
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { rad: f32 },
    Rectangle { width: f32, height: f32 }, //the coords of the rect are in the center of gravity
    Line { p: Vec2 },                      //this is the second point of the line ,being the pos the first one
    Capsule { rad: f32, half_len: f32 },   //
}
///enum representing the behaviour of the body, is the Body is a hitbox this regulates how the hitbox behaves when external forces apply
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BodyType {
    ///It cannot be moved in any situation mass = inf , vel {0 , 0}
    Static,
    ///Can be moved but only if the user wants mass= inf
    Kinematic,
    ///regular dynamic object, even a hitbox gets affected by external forces(gravity, wind ...)
    Dynamic,
}
///Representation of every rigid body
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
    pub inv_inert: f32, //inertia value, if is zero is non-rotable
    pub shape: Shape,
    //variables referencing the behaviour of the body
    pub is_hitbox: bool,
    pub body_type: BodyType,
}

impl Body {
    #[inline]
    pub fn is_rotable(&self) -> bool {
        self.inv_inert != 0.0
    }
    #[inline]
    pub fn is_static(&self) -> bool {
        self.body_type == BodyType::Static
    }
    pub fn is_dynamic(&self) -> bool {
        self.body_type == BodyType::Dynamic
    }
    pub fn is_kinematic(&self) -> bool {
        self.body_type == BodyType::Kinematic
    }
    ///turns a body into their hitbox counterpart
    pub fn make_hitbox(mut self) -> Self {
        self.is_hitbox = true;
        self
    }
    #[inline]
    ///setter method for changing the hitbox status
    pub fn set_hitbox(&mut self, enabled: bool) {
        self.is_hitbox = enabled;
    }
}
