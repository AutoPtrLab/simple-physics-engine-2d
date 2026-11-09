//the system that executes all the events happening in the world

use crate::{
    game_systems::creation_system::new_spell,
    input::cursor_state::CursorState,
    render::render_system::RenderSystem,
    ui::UiEvent,
    world::{World, WorldEvent},
};
use simple_physics_engine_2d::math::Vec2;
use simple_physics_engine_2d::v2;

pub fn execute_world_events(w: &mut World, cursor_state: &mut CursorState, ui_events: &mut Vec<UiEvent>, rs: &mut RenderSystem) {
    //we drain the vector
    for e in w.events.drain(..) {
        //static dispatch of the events
        match e {
            WorldEvent::DamagePlayer(damage) => {
                w.level.player_info.hp -= damage;
                if w.level.player_info.hp < 0 {
                    ui_events.push(UiEvent::OpenGameOver);
                    w.level.player_info.hp = 0;
                }
            }
            WorldEvent::BuyObstacle(kind) => {
                *cursor_state = CursorState::PlacingObstacle(kind, None);
            }
            WorldEvent::BuyTower(kind) => *cursor_state = CursorState::PlacingTower(kind),
            WorldEvent::UpgradeObstacle(_id) => println!("UPGRADEADO EL OPSTACULo"),
            WorldEvent::UpgradeTower(_id) => println!("UPGRADEADO EL TOPWEo"),
            WorldEvent::CursorAimState(tower_id) => *cursor_state = CursorState::ChangingTowerDirection(tower_id),
            WorldEvent::CastSpell { kind, pos, dir } => new_spell(&mut w.level.bodies, &mut w.level.spells, kind, pos, dir),
            WorldEvent::MoneyPlayer(pos, money) => {
                w.level.player_info.gold += money;
                rs.new_particle_text(
                    crate::render::particle_text::ParticleTextKind::Money,
                    v2!(pos.x, 1000.0),
                    Vec2::ZERO,
                    money as i16,
                    0.5,
                );
            }
            WorldEvent::DamageEnemy { pos, damage } => {
                rs.new_particle_text(
                    crate::render::particle_text::ParticleTextKind::Damage,
                    pos + v2!(10.0, -15.0),
                    Vec2::ZERO,
                    damage,
                    0.3,
                );
            }
            WorldEvent::DeleteObstacle(id) => {
                w.level.obstacles[id].durability = -1;
                ui_events.push(UiEvent::OpenShop)
            }
            WorldEvent::DeleteTower(id) => {
                let body = w.level.towers.remove(id).expect("failed in delete tower").range_body;
                w.level.bodies.remove(body);
                ui_events.push(UiEvent::OpenShop);
            }
        }
    }
}
