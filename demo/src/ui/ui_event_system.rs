//system that checks the Ui event

use crate::ui::{
        GameUi, UiEvent,
        load_ui::{load_game_over, load_obstacle_descriptor, load_shop_ui, load_tower_descriptor},
    };
use simple_physics_engine_2d::v2;
impl GameUi {
    pub fn execute_ui_events(&mut self) {
        for e in self.events.drain(..) {
            match e {
                UiEvent::CloseRightPanel => self.right_panel = None,
                UiEvent::OpenTowerDescriptor(tower, id) => self.right_panel = Some(load_tower_descriptor(v2!(1400.0, 0.0), &tower, id)),
                UiEvent::OpenObstacleDescriptor(obstacle, id) => {
                    self.right_panel = Some(load_obstacle_descriptor(v2!(1400.0, 0.0), &obstacle, id))
                }
                UiEvent::OpenShop => self.right_panel = Some(load_shop_ui(v2!(1400.0, 0.0))),
                UiEvent::OpenGameOver => {
                    self.right_panel = None;
                    self.left_panel = None;
                    self.hud = load_game_over();
                }
            }
        }
    }
}
