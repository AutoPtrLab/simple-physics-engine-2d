pub mod body_shapes;
pub mod collision_resolver;
pub mod math;
pub mod movement_systems;

//prelude pattern
pub mod prelude {
    //math
    pub use crate::math::Vec2;
    pub use crate::v2;
    //body shapes
    pub use crate::body_shapes::body::*;
    pub use crate::body_shapes::capsule::*;
    pub use crate::body_shapes::circle::*;
    pub use crate::body_shapes::line::*;
    pub use crate::body_shapes::rectangle::*;

    //important functions
    pub use crate::collision_resolver::update_collisions;
    pub use crate::movement_systems::update_movement;
}
