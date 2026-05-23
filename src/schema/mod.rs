use crate::assets::{load_assets, load_atlas, load_font, load_shaders};
use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub mod global_inventory;
pub mod economy_inventory;
pub mod config;
pub mod logic;
pub mod resources;
pub mod save_file;
pub mod types_and_states;
pub mod upgrade_storege;
pub mod world_components;

pub struct SchemaPlugin;

impl Plugin for SchemaPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<types_and_states::GameState>();
        app.init_state::<types_and_states::CurrentWorld>();

        app.init_resource::<global_inventory::GlobalInventory>()
            .init_resource::<economy_inventory::Economy>()
            .init_resource::<save_file::ItemTypeInfo>()
            .init_resource::<upgrade_storege::UpgradeStorege>()
            .init_resource::<save_file::SaveSlotInv>()
            .init_resource::<save_file::GlobalSettings>()
            .init_resource::<save_file::PrestigeRoom>()
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
    }
}

