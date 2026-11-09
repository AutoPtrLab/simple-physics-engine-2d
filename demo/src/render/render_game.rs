use crate::world::enemy::{Enemy, StatusKind};
use crate::world::hero::Hero;
use crate::world::level::{BuildZone, PhysicsID, Terrain};
use crate::world::obstacle::Obstacle;

use crate::RenderSystem;
use crate::world::World;
use crate::world::player_info::PlayerInfo;
use crate::world::spell::Spell;
use crate::world::tower::Tower;
use macroquad::color::{GOLD, GREEN, RED, WHITE, YELLOW};
use macroquad::math::Vec2 as Vec2mq;
use macroquad::shapes::{draw_circle, draw_rectangle};
use macroquad::text::draw_text;
use macroquad::texture::{DrawTextureParams, draw_texture_ex};
use simple_physics_engine_2d::body_shapes::body::{Body, Shape};

use slotmap::DenseSlotMap;

//de render holds the reference to all the sprites in the game

impl RenderSystem {
    // this function charges all the sprites into the vram memory, this may change when there are to many sprites

    // ------------------------------------------
    // PRINCIPAL RENDERwad
    // ------------------------------------------

    pub fn render_game(&self, w: &World) {
        self.render_zones(&w.level.obstacle_zone, &w.level.tower_zone);
        //render all the enemies
        self.render_towers(w.level.towers.values_as_slice());
        self.render_spells(w.level.spells.values_as_slice(), &w.level.bodies);
        self.render_enemies(w.level.enemies.values_as_slice(), &w.level.bodies);
        self.render_obstacles(w.level.obstacles.values_as_slice(), &w.level.bodies);
        self.render_player_info(&w.level.player_info);
        if let Some(hero) = &w.level.hero {
            self.render_hero(hero, &w.level.bodies[hero.body]);
        }
        self.render_terrain(w.level.terrain.values_as_slice(), &w.level.bodies);
    }

    //private funcitons to configure the UI of the player
    fn render_player_info(&self, player_info: &PlayerInfo) {
        let percent: f32 = player_info.hp as f32 / player_info.max_hp as f32;
        draw_rectangle(1500.0, 800.0, 300.0 * percent, 40.0, RED);
        draw_text(format!("Gold : {}", player_info.gold), 1500.0, 700.0, 30.0, GOLD);
    }
    //private to help render each compnent
    fn render_enemies(&self, enemies: &[Enemy], bodies: &DenseSlotMap<PhysicsID, Body>) {
        for e in enemies {
            let x = bodies[e.body].pos.x;
            let y = bodies[e.body].pos.y;
            let ang = bodies[e.body].ang;
            let mut color = WHITE;
            if let Some(status) = &e.status
                && matches!(status.kind, StatusKind::Damaged)
            {
                color.a = 0.4;
                draw_circle(x, y, e.kind.stats().size.x / 2.0, WHITE);
            }
            if e.hp <= 0 {
                color.a = 0.25;
                color.r = 0.5
            }

            draw_texture_ex(
                &self.enemy_sprites[e.kind as usize],
                x - e.size().x * 0.5,
                y - e.size().y * 0.5,
                color,
                DrawTextureParams {
                    dest_size: Some(e.size()),
                    rotation: ang,
                    ..Default::default()
                },
            );
        }
    }
    fn render_obstacles(&self, obstacles: &[Obstacle], bodies: &DenseSlotMap<PhysicsID, Body>) {
        for o in obstacles {
            let x = bodies[o.body].pos.x - o.size().x * 0.5;
            let y = bodies[o.body].pos.y - o.size().y * 0.5;
            let ang = bodies[o.body].ang;

            let mut color = WHITE;
            if matches!(o.kind, crate::world::obstacle::ObstacleKind::GravityZone) {
                color.a = 0.5;
            }
            draw_texture_ex(
                &self.obstacle_sprites[o.kind as usize],
                x,
                y,
                color,
                macroquad::texture::DrawTextureParams {
                    dest_size: Some(o.size()),
                    rotation: ang,

                    ..Default::default()
                },
            );
        }
    }

    fn render_towers(&self, towers: &[Tower]) {
        for t in towers {
            draw_texture_ex(
                &self.tower_sprites[t.kind as usize],
                t.pos.x - t.size().x * 0.5,
                t.pos.y - t.size().y * 0.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(t.size()),

                    ..Default::default()
                },
            )
        }
    }

    fn render_spells(&self, spells: &[Spell], bodies: &DenseSlotMap<PhysicsID, Body>) {
        for s in spells {
            let body = bodies[s.body];
            let x = body.pos.x - s.size().x * 0.5;
            let y = body.pos.y - s.size().y * 0.5;

            draw_texture_ex(
                &self.spell_sprites[usize::from(s.kind)],
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(s.size()),
                    rotation: body.ang,
                    ..Default::default()
                },
            )
        }
    }
    fn render_terrain(&self, terrain: &[Terrain], bodies: &DenseSlotMap<PhysicsID, Body>) {
        for t in terrain {
            let x: f32;
            let y: f32;
            let body = bodies[t.body];
            if matches!(body.shape, Shape::Line { p: _ }) {
                x = body.pos.x;
                y = body.pos.y;
            } else {
                x = body.pos.x - t.size.x * 0.5;
                y = body.pos.y - t.size.y * 0.5;
            }
            draw_texture_ex(
                &self.terrain_sprites[t.kind as usize],
                x,
                y,
                WHITE,
                macroquad::texture::DrawTextureParams {
                    dest_size: Some(t.size),
                    rotation: body.ang,

                    pivot: if matches!(body.shape, Shape::Line { p: _ }) {
                        Some(Vec2mq::new(x, y))
                    } else {
                        None // O el centro por defecto
                    },
                    ..Default::default()
                },
            )
        }
    }
    //render when you are going to place an item of the shop

    //renders the building and obstacle zones
    fn render_zones(&self, obs_zone: &[BuildZone], tower_zone: &[BuildZone]) {
        let mut obs_color = GREEN;
        obs_color.a = 0.3;
        let mut tower_color = YELLOW;
        tower_color.a = 0.3;

        for o in obs_zone {
            match o {
                BuildZone::Circle(c) => draw_circle(c.x, c.y, c.radius(), obs_color),
                BuildZone::Rect(r) => draw_rectangle(r.x, r.y, r.w, r.h, obs_color),
            }
        }
        for t in tower_zone {
            match t {
                BuildZone::Circle(c) => draw_circle(c.x, c.y, c.radius(), tower_color),
                BuildZone::Rect(r) => draw_rectangle(r.x, r.y, r.w, r.h, tower_color),
            }
        }
    }

    fn render_hero(&self, _hero: &Hero, body: &Body) {
        draw_texture_ex(
            &self.enemy_sprites[2],
            body.pos.x - Hero::radius(),
            body.pos.y - Hero::radius(),
            RED,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(Hero::size()),
                rotation: body.ang,
                ..Default::default()
            },
        )
    }
}
