use crate::{
    render::render_ui_text::render_text,
    ui::{GameUi, Ui, drawing::Drawing},
    world::EntityKind,
};
use macroquad::{
    color::WHITE,
    texture::{DrawTextureParams, draw_texture_ex},
};
use simple_physics_engine_2d::{math::Vec2, v2};

use crate::{RenderSystem, ui::button::Button};

const SHOP_POSITION: Vec2 = v2!(1455.0, 0.0); // the position where the shop render starts to create

impl RenderSystem {
    //renders all the UI instance of the game Ui struct
    pub fn render_game_ui(&self, game_ui: &GameUi) {
        self.render_ui(&game_ui.hud);
        if let Some(left_panel) = &game_ui.left_panel {
            self.render_ui(left_panel);
        }

        if let Some(right_panel) = &game_ui.right_panel {
            self.render_ui(right_panel);
        }
    }
    fn render_ui(&self, ui: &Ui) {
        self.render_buttons(&ui.buttons, ui.pos);
        self.render_drawings(&ui.drawings, ui.pos);
        render_text(&ui.text, ui.pos);
    }

    fn render_buttons(&self, buttons: &[Button], ui_pos: Vec2) {
        for b in buttons {
            draw_texture_ex(
                &self.buttons_sprites[b.kind as usize],
                b.pos.x + ui_pos.x,
                b.pos.y + ui_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(b.size),
                    ..Default::default()
                },
            );
        }
    }
    fn render_drawings(&self, drawings: &[Drawing], ui_pos: Vec2) {
        for d in drawings {
            let texture = match d.kind {
                EntityKind::Enemy => &self.enemy_sprites[d.id],
                EntityKind::Obstacle => &self.obstacle_sprites[d.id],
                EntityKind::Spell => &self.spell_sprites[d.id],
                EntityKind::Terrain => &self.terrain_sprites[d.id],
                EntityKind::Tower => &self.tower_sprites[d.id],
                EntityKind::Hero => panic!(),
            };

            draw_texture_ex(
                texture,
                d.pos.x + ui_pos.x,
                d.pos.y + ui_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(d.size),
                    ..Default::default()
                },
            );
        }
    }
}
