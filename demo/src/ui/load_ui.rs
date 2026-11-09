//funcitons to load the UI corresponding to each state

use crate::ui::text::TextKind::GameOver;
use crate::world::WorldEvent;
use crate::world::level::{ObstacleID, TowerID};

use crate::{
    ui::{
        Ui, UiEvent,
        button::{Button, ButtonKind},
        drawing::Drawing,
        text::TextKind::{ObstacleDescriptor, TowerDescriptor},
    },
    world::obstacle::{
        Obstacle,
        ObstacleKind::{self, Bumper},
    },
    world::tower::{Tower, TowerKind},
    world::EntityKind,
};
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::math::Vec2;
use simple_physics_engine_2d::v2;

pub fn load_tower_descriptor(pos: Vec2, tower: &Tower, id: TowerID) -> Ui {
    //load of the radius

    //the buttons position and evreytghihgnis relativo to the UI pos
    let buttons = vec![
        Button::new(
            v2!(100.0, 300.0),
            Vec2mq::new(100.0, 50.0),
            ButtonKind::Wood,
            WorldEvent::UpgradeTower(id).into(),
        ),
        Button::new(
            v2!(100.0, 600.0),
            Vec2mq::new(100.0, 50.0),
            ButtonKind::Wood,
            UiEvent::OpenShop.into(),
        ),
        Button::new(
            v2!(400.0, 300.0),
            Vec2mq::new(100.0, 50.0),
            ButtonKind::Wood,
            WorldEvent::CursorAimState(id).into(),
        ),
        Button::new(
            v2!(400.0, 600.0),
            Vec2mq::new(100.0, 50.0),
            ButtonKind::Wood,
            WorldEvent::DeleteTower(id).into(),
        ),
    ];
    let true_tower_pos = tower.pos - pos;
    let range = tower.kind.stats().range;
    let drawings = vec![
        Drawing::new(v2!(200.0, 100.0), tower.size() * 3.0, EntityKind::Tower, tower.kind as usize),
        Drawing::new(
            true_tower_pos - v2!(range, range),
            Vec2mq::new(range * 2.0, range * 2.0),
            EntityKind::Tower,
            TowerKind::TowerRange as usize,
        ),
    ];
    // drawing the text
    Ui {
        pos,
        buttons,
        drawings,
        text: Some(TowerDescriptor(*tower)),
    }
}

pub fn load_obstacle_descriptor(pos: Vec2, obstacle: &Obstacle, id: ObstacleID) -> Ui {
    //the buttons position and evreytghihgnis relativo to the UI pos
    let buttons = vec![
        Button::new(
            v2!(100.0, 300.0),
            Vec2mq::new(100.0, 50.0),
            ButtonKind::Wood,
            WorldEvent::UpgradeObstacle(id).into(),
        ),
        Button::new(
            v2!(400.0, 300.0),
            Vec2mq::new(100.0, 50.0),
            ButtonKind::Wood,
            UiEvent::OpenShop.into(),
        ),
        Button::new(
            v2!(100.0, 600.0),
            Vec2mq::new(100.0, 50.0),
            ButtonKind::Wood,
            WorldEvent::DeleteObstacle(id).into(),
        ),
    ];
    let drawings = vec![Drawing::new(
        v2!(200.0, 100.0),
        obstacle.size() * 3.0,
        EntityKind::Obstacle,
        obstacle.kind as usize,
    )];
    // drawing the text
    Ui {
        pos,
        buttons,
        drawings,
        text: Some(ObstacleDescriptor(*obstacle)),
    }
}

pub fn load_shop_ui(pos: Vec2) -> Ui {
    let mut x = 100.0;
    let mut y = 100.0;
    let size = Vec2mq::new(100.0, 100.0);
    let mut buttons = vec![];
    let mut drawings = vec![];

    buttons.push(Button::new(
        v2!(x, y),
        size,
        ButtonKind::Wood,
        WorldEvent::BuyObstacle(Bumper).into(),
    ));
    drawings.push(Drawing::new(v2!(x, y), size, EntityKind::Obstacle, ObstacleKind::Bumper as usize));
    x += 200.0;
    buttons.push(Button::new(
        v2!(x, y),
        size,
        ButtonKind::Wood,
        WorldEvent::BuyTower(TowerKind::Archer).into(),
    ));
    x -= 200.0;
    y += 200.0;
    buttons.push(Button::new(
        v2!(x, y),
        size,
        ButtonKind::Wood,
        WorldEvent::BuyTower(TowerKind::Wizard).into(),
    ));
    x += 200.0;
    buttons.push(Button::new(
        v2!(x, y),
        size,
        ButtonKind::Wood,
        WorldEvent::BuyTower(TowerKind::Bomber).into(),
    ));
    x -= 200.0;
    y += 200.0;
    buttons.push(Button::new(
        v2!(x, y),
        size,
        ButtonKind::Wood,
        WorldEvent::BuyObstacle(ObstacleKind::GravityZone).into(),
    ));
    x += 200.0;
    buttons.push(Button::new(
        v2!(x, y),
        size,
        ButtonKind::Wood,
        WorldEvent::BuyObstacle(ObstacleKind::Trampoline).into(),
    ));

    Ui {
        pos,
        buttons,
        drawings,
        text: None,
    }
}

pub fn load_game_over() -> Ui {
    Ui {
        pos: v2!(0.0, 0.0),
        buttons: vec![],
        drawings: vec![],
        text: Some(GameOver),
    }
}
