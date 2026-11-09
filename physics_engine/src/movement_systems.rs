use crate::body_shapes::body::Body;
use crate::body_shapes::body::BodyType;
use crate::math::Vec2;
use std::f32::consts::TAU;
///Cinematic update of the bodies (the frictions represent how much velocity the body KEEPS, but not the real body-body friction), a Dynamic body gets affected by kinematics(vel, accel ) and external forces,
/// meanwhile kinematics bodies only gets affected by their own vel and not external forces such as gravity, static bodies just dont update
///
/// take into accounc that the linear friction and angular friciton are 0.0 when there is no friction and there is no max
pub fn update_movement(bodies: &mut [Body], dt: f32, grav: Vec2, linear_frict: f32, ang_frict: f32) {
    for b in bodies {
        if b.is_static() {
            continue;
        }
        if let BodyType::Dynamic { gravity_scale } = b.body_type {
            //apply the acummulated directional impulse

            b.vel += b.tot_impulse * b.inv_mass;
            b.tot_impulse = Vec2::ZERO;
            b.vel += (grav * gravity_scale) * dt;
            b.vel *= 1.0 - linear_frict * dt;
            b.ang_vel *= 1.0 - ang_frict * dt;
        }

        b.pos += b.vel * dt;
        b.ang += b.ang_vel * dt;
        b.ang = b.ang.rem_euclid(TAU);

        // if b.vel.len_sq() < 25.0 {
        //     b.vel = Vec2::ZERO;
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::TAU;

    #[test]
    fn static_bodies_do_not_move_or_rotate() {
        let mut bodies = [Body::new_static_capsule(Vec2::new(10.0, 10.0), 20.0, 20.0, 45.0)];
        bodies[0].ang_vel = 10.0;
        bodies[0].vel = Vec2::new(100.0, 1000.0);
        update_movement(&mut bodies, 1.0, Vec2::new(0.0, -9.8), 0.1, 0.1);

        assert_eq!(bodies[0].pos, Vec2::new(10.0, 10.0));
        assert_eq!(bodies[0].ang, 45.0f32.to_radians());
    }

    #[test]
    fn kinetic_bodies() {
        let mut bodies = [Body::new_kinematic_capsule(
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 0.0),
            20.0,
            20.0,
            45.0,
        )];

        update_movement(&mut bodies, 1.0, Vec2::new(0.0, -1009.8), 0.1, 0.1);

        assert_eq!(bodies[0].pos, Vec2::new(10.0, 10.0));
        assert_eq!(bodies[0].vel, Vec2::ZERO);
    }
    #[test]
    fn gravity_scale() {
        let mut bodies = [
            Body::new_capsule(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), 20.0, 20.0, 50.0, 45.0).with_gravity_scale(2.0),
            Body::new_capsule(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), 20.0, 20.0, 50.0, 45.0),
        ];

        update_movement(&mut bodies, 1.0, Vec2::new(0.0, 90.8), 0.1, 0.1);

        assert!(bodies[0].pos.y > bodies[1].pos.y);
    }
}
