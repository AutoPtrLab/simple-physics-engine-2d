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
///constants representing a binary number layer for easier use of the mask and layer u16
#[allow(non_snake_case)]
pub mod Layer {

    pub const L0: u16 = 1 << 0;
    pub const L1: u16 = 1 << 1;
    pub const L2: u16 = 1 << 2;
    pub const L3: u16 = 1 << 3;
    pub const L4: u16 = 1 << 4;
    pub const L5: u16 = 1 << 5;
    pub const L6: u16 = 1 << 6;
    pub const L7: u16 = 1 << 7;
    pub const L8: u16 = 1 << 8;
    pub const L9: u16 = 1 << 9;
    pub const L10: u16 = 1 << 10;
    pub const L11: u16 = 1 << 11;
    pub const L12: u16 = 1 << 12;
    pub const L13: u16 = 1 << 13;
    pub const L14: u16 = 1 << 14;
    pub const L15: u16 = 1 << 15;
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
    ///variables referencing the behaviour of the body , is_hitbox determines that the body cannot be affected by other bodies , and the behaviour depends in the BodyType
    pub is_hitbox: bool,
    pub body_type: BodyType,

    pub restitution_coef: f32, // restitution coefficient to address how the collision happens, its value usually goes between 0 and 1.0,althougth you can go over 1 to make the collision gain energy
    pub friction_coef: f32, //frition coefficient to address how is the surface of the material , this cannot   be negative and when two objetct colide we calculate the geometrical median
    ///bit mask representing what kind of object you are or in which layer you live in, default is L0
    pub layer_bits: u16,
    /// bit mask representing with which other kinds of bodies you can interact,default is L0
    pub mask_bits: u16,
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
    #[inline]
    pub fn is_dynamic(&self) -> bool {
        self.body_type == BodyType::Dynamic
    }
    #[inline]
    pub fn is_kinematic(&self) -> bool {
        self.body_type == BodyType::Kinematic
    }
    #[inline]
    ///turns a body into their hitbox counterpart, builder pattern
    pub fn with_hitbox(mut self) -> Self {
        self.is_hitbox = true;
        self
    }
    ///builder with the mask bits representing which layer affects
    pub fn with_mask_bits(mut self, mask: u16) -> Self {
        self.mask_bits = mask;
        self
    }
    ///builder with the layer bit , it can only be active in one layer , else it panics
    pub fn with_layer_bit(mut self, layer: u16) -> Self {
        //check if theres is only one layer active
        if !(self.layer_bits != 0 && (self.layer_bits & (self.layer_bits - 1)) == 0) {
            panic!("The body can only live in one layer at once");
        }
        self.layer_bits = layer;
        self
    }
    #[inline]
    ///setter method for changing the hitbox status
    pub fn set_hitbox(&mut self, enabled: bool) {
        self.is_hitbox = enabled;
    }
    #[inline]
    ///sets a new mask_bits replacing the layer it was before, this represents the layer the entity affects
    pub fn set_mask_bits(&mut self, mask: u16) {
        self.mask_bits = mask;
    }
    #[inline]
    ///sets a new layer_bits replacing the layer it was before, this represet the layer in which the body lives in, it can only live in one layer
    pub fn set_layer_bit(&mut self, layer: u16) {
        if !(self.layer_bits != 0 && (self.layer_bits & (self.layer_bits - 1)) == 0) {
            panic!("The body can only live in one layer at once");
        }
        self.layer_bits = layer;
    }
    #[inline]
    //adds a new bit mask not replcing the existing ones
    pub fn add_mask_bits(&mut self, mask: u16) {
        self.mask_bits |= mask;
    }
}
