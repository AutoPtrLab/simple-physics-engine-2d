use crate::game_systems::creation_system::new_terrain;
use crate::world::CADAVER_LAYER;
//data to form a level

use crate::world::TOWER_LAYER;
use crate::world::enemy::Enemy;
use crate::world::hero::Hero;

use crate::world::obstacle::Obstacle;
use crate::world::obstacle::ObstacleKind;
use crate::world::player_info::PlayerInfo;
use crate::world::spell::Spell;

use crate::world::tower::Tower;
use crate::world::tower::TowerKind;
use crate::world::wave_system::WaveSystem;
use macroquad::math::{Circle, Rect, Vec2 as Vec2mq};
use simple_physics_engine_2d::collision_resolver::check_collision;
use slotmap::dense::DenseSlotMap;

use simple_physics_engine_2d::body_shapes::body::{Body, Shape};
use simple_physics_engine_2d::collision_resolver::{self};
use simple_physics_engine_2d::contain_funcs;
use simple_physics_engine_2d::helpers;
use simple_physics_engine_2d::helpers::get_shape_points;
use simple_physics_engine_2d::math::Vec2;
use simple_physics_engine_2d::v2;

pub const NUMBER_OF_TERRAINS: usize = 1;

#[derive(Clone, Copy)]
pub enum TerrainKind {
    Plains,
}
slotmap::new_key_type! {
   pub  struct PhysicsID;
}
slotmap::new_key_type! {
   pub  struct EnemyID;
}
slotmap::new_key_type! {
   pub  struct ObstacleID;
}
slotmap::new_key_type! {
   pub  struct TowerID;
}
slotmap::new_key_type! {
   pub  struct TerrainID;
}
slotmap::new_key_type! {
   pub  struct SpellID;
}
//where the user can build the tower or the obstacles
//if the body is a hitbox yis automatically a sewer core
pub struct Terrain {
    pub body: PhysicsID,
    pub size: Vec2mq,
    pub kind: TerrainKind,
}

pub enum BuildZone {
    Rect(Rect),
    Circle(Circle),
}

pub struct Level {
    //entities conforming the level
    pub enemies: DenseSlotMap<EnemyID, Enemy>,         //vector holding all the enemies
    pub obstacles: DenseSlotMap<ObstacleID, Obstacle>, //DenseSlotMaptor holding all the physical objects
    pub towers: DenseSlotMap<TowerID, Tower>,
    pub spells: DenseSlotMap<SpellID, Spell>,
    pub hero: Option<Hero>,

    //
    pub bodies: DenseSlotMap<PhysicsID, Body>,
    //
    pub tower_zone: Vec<BuildZone>, // the zone where the player can build the towers, check for the other towers overlapping
    pub obstacle_zone: Vec<BuildZone>, // the zone where the player can build the obstacles,check for the other towers overlapping
    pub terrain: DenseSlotMap<TerrainID, Terrain>, //bodies that are actulizee part of the level it self and cannot be move nor bought
    //waves
    pub wave_sys: WaveSystem,

    pub player_info: PlayerInfo,
}

impl Default for Level {
    fn default() -> Self {
        let mut l = Level::load_level();
        new_terrain(
            &mut l.bodies,
            &mut l.terrain,
            Body::new_line(v2!(400.0, 0.0), v2!(400.0, 1000.0))
                .with_friction(01.0)
                .with_restitution(1.0),
            TerrainKind::Plains,
        );
        new_terrain(
            &mut l.bodies,
            &mut l.terrain,
            Body::new_line(v2!(1000.0, 0.0), v2!(1000.0, 1000.0))
                .with_friction(01.0)
                .with_restitution(1.0),
            TerrainKind::Plains,
        );
        new_terrain(
            &mut l.bodies,
            &mut l.terrain,
            Body::new_line(v2!(400.0, 200.00), v2!(900.0, 300.0))
                .with_friction(01.0)
                .with_restitution(0.3),
            TerrainKind::Plains,
        );
        new_terrain(
            &mut l.bodies,
            &mut l.terrain,
            Body::new_line(v2!(500.0, 500.0), v2!(1000.0, 400.0))
                .with_friction(01.0)
                .with_restitution(0.3),
            TerrainKind::Plains,
        );
        new_terrain(
            &mut l.bodies,
            &mut l.terrain,
            Body::new_line(v2!(400.0, 600.0), v2!(900.0, 700.0))
                .with_friction(01.0)
                .with_restitution(0.3),
            TerrainKind::Plains,
        );
        new_terrain(
            &mut l.bodies,
            &mut l.terrain,
            Body::new_static_rect(v2!(700.0, 800.0), 200.0, 100.0)
                .with_friction(00.2)
                .with_restitution(1.0),
            TerrainKind::Plains,
        );
        new_terrain(
            &mut l.bodies,
            &mut l.terrain,
            Body::new_static_rect(v2!(700.0, 1200.0), 600.0, 100.0)
                .with_friction(00.2)
                .with_restitution(1.0)
                .with_hitbox()
                .with_mask_bits(CADAVER_LAYER),
            TerrainKind::Plains,
        );
        l.new_hero(v2!(700.0, 200.0));
        l
    }
}
impl Level {
    pub fn load_level() -> Self {
        let tower_zone = vec![
            BuildZone::Rect(Rect {
                x: 200.0,
                y: 0.0,
                w: 200.0,
                h: 1000.0,
            }),
            BuildZone::Rect(Rect {
                x: 1000.0,
                y: 0.0,
                w: 200.0,
                h: 1000.0,
            }),
        ];
        let obstacle_zone = vec![BuildZone::Rect(Rect {
            x: 400.0,
            y: 0.0,
            w: 600.0,
            h: 1000.0,
        })];

        Level {
            tower_zone,
            obstacle_zone,
            terrain: DenseSlotMap::default(),
            towers: DenseSlotMap::default(),
            spells: DenseSlotMap::default(),
            enemies: DenseSlotMap::default(),
            obstacles: DenseSlotMap::default(),
            wave_sys: WaveSystem::new(),
            player_info: PlayerInfo::default(),
            hero: None,
            bodies: DenseSlotMap::default(),
        }
    }
    //function that returns true if you are able to build the tower in that position
    pub fn is_tower_builable(&self, kind: TowerKind, pos: Vec2) -> bool {
        //check building zone
        //converting the tower in points

        let points = helpers::get_shape_points::<12>(
            pos,
            Shape::Rectangle {
                half_width: kind.size().x,
                half_height: kind.size().y,
            },
            Some(0.0),
        );
        //we have to check if every point is contained in atleast one of tehe building zone . if any of the points is not contained is a false case
        for p in points {
            let is_valid = self.tower_zone.iter().any(|zone| match zone {
                BuildZone::Circle(c) => contain_funcs::circ_contains_point(v2!(c.x, c.y), c.radius(), p),
                BuildZone::Rect(r) => contain_funcs::rect_contains_point(v2!(r.x + r.w / 2.0, r.y + r.h / 2.00), r.w, r.h, p),
            });
            if !is_valid {
                return false;
            }
        }
        //if is not contain in any of the zones returns false

        //we check the other towers
        for t in self.towers.values() {
            if collision_resolver::collision_rect_rect(pos, kind.size().x, kind.size().y, t.pos, t.size().x, t.size().y).is_some() {
                return false; // if is colliding with any other tower...
            }
        }

        //if nothing goes wrong you can build up the tower
        true
    }

    //function that returns true if you are able to build the obstacle in that position
    pub fn is_obstacle_builable(&self, kind: ObstacleKind, pos: Vec2, ang: Option<f32>) -> bool {
        //points of the shape
        let points = get_shape_points::<12>(pos, kind.stats().body.shape, ang);

        for p in points {
            let is_valid = self.obstacle_zone.iter().any(|zone| match zone {
                BuildZone::Circle(c) => contain_funcs::circ_contains_point(v2!(c.x, c.y), c.radius(), p),
                BuildZone::Rect(r) => contain_funcs::rect_contains_point(v2!(r.x + r.w / 2.0, r.y + r.h / 2.0), r.w, r.h, p),
            });

            if !is_valid {
                return false;
            }
        }

        let mut obstacle_body = kind.stats().body;
        obstacle_body.pos = pos;

        //we check if is colliding with any of the other obstacles
        for b in self.bodies.values() {
            if check_collision(b, &obstacle_body).is_some() && (b.layer_bits & (CADAVER_LAYER | TOWER_LAYER) == 0) {
                return false;
            }
        }
        true
    }
}

//geometrical helper
