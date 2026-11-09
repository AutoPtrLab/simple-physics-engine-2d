// wave system

use crate::{
    game_systems::creation_system::new_enemy,
    world::{enemy::EnemyKind, level::Level},
};
use simple_physics_engine_2d::v2;
//
pub fn update_waves(level: &mut Level, dt: f32) {
    if level.wave_sys.current_wave == 1 {
        spawn_enemies(level);
        level.wave_sys.current_wave += 1;
    }
    if level.wave_sys.timer.update_and_check(dt) {
        spawn_enemies(level);
        level.wave_sys.current_wave += 1;
        //we repair all the obstacles
        for o in level.obstacles.values_mut() {
            o.restore_durability();
        }
    }
}
fn spawn_enemies(level: &mut Level) {
    for i in 0..(level.wave_sys.current_wave * 2 + 1) {
        new_enemy(
            &mut level.bodies,
            &mut level.enemies,
            EnemyKind::Mini,
            v2!(500.0, -100.0 + i as f32),
        );
    }
    for i in 0..(level.wave_sys.current_wave / 2) {
        new_enemy(
            &mut level.bodies,
            &mut level.enemies,
            EnemyKind::Golem,
            v2!(500.0 + i as f32 * 30.0, -100.0),
        );
    }

    for i in 0..(level.wave_sys.current_wave + 1) {
        new_enemy(
            &mut level.bodies,
            &mut level.enemies,
            EnemyKind::Slime,
            v2!(500.0 + i as f32 * 30.0, -100.0),
        );
    }
}
