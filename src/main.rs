// #![windows_subsystem = "windows"]

use bevy::{prelude::*, window::WindowMode};
use schema::SchemaPlugin;
use systems::SystemPlugin;
mod assets;
mod content;
mod schema;
mod systems;

pub const GAME_VERSION: &'static str = "0.6.3 pre-release";


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
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


