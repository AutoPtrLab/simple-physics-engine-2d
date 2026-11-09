//updates the hero position
use crate::{
    input::{
        Input,
        move_keys::{self},
    },
    world::{
        WorldEvent,
        hero::Hero,
        level::PhysicsID,
        spell::SpellKind,
    },
};
use simple_physics_engine_2d::{body_shapes::body::Body, math::Vec2, v2};
use slotmap::DenseSlotMap;

pub fn update_hero(input: &Input, hero: &mut Hero, bodies: &mut DenseSlotMap<PhysicsID, Body>, events: &mut Vec<WorldEvent>, dt: f32) {
    let body = &mut bodies[hero.body];
    let mut movement: Vec2 = v2!(0.0, 0.0);
    if !hero.can_dash && hero.dash_timer.update_and_check(dt) {
        // this if is short circuited so if can dash is true the timer does not execute
        hero.can_dash = true;
    }

    

    if hero.is_dashing {
        body.set_gravity_scale(0.0);
        if hero.timer.update_and_check(dt) {
            hero.is_dashing = false;
        }
    } else {
        body.set_gravity_scale(4.5);

        if (input.movement_keys & move_keys::A) != 0 {
            movement.x -= 1.0;
        }

        if (input.movement_keys & move_keys::D) != 0 {
            movement.x += 1.0;
        }
        body.vel.x = movement.x * 200.0;

        if (input.movement_keys & move_keys::S) != 0 {
            movement.y += 1.0;
            body.add_impulse(v2!(0.0, 100.0));
        }
        if (input.movement_keys & move_keys::W) != 0 {
            movement.y -= 1.0;
            if hero.can_jump {
                //body.add_impulse(v2!(0.0, -400.0))
                body.vel = v2!(0.0, -550.0);
            } else if hero.can_l_climb {
                hero.is_dashing = true;
                //body.add_impulse(v2!(5000.0, -2000.0));
                body.vel = v2!(500.0, -500.0);
            } else if hero.can_r_climb {
                hero.is_dashing = true;
                //body.add_impulse(v2!(-5000.0, -2000.0));
                body.vel = v2!(-500.0, -500.0);
            }
        }
        //dash attack
        if (input.movement_keys & move_keys::SHIFT) != 0 && hero.can_dash {
            let norm_mov = movement.normalize();

            body.vel = norm_mov * 800.0;
            hero.can_dash = false;
            hero.is_dashing = true;
            events.push(WorldEvent::CastSpell {
                kind: SpellKind::WindGale { ang: norm_mov.ang() },
                pos: body.pos + 50.0 * norm_mov,
                dir: Vec2::ZERO,
            });
        }
    }

    if (hero.can_l_climb && movement.x == -1.0) || (hero.can_r_climb && movement.x == 1.0) {
        body.vel *= 0.9 * (1.0 - dt);
    }
    hero.can_jump = false;
    hero.can_l_climb = false;
    hero.can_r_climb = false;
    
    

    let body_pos = body.pos;
    bodies[hero.jump_area].pos = body_pos + v2!(0.0, Hero::radius());
    bodies[hero.right_climb_area].pos = body_pos + v2!(Hero::radius(), 0.0);
    bodies[hero.left_climb_area].pos = body_pos - v2!(Hero::radius(), 0.0);
}
