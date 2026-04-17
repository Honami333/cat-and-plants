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
                title: "cat and plants".into(),
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest()))
        .add_plugins(SystemPlugin)
        .add_plugins(SchemaPlugin)

        .run(); 
}


// mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
