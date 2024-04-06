use std::default::Default;

use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::petgraph::visit::Walker;
use crate::camera::MousePressed;

mod player;
mod setup;
mod camera;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title: "aqua-waifu".to_string(),
            present_mode: bevy::window::PresentMode::Immediate,
            resizable: true,
            cursor: bevy::window::Cursor::default(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: bevy::window::WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
            name: None,
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    }
    app.insert_resource(MousePressed(false));
    app.add_systems(Startup, setup::setup);
    app.add_systems(Update, camera::camera_movement);
    app.add_systems(Update, camera::handle_mouse);
    app.run();
}