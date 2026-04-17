use bevy::{prelude::*, sprite_render::Material2dPlugin};
use crate::assets::{load_atlas, load_shaders, load_assets, load_font};
use crate::schema::resources::{GameAssets, ShaderAssets, AtlasAssets, FontAssets};
use crate::schema::types_and_states::GameState;

pub mod config;
pub mod logic;
pub mod resources;
pub mod types_and_states;
pub mod world_components;


pub struct SchemaPlugin;
impl Plugin for SchemaPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<types_and_states::GameState>();

        app.init_resource::<types_and_states::CurrentWorld>()
            .init_resource::<types_and_states::GlobalInventory>()
            .init_resource::<types_and_states::DragItem>()
            .init_resource::<types_and_states::Economy>();


        app.add_plugins(Material2dPlugin::<config::ShaderMaterial>::default());
        
        app.add_systems(OnEnter(types_and_states::GameState::Loading), (load_atlas, load_shaders, load_assets, load_font));
        
        app.add_systems(Update, (check_assts_ready).run_if(in_state(types_and_states::GameState::Loading)));
    }
}

pub fn check_assts_ready(
    mut next_state: ResMut<NextState<GameState>>,
    assets: Option<Res<GameAssets>>,
    shaders: Option<Res<ShaderAssets>>,
    atlas: Option<Res<AtlasAssets>>, 
    font: Option<Res<FontAssets>>,
) {
    if assets.is_some() && shaders.is_some() && atlas.is_some() && font.is_some() { 
        next_state.set(types_and_states::GameState::Playing)
    }
}















