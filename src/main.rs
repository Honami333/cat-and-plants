// #![windows_subsystem = "windows"]

use bevy::{prelude::*, window::WindowMode};
use schema::SchemaPlugin;
use systems::SystemPlugin;
use cat_ai::CatAIPlugin;

mod assets;
mod content;
mod schema;
mod systems;
mod cat_ai;

pub const GAME_VERSION: &str = "0.6.5 pre-release";


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
                        title: "Cat and Plants".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SystemPlugin)
        .add_plugins(SchemaPlugin)
        .add_plugins(CatAIPlugin)
        .run();
}


