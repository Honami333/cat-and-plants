// #![windows_subsystem = "windows"]

use bevy::{prelude::*, window::WindowMode};
use audio::AudioPlugin;
use schema::SchemaPlugin;
use systems::SystemPlugin;
use cat_ai::CatAIPlugin;

mod audio;
mod assets;
mod content;
mod schema;
mod systems;
mod cat_ai;

pub const GAME_VERSION: &str = "0.7.0 pre-release";

const TITLE_NAME: &str = "Cat and Plants";


fn main() {
    let mut app = App::new();

    add_plugins(&mut app);

    app.run();
}

fn add_plugins(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resizable: false,
                    title: TITLE_NAME.into(),
                    ..default()
                }),
                ..default()
            })

            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins(AudioPlugin);
    app.add_plugins(SystemPlugin);
    app.add_plugins(SchemaPlugin);
    app.add_plugins(CatAIPlugin);
}
