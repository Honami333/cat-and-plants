#![windows_subsystem = "windows"]

use bevy::{prelude::*, window::WindowMode};
use systems::SystemPlugin;
use schema::SchemaPlugin;

mod schema;
mod systems;
mod assets;
mod world;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoVsync,
                title: "cat and plants".into(),
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            file_path: "assets".to_string(),
            ..default()
        }))
        .add_plugins(SystemPlugin)
        .add_plugins(SchemaPlugin)

        .run(); 
}


// mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
