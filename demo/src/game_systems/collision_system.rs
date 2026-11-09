//Collision system handles the physical collisions

use simple_physics_engine_2d::collision_resolver::CollisionEvent;
use simple_physics_engine_2d::prelude::*;
use simple_physics_engine_2d::v2;

use crate::timer::Timer;
use crate::world::EntityKind;
use crate::world::World;
use crate::world::WorldEvent;
use crate::world::body_data::unpack_body_data;
use crate::world::enemy::Enemy;
use crate::world::enemy::Status;
use crate::world::enemy::StatusKind;
use crate::world::hero::Hero;
use crate::world::level::Terrain;
use crate::world::obstacle::Obstacle;
use crate::world::obstacle::ObstacleKind;
use crate::world::spell::Spell;
use crate::world::spell::SpellKind;
use crate::world::tower::Tower;

const DAMAGE_FACTOR: f32 = 0.0001;
fn enemy_vs_obstacle(
    enemy: &mut Enemy,
    e_body: &mut Body,
    obstacle: &mut Obstacle,
    o_body: &Body,
    _events: &mut Vec<WorldEvent>,
    _dt: f32,
) {
    enemy.hp -= obstacle.damage;
    obstacle.durability -= enemy.damage;
    if obstacle.kind == ObstacleKind::GravityZone {
        //maybe agruparlo en efectos
        e_body.add_impulse(v2!(o_body.ang.cos(), o_body.ang.sin()) * 60.0);
    }
}

fn hero_vs_terrain(hero: &mut Hero, hero_body: &mut Body) {
    match hero_body.user_data as u8 {
        0 => (),
        1 => hero.can_jump = true,
        2 => hero.can_l_climb = true,
        3 => hero.can_r_climb = true,
        _ => (),
    }
}
fn enemy_vs_terrain(
    enemy: &mut Enemy,
    e_body: &mut Body,
    _terrain: &Terrain,
    t_body: &mut Body,
    _impact_force: f32,
    events: &mut Vec<WorldEvent>,
) {
    if t_body.is_hitbox {
        if enemy.hp > 0 {
            events.push(WorldEvent::DamagePlayer(enemy.damage));
        } else {
            events.push(WorldEvent::MoneyPlayer(e_body.pos, enemy.kind.stats().money))
        }
        enemy.hp = i16::MIN;
    } else {
        //if the terrain is no the sewer_core
    }
}
fn enemy_vs_spell(enemy: &mut Enemy, e_body: &mut Body, spell: &mut Spell, s_body: &mut Body, events: &mut Vec<WorldEvent>) {
    enemy.hp -= spell.damage;
    enemy.status = Some(Status {
        timer: Timer::new(0.1),
        kind: StatusKind::Damaged,
    });
    events.push(WorldEvent::DamageEnemy {
        pos: e_body.pos,
        damage: spell.damage,
    });
    spell.is_dead = true;
    match spell.kind {
        SpellKind::Arrow => (),
        SpellKind::EnergyBall => (),
        SpellKind::ExplosionBall => spell.convert(SpellKind::Explosion, Vec2::ZERO, s_body),
        SpellKind::Explosion => {
            let spell_enemy_vec = e_body.pos - s_body.pos;
            //maybe make it depend on how far is from the explosion
            e_body.add_impulse(spell_enemy_vec.normalize() * 5000.0);
        }
        SpellKind::WindGale { ang } => {
            e_body.add_impulse(v2!(ang.cos(), ang.sin()) * 4000.0);
        }
    }
}

fn tower_range_vs_enemy(e_body: &mut Body, tower: &mut Tower, most_down: &mut f32) {
    if e_body.pos.y > *most_down {
        *most_down = e_body.pos.y;
        tower.dir = (e_body.pos - tower.pos).normalize();
    }
}

//resolves the collision of the World

pub fn update_physics(w: &mut World, dt: f32) {
    update_movement(w.level.bodies.values_as_mut_slice(), dt, v2!(0.0, 200.0), 0.1, 0.10);
    update_collisions(w.level.bodies.values_as_mut_slice(), &mut w.collision_events);

    let mut most_down = 0.0; //most down enemy ;

    //the collision event resolver

    for CollisionEvent {
        body_a_id,
        body_b_id,
        impact_force,
    } in w.collision_events.drain(..)
    {
        let (kind_a, entity_id_a) = unpack_body_data(w.level.bodies.values_as_slice()[body_a_id].user_data);
        let (kind_b, entity_id_b) = unpack_body_data(w.level.bodies.values_as_slice()[body_b_id].user_data);
        //Resolver of the clashes between two entities

        let bodies = w.level.bodies.values_as_mut_slice();

        match ((kind_a, entity_id_a, body_a_id), (kind_b, entity_id_b, body_b_id)) {
            ((EntityKind::Obstacle, o_id, o_body), (EntityKind::Enemy, e_id, e_body))
            | ((EntityKind::Enemy, e_id, e_body), (EntityKind::Obstacle, o_id, o_body)) => {
                let [obstacle_body, enemy_body] = bodies.get_disjoint_mut([o_body, e_body]).expect("Failure in Obstacle vs Enemy");
                enemy_vs_obstacle(
                    &mut w.level.enemies[e_id.into()],
                    enemy_body,
                    &mut w.level.obstacles[o_id.into()],
                    obstacle_body,
                    &mut w.events,
                    dt,
                );
            }
            ((EntityKind::Spell, s_id, s_body), (EntityKind::Enemy, e_id, e_body))
            | ((EntityKind::Enemy, e_id, e_body), (EntityKind::Spell, s_id, s_body)) => {
                let [spell_body, enemy_body] = bodies.get_disjoint_mut([s_body, e_body]).expect("Failure in Spell vs Enemy");
                enemy_vs_spell(
                    &mut w.level.enemies[e_id.into()],
                    enemy_body,
                    &mut w.level.spells[s_id.into()],
                    spell_body,
                    &mut w.events,
                );
            }
            ((EntityKind::Terrain, t_id, t_body), (EntityKind::Enemy, e_id, e_body))
            | ((EntityKind::Enemy, e_id, e_body), (EntityKind::Terrain, t_id, t_body)) => {
                let [terrain_body, enemy_body] = bodies.get_disjoint_mut([t_body, e_body]).expect("Failure in Terrain vs Enemy");
                enemy_vs_terrain(
                    &mut w.level.enemies[e_id.into()],
                    enemy_body,
                    &w.level.terrain[t_id.into()],
                    terrain_body,
                    impact_force,
                    &mut w.events,
                );
            }
            ((EntityKind::Tower, t_id, _t_body), (EntityKind::Enemy, _e_id, e_body))
            | ((EntityKind::Enemy, _e_id, e_body), (EntityKind::Tower, t_id, _t_body)) => {
                tower_range_vs_enemy(&mut bodies[e_body], &mut w.level.towers[t_id.into()], &mut most_down);
            }
            ((EntityKind::Terrain, _t_id, _t_body), (EntityKind::Hero, _h_id, h_body))
            | ((EntityKind::Hero, _h_id, h_body), (EntityKind::Terrain, _t_id, _t_body)) => {
                if let Some(hero) = &mut w.level.hero {
                    hero_vs_terrain(hero, &mut bodies[h_body]);
                }
            }
            _ => (),
        }
    }
}
