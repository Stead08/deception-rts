mod camera;
mod input;
mod renderer;

use bevy::prelude::*;
use rts_core::GameState;

#[derive(Resource)]
pub struct SimulationState(pub GameState);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.035, 0.055, 0.085)))
        .insert_resource(SimulationState(GameState::default()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Deception RTS".into(),
                canvas: Some("#game-canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (renderer::setup_scene, mark_ready))
        .add_systems(Update, (camera::zoom_camera, input::pan_camera))
        .run();
}

#[cfg(target_arch = "wasm32")]
fn mark_ready() {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("boot-status") {
            status.set_text_content(Some("作戦システム: ONLINE"));
            status.set_class_name("ready");
        }
        if let Some(body) = document.body() {
            let _ = body.set_attribute("data-bevy-ready", "true");
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn mark_ready() {}
