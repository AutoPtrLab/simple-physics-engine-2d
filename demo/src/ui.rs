
use simple_physics_engine_2d::math::Vec2;

use crate::{
    ui::{
        button::{Button, ButtonEvent},
        drawing::Drawing,
        text::TextKind,
    },
    world::{
        level::{ObstacleID, TowerID},
        obstacle::Obstacle,
        tower::Tower,
    },
};

pub mod button;
pub mod drawing;
pub mod load_ui;
pub mod text;
pub mod ui_event_system;

#[derive(Clone, Copy)]
pub enum UiEvent {
    CloseRightPanel,
    OpenTowerDescriptor(Tower, TowerID),
    OpenObstacleDescriptor(Obstacle, ObstacleID),
    OpenShop,
    OpenGameOver,
}

#[derive(Default)]
pub struct GameUi {
    pub events: Vec<UiEvent>,

    pub hud: Ui,
    pub left_panel: Option<Ui>,
    pub right_panel: Option<Ui>,
}
impl GameUi {
    pub fn update_buttons(&mut self, mouse_pos: Vec2) -> Option<ButtonEvent> {
        if let Some(e) = self.hud.update_buttons(mouse_pos) {
            return Some(e);
        }
        if let Some(panel) = &mut self.left_panel
            && let Some(event) = panel.update_buttons(mouse_pos) {
                return Some(event);
            }

        if let Some(panel) = &mut self.right_panel
            && let Some(event) = panel.update_buttons(mouse_pos) {
                return Some(event);
            }
        None
    }
}
//this contains all the UI of the game
#[derive(Default)]
pub struct Ui {
    pub pos: Vec2, // position of the Ui so we can make the elements relative to this coordenates, top left
    pub buttons: Vec<Button>,
    pub drawings: Vec<Drawing>,
    pub text: Option<TextKind>,
}

impl Ui {
    fn update_buttons(&mut self, mouse_pos: Vec2) -> Option<ButtonEvent> {
        for b in &mut self.buttons {
            if b.is_pressed(mouse_pos, self.pos) {
                return Some(b.event);
            }
        }
        None
    }
}
