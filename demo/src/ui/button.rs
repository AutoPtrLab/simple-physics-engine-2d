//literraly a button
use macroquad::math::Vec2 as Vec2mq;
use simple_physics_engine_2d::math::Vec2;

use crate::{ui::UiEvent, world::WorldEvent};

pub const NUMBER_OF_BUTTONS: usize = 1;

#[derive(Clone, Copy)]
pub enum ButtonKind {
    Wood,
}
#[derive(Clone, Copy)]
pub enum ButtonEvent {
    WorldEvent(WorldEvent),
    UiEvent(UiEvent),
}
impl From<WorldEvent> for ButtonEvent {
    fn from(value: WorldEvent) -> Self {
        ButtonEvent::WorldEvent(value)
    }
}
impl From<UiEvent> for ButtonEvent {
    fn from(value: UiEvent) -> Self {
        ButtonEvent::UiEvent(value)
    }
}
pub struct Button {
    pub pos: Vec2,          //position of the button , located in the left up corner
    pub size: Vec2mq,       //size of the button
    pub kind: ButtonKind,   //texture of the bottom it self
    pub event: ButtonEvent, //event that the buttom sends
    pub clicked: bool,      //has the button been clicked
}

impl Button {
    pub fn new(pos: Vec2, size: Vec2mq, kind: ButtonKind, event: ButtonEvent) -> Self {
        Button {
            pos,
            size,
            kind,
            event,
            clicked: false,
        }
    }
    //check if the botton is being pressed taking into account the ui_pos
    pub fn is_pressed(&self, mouse_pos: Vec2, ui_pos: Vec2) -> bool {
        //AABB checking bound
        (mouse_pos.x >= self.pos.x + ui_pos.x)
            && (mouse_pos.x <= (self.pos.x + self.size.x + ui_pos.x))
            && (mouse_pos.y >= self.pos.y + ui_pos.y)
            && (mouse_pos.y <= (self.pos.y + self.size.y) + ui_pos.y)
    }
}
