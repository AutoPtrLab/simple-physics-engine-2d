use crate::body_shapes::body::Body;
use crate::math::Vec2;
use std::f32::consts::TAU;
///Cinematic update of the bodies (the frictions represent how much velocity the body KEEPS, but not the real body-body friction), a Dynamic body gets affected by kinematics(vel, accel ) and external forces,
/// meanwhile kinematics bodies only gets affected by their own vel and not external forces such as gravity, static bodies just dont update
pub fn update_movement(bodies: &mut [Body], dt: f32, grav: Vec2, linear_frict: f32, ang_frict: f32) {
    for b in bodies {
        if b.is_static() {
            continue;
        }
        if b.is_dynamic() {
            b.vel += (b.accel + grav) * dt;
            b.vel *= 1.0 - linear_frict * dt;
            b.ang_vel *= 1.0 - ang_frict * dt;
        }

        b.pos += b.vel * dt;

        b.ang += b.ang_vel * dt;
        b.ang = b.ang.rem_euclid(TAU);
        b.accel = Vec2::ZERO;
    }
}
