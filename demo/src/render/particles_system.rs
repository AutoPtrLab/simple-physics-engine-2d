//render the particles
use crate::{
    render::{particle_text::ParticleTextKind, render_system::RenderSystem},
    timer::Timer,
};

const FONT_H: f32 = 16.0;
const FONT_W: f32 = 10.0;

use macroquad::{
    color::{GOLD, RED},
    math::Rect,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, draw_texture_ex},
};
use simple_physics_engine_2d::math::Vec2;

impl RenderSystem {
    pub fn render_particles(&self) {
        for (pos, color) in self.particles.positions.iter().zip(self.particles.colours.iter()) {
            draw_rectangle(pos.x, pos.y, 1.0, 1.0, *color);
        }
    }
    pub fn new_particle_text(&mut self, kind: ParticleTextKind, pos: Vec2, vel: Vec2, value: i16, duration: f32) {
        self.particle_texts.kinds.push(kind);
        self.particle_texts.positions.push(pos);
        self.particle_texts.velocities.push(vel);
        self.particle_texts.values.push(value);
        self.particle_texts.timers.push(Timer::new(duration));
    }
    pub fn render_particles_text(&self) {
        for (pos, (value, kind)) in self
            .particle_texts
            .positions
            .iter()
            .zip(self.particle_texts.values.iter().zip(self.particle_texts.kinds.iter()))
        {
            let color = match kind {
                ParticleTextKind::Damage => RED,
                ParticleTextKind::Money => {
                    draw_texture_ex(
                        &self.font,
                        pos.x - FONT_W,
                        pos.y,
                        GOLD,
                        DrawTextureParams {
                            source: Some(Rect {
                                x: const { FONT_W * 11.0 },
                                y: const { FONT_H * 2.0 },
                                w: FONT_W,
                                h: FONT_H,
                            }),
                            ..Default::default()
                        },
                    );
                    GOLD
                }
            };
            let mut aux_value = *value;
            let mut cont = 0;
            while aux_value != 0 {
                let digit = aux_value % 10;
                aux_value /= 10;
                draw_texture_ex(
                    &self.font,
                    pos.x - cont as f32 * FONT_W,
                    pos.y,
                    color,
                    DrawTextureParams {
                        source: Some(Rect {
                            x: digit as f32 * FONT_W,
                            y: const { FONT_H * 3.0 },
                            w: FONT_W,
                            h: FONT_H,
                        }),
                        ..Default::default()
                    },
                );
                cont += 1;
            }
            //this need to be optimized by using the sprite bitmap of the letters
        }
    }
    pub fn update_particles(&mut self, dt: f32) {
        for i in (0..self.particle_texts.timers.len()).rev() {
            if self.particle_texts.timers[i].update_and_check(dt) {
                self.particle_texts.kinds.swap_remove(i);
                self.particle_texts.positions.swap_remove(i);
                self.particle_texts.velocities.swap_remove(i);
                self.particle_texts.values.swap_remove(i);
                self.particle_texts.timers.swap_remove(i);
            }
        }
        for i in (0..self.particles.timers.len()).rev() {
            if self.particles.timers[i].update_and_check(dt) {
                self.particles.colours.swap_remove(i);
                self.particles.positions.swap_remove(i);
                self.particles.velocities.swap_remove(i);
                self.particles.timers.swap_remove(i);
            }
        }
        for (pos, vel) in self.particles.positions.iter_mut().zip(self.particles.velocities.iter()) {
            *pos += *vel * dt;
        }
        for (pos, vel) in self.particle_texts.positions.iter_mut().zip(self.particle_texts.velocities.iter()) {
            *pos += *vel * dt;
        }
    }
}
