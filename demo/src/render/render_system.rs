//this file will load all the textures
use crate::input::Input;
use crate::render::particle::{MAX_PARTICLES, Particles};
use crate::render::particle_text::ParticlesText;
use crate::ui::GameUi;
use crate::ui::button::{ButtonKind, NUMBER_OF_BUTTONS};
use crate::world::World;
use crate::world::enemy::{EnemyKind, NUMBER_OF_ENEMIES};
use crate::world::level::{NUMBER_OF_TERRAINS, TerrainKind};

use crate::world::obstacle::{NUMBER_OF_OBSTACLES, ObstacleKind};
use crate::world::spell::NUMBER_OF_SPELLS;
use crate::world::tower::{NUMBER_OF_TOWERS, TowerKind};
use macroquad::texture::{Texture2D, build_textures_atlas, load_texture};

pub struct RenderSystem {
    pub enemy_sprites: [Texture2D; NUMBER_OF_ENEMIES],
    pub obstacle_sprites: [Texture2D; NUMBER_OF_OBSTACLES],
    pub spell_sprites: [Texture2D; NUMBER_OF_SPELLS],
    pub buttons_sprites: [Texture2D; NUMBER_OF_BUTTONS],
    pub tower_sprites: [Texture2D; NUMBER_OF_TOWERS],
    pub terrain_sprites: [Texture2D; NUMBER_OF_TERRAINS],
    pub font: Texture2D,
    pub particles: Particles,
    pub particle_texts: ParticlesText,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self {
            enemy_sprites: std::array::from_fn(|_| Texture2D::empty()),
            obstacle_sprites: std::array::from_fn(|_| Texture2D::empty()),
            spell_sprites: std::array::from_fn(|_| Texture2D::empty()),
            tower_sprites: std::array::from_fn(|_| Texture2D::empty()),
            buttons_sprites: std::array::from_fn(|_| Texture2D::empty()),
            terrain_sprites: std::array::from_fn(|_| Texture2D::empty()),
            font: Texture2D::empty(),
            particles: Particles {
                positions: Vec::with_capacity(MAX_PARTICLES),
                velocities: Vec::with_capacity(MAX_PARTICLES),
                colours: Vec::with_capacity(MAX_PARTICLES),
                timers: Vec::with_capacity(MAX_PARTICLES),
            },
            particle_texts: ParticlesText {
                positions: Vec::with_capacity(MAX_PARTICLES),
                velocities: Vec::with_capacity(MAX_PARTICLES),
                values: Vec::with_capacity(MAX_PARTICLES),
                kinds: Vec::with_capacity(MAX_PARTICLES),
                timers: Vec::with_capacity(MAX_PARTICLES),
            },
        }
    }
    //render general function that recopilats all the other render functions
    pub fn render(&self, w: &World, game_ui: &GameUi, input: &Input) {
        self.render_game(w);
        self.render_game_ui(game_ui);
        self.render_cursor(input.cursor_state, input.mouse_pos);
        self.render_particles();
        self.render_particles_text();
    }
    pub async fn init(&mut self) {
        //enemies sprite load
        self.enemy_sprites[EnemyKind::Slime as usize] = load_texture("assets/sprites/slime.png")
            .await
            .expect("The slime sprite was not found");
        self.enemy_sprites[EnemyKind::Golem as usize] = load_texture("assets/sprites/golem_slime.png")
            .await
            .expect("The golem sprite was not found");
        self.enemy_sprites[EnemyKind::Mini as usize] = load_texture("assets/sprites/mini_slime.png")
            .await
            .expect("The golem sprite was not found");
        //obstacle sprite laod
        self.obstacle_sprites[ObstacleKind::Bumper as usize] = load_texture("assets/sprites/bumper.png")
            .await
            .expect("The bumper sprite was not found");

        self.buttons_sprites[ButtonKind::Wood as usize] = load_texture("assets/sprites/line.png")
            .await
            .expect("Button Wood type texture was not found");
        self.terrain_sprites[TerrainKind::Plains as usize] = load_texture("assets/sprites/line.png")
            .await
            .expect("Plain terrain type texture was not found");
        self.tower_sprites[TowerKind::Archer as usize] = load_texture("assets/sprites/line.png")
            .await
            .expect("Archer tower type texture was not found");
        self.tower_sprites[TowerKind::TowerRange as usize] = load_texture("assets/sprites/tower_range.png")
            .await
            .expect("Archer tower type texture was not found");

        self.font = load_texture("assets/sprites/monofont.png").await.expect("no font");
        //miscelanious

        //build the texture atlas
        build_textures_atlas();
    }
}
