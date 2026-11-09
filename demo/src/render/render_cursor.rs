use crate::input::cursor_state::CursorState;
use crate::render::render_system::RenderSystem;
use macroquad::color::WHITE;
use macroquad::texture::{DrawTextureParams, draw_texture_ex};
use simple_physics_engine_2d::math::Vec2;
impl RenderSystem {
    pub fn render_cursor(&self, state: CursorState, pos: Vec2) {
        let mut color = WHITE;
        color.a = 0.5;

        match state {
            CursorState::PlacingObstacle(kind, obs_pos) => {
                let ang: f32;
                if let Some(obs_pos) = obs_pos {
                    ang = (pos - obs_pos).ang();
                    draw_texture_ex(
                        &self.obstacle_sprites[kind as usize],
                        obs_pos.x - kind.size().x * 0.5,
                        obs_pos.y - kind.size().y * 0.5,
                        color,
                        DrawTextureParams {
                            dest_size: Some(kind.size()),
                            rotation: ang,
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        &self.obstacle_sprites[kind as usize],
                        pos.x - kind.size().x * 0.5,
                        pos.y - kind.size().y * 0.5,
                        color,
                        DrawTextureParams {
                            dest_size: Some(kind.size()),
                            ..Default::default()
                        },
                    );
                }
            }
            CursorState::PlacingTower(kind) => {
                draw_texture_ex(
                    &self.tower_sprites[kind as usize],
                    pos.x - kind.size().x * 0.5,
                    pos.y - kind.size().y * 0.5,
                    color,
                    DrawTextureParams {
                        dest_size: Some(kind.size()),
                        ..Default::default()
                    },
                );
            }
            _ => (),
        }
    }
}
