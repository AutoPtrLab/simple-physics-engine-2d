use macroquad::prelude::*;

mod game_systems;
mod input;

mod render;

mod timer;

mod ui;

mod world;
use crate::game_systems::{cleaning_system, world_event_system};
use crate::input::Input;

use crate::render::render_system::RenderSystem;
use crate::ui::GameUi;
use world::World;
fn window_conf() -> Conf {
    Conf {
        window_title: "Freefall Tower Defense".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true, // Permite redimensionar la ventana con el ratón
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    //define all the systems
    let mut w = World::default();
    let mut render_system = RenderSystem::new();
    let mut game_ui = GameUi::default();
    game_ui.events.push(ui::UiEvent::OpenShop);
    let mut input = Input::default();

    render_system.init().await;

    loop {
        let dt = get_frame_time().min(0.016); //we get the delta time
        clear_background(BLACK);

        input.poll_events(&mut game_ui, &mut w); //adds the events to the events vector
        w.update_world(&mut input, dt); //update the events and completes the world events
        render_system.update_particles(dt);
        world_event_system::execute_world_events(&mut w, &mut input.cursor_state, &mut game_ui.events, &mut render_system);
        game_ui.execute_ui_events();

        render_system.render(&w, &game_ui, &input); //renders
        cleaning_system::clean_entities(&mut w.level);

        draw_text(format!("{}", get_fps()), 0.0, 100.0, 100.0, DARKBLUE);
        next_frame().await
    }
}
