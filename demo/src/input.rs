//checks the input of the user
pub mod cursor_state;
use crate::{
    game_systems::creation_system::{new_obstacle, new_spell, new_tower},
    input::cursor_state::CursorState,
    ui::{
        GameUi,
        UiEvent::{self},
        button::ButtonEvent,
    },
    world::{
        World,
        level::{Level, ObstacleID, TowerID},
        spell::SpellKind::WindGale,
        tower::Tower,
    },
};
use macroquad::prelude::*;
use simple_physics_engine_2d::{
    body_shapes::body::Shape,
    contain_funcs::{self, capsule_contains_point, circ_contains_point, rect_contains_point},
    math::Vec2,
};
use slotmap::DenseSlotMap;

pub mod move_keys {
    pub const Q: u8 = 0b10000000;
    pub const A: u8 = 0b01000000;
    pub const W: u8 = 0b00100000;
    pub const D: u8 = 0b00010000;
    pub const S: u8 = 0b00001000;
    pub const SHIFT: u8 = 0b00000100;
    pub const SPACE: u8 = 0b00000010;
    pub const E: u8 = 0b00000001;
}

#[derive(Default)]
pub struct Input {
    pub mouse_pos: Vec2, // actual mouse position
    pub cursor_state: CursorState,
    pub movement_keys: u8,
}

impl Input {
    //updates the inputs and polls the events that happen outside the game
    pub fn poll_events(&mut self, ui: &mut GameUi, w: &mut World) {
        self.movement_keys = get_mov_keys();
        //update of the global mouse position
        self.mouse_pos = mouse_position().into();
        if is_key_pressed(KeyCode::R) {
            self.cursor_state = CursorState::CastSpell(WindGale { ang: -90.0 });
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            self.cursor_state = CursorState::Idle;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            match &mut self.cursor_state {
                CursorState::Idle => {
                    if let Some(event) = ui.update_buttons(self.mouse_pos) {
                        match event {
                            ButtonEvent::UiEvent(e) => ui.events.push(e),
                            ButtonEvent::WorldEvent(e) => w.events.push(e),
                        }
                    };

                    if let Some(id) = get_clicked_tower(self.mouse_pos, &w.level.towers) {
                        ui.events.push(UiEvent::OpenTowerDescriptor(w.level.towers[id], id))
                    } else if let Some(id) = get_clicked_obstacle(self.mouse_pos, &w.level) {
                        ui.events.push(UiEvent::OpenObstacleDescriptor(w.level.obstacles[id], id));
                    }
                }
                CursorState::PlacingObstacle(kind, position) => {
                    if let Some(pos) = position {
                        let ang = (self.mouse_pos - *pos).ang();

                        if w.level.is_obstacle_builable(*kind, *pos, Some(ang)) {
                            if w.level.player_info.gold >= kind.stats().price {
                                w.level.player_info.gold -= kind.stats().price;
                                new_obstacle(&mut w.level.bodies, &mut w.level.obstacles, *kind, *pos, Some(ang));
                            } else {
                                println!("no te llega campeon");
                            }
                        }
                        self.cursor_state = CursorState::Idle;
                    }
                    //second click to choose the angle
                    else {
                        *position = Some(self.mouse_pos);
                    }
                }

                CursorState::PlacingTower(kind) => {
                    //first phase, we are puting the position

                    //angle phase
                    if w.level.is_tower_builable(*kind, self.mouse_pos) {
                        if w.level.player_info.gold >= kind.stats().price {
                            w.level.player_info.gold -= kind.stats().price;
                            new_tower(&mut w.level.bodies, &mut w.level.towers, *kind, self.mouse_pos);
                        } else {
                            println!("no te llega campeon");
                        }
                    }
                }
                CursorState::ChangingTowerDirection(tower_id) => {
                    let tower = &mut w.level.towers[*tower_id];
                    tower.dir = (self.mouse_pos - tower.pos).normalize();
                    self.cursor_state = CursorState::Idle;
                }

                CursorState::CastSpell(spell_kind) => {
                    new_spell(&mut w.level.bodies, &mut w.level.spells, *spell_kind, self.mouse_pos, Vec2::ZERO);
                    self.cursor_state = CursorState::Idle;
                }
            }
        }

        //if nothing is happening
    }
}

//this funciton return the id of the tower

fn get_clicked_tower(mouse_pos: Vec2, towers: &DenseSlotMap<TowerID, Tower>) -> Option<TowerID> {
    for (i, t) in towers {
        if rect_contains_point(t.pos, t.size().x, t.size().y, mouse_pos) {
            return Some(i);
        }
    }
    None
}

fn get_clicked_obstacle(mouse_pos: Vec2, level: &Level) -> Option<ObstacleID> {
    for (i, o) in &level.obstacles {
        let body = &level.bodies[o.body];
        if match body.shape {
            Shape::Capsule { rad, half_len } => contain_funcs::capsule_contains_point(body.pos, half_len, rad, body.ang, mouse_pos),
            Shape::Circle { rad } => circ_contains_point(body.pos, rad, mouse_pos),
            Shape::Rectangle { half_width, half_height } => rect_contains_point(body.pos, half_width, half_height, mouse_pos),
            Shape::Line { p } => capsule_contains_point(body.pos, ((body.pos - p) / 2.0).len(), 3.0, body.ang, mouse_pos),
        } {
            return Some(i);
        }
    }
    None
}

fn get_mov_keys() -> u8 {
    let mut aux: u8 = 0;
    if is_key_down(KeyCode::A) {
        aux |= move_keys::A;
    }
    if is_key_down(KeyCode::Q) {
        aux |= move_keys::Q;
    }
    if is_key_down(KeyCode::E) {
        aux |= move_keys::E;
    }
    if is_key_down(KeyCode::W) {
        aux |= move_keys::W;
    }
    if is_key_down(KeyCode::S) {
        aux |= move_keys::S;
    }
    if is_key_down(KeyCode::D) {
        aux |= move_keys::D;
    }
    if is_key_down(KeyCode::LeftShift) {
        aux |= move_keys::SHIFT;
    }
    if is_key_down(KeyCode::Space) {
        aux |= move_keys::SPACE;
    }
    aux
}
