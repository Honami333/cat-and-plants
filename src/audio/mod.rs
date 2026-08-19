use bevy::prelude::*;

mod f;


pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<f::AuidioSystem>();
    }
}

