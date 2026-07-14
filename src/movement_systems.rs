use crate::body_shapes::body::Body;
use crate::math::Vec2;
///Cinematic update of the bodies (the frictions represent how much velocity the body KEEPS)
pub fn update_movement(bodies: &mut [Body], dt: f32, grav: Vec2, linear_frict: f32, ang_frict: f32) {
    for b in bodies {
        if b.is_static() {
            continue;
        }
        b.vel += b.accel + grav;
        b.vel *= linear_frict;
        b.pos += b.vel * dt;
        b.ang_vel *= ang_frict;
        b.ang += b.ang_vel * dt;
    }
}
