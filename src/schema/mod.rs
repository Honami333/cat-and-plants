use crate::assets::{load_assets, load_atlas, load_font, load_shaders};
use crate::schema::resources::{AtlasAssets, FontAssets, GameAssets, ShaderAssets};
use crate::schema::types_and_states::GameState;
use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub mod config;
pub mod logic;
pub mod resources;
pub mod save_file;
pub mod types_and_states;
pub mod world_components;

pub struct SchemaPlugin;

impl Plugin for SchemaPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<types_and_states::GameState>();
        app.init_state::<types_and_states::CurrentWorld>();

        app.init_resource::<save_file::GlobalInventory>()
            .init_resource::<save_file::Economy>()
            .init_resource::<save_file::CountItemType>()
            .init_resource::<save_file::UpgradeStorege>()
            .init_resource::<save_file::SaveSlotInv>()
            .init_resource::<save_file::GlobalSettings>()
            .init_resource::<types_and_states::DragItem>()
            .init_resource::<types_and_states::WorldScale>()
            .init_resource::<types_and_states::TradeState>()
            .init_resource::<types_and_states::UpgradeState>()
            .init_resource::<types_and_states::MenuCurPage>();

        app.add_plugins(Material2dPlugin::<config::ShaderMaterial>::default());

        app.add_systems(
            OnEnter(types_and_states::GameState::Loading),
            (load_atlas,
                load_shaders,
                load_assets,
                load_font, 
            ),
        );

        app.add_systems(
            Update,
            (check_assets_ready).run_if(in_state(types_and_states::GameState::Loading)),
        );
    }
}

pub fn check_assets_ready(
    mut next_state: ResMut<NextState<GameState>>,
    assets: Option<Res<GameAssets>>,
    shaders: Option<Res<ShaderAssets>>,
    atlas: Option<Res<AtlasAssets>>,
    font: Option<Res<FontAssets>>,
) {
    if assets.is_some() && shaders.is_some() && atlas.is_some() && font.is_some() {
        next_state.set(types_and_states::GameState::Menu)
    }
}
