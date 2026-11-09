//funcitonal aproach to render the text in the UIs

use crate::{
    ui::text::TextKind,
    world::obstacle::Obstacle,
    world::tower::Tower,
};
use macroquad::{
    color::{GOLD, GREEN, RED, WHITE},
    text::draw_text,
};
use simple_physics_engine_2d::math::Vec2;

pub fn render_text(text: &Option<TextKind>, ui_pos: Vec2) {
    let Some(text) = text else {
        return;
    };
    match text {
        TextKind::ObstacleDescriptor(obstacle) => render_obstacle_text(obstacle, ui_pos),
        TextKind::TowerDescriptor(tower) => render_tower_text(tower, ui_pos),
        TextKind::GameOver => {
            draw_text("GAME OVER", 600.0, 400.0, 100.0, GREEN);
        }
    };
}

fn render_obstacle_text(obst: &Obstacle, ui_pos: Vec2) {
    let font_size = 20.0;
    let color = WHITE;

    let x = ui_pos.x + 10.0;
    let mut y = ui_pos.y + 20.0;

    let kind = format!("{:?}", obst.kind);
    draw_text(&kind, x, y, font_size + 10.0, GOLD);
    y += 22.0;

    let texto_dur = format!("Durability: {}", obst.durability);
    draw_text(&texto_dur, x, y, font_size, color);
    y += 22.0;

    // 3. Renderizamos el Daño
    let texto_dmg = format!("Damage: {}", obst.damage);
    draw_text(&texto_dmg, x, y, font_size, RED);
}

fn render_tower_text(tower: &Tower, ui_pos: Vec2) {
    let font_size = 20.0;

    // Coordenadas iniciales con márgenes
    let x = ui_pos.x + 10.0;
    let mut y = ui_pos.y + 20.0;

    let kind = format!("{:?}", tower.kind);
    draw_text(&kind, x, y, font_size + 10.0, GOLD);
    y += 22.0; // Salto de línea

    // 2. Renderizamos el Daño
    let dmg = format!("Damage: {}", tower.kind.damage());
    draw_text(&dmg, x, y, font_size, RED);
    y += 22.0;
    let cooldown = format!("Cooldown: {}", tower.kind.stats().cooldown);
    draw_text(&cooldown, x, y, font_size, RED);
}
