//#![windows_subsystem = "windows"]

// use bevy::{prelude::*, window::WindowMode};
use bevy::prelude::*;
use schema::SchemaPlugin;
use systems::SystemPlugin;

mod assets;
mod content;
mod schema;
mod systems;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        title: "cat and plants".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SystemPlugin)
        .add_plugins(SchemaPlugin)
        .run();
}

// mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
