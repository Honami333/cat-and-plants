use crate::assets::{load_assets, load_atlas, load_font, load_shaders};
use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub mod common;
pub mod config;
pub mod economy_inventory;
pub mod global_inventory;
pub mod global_settings;
pub mod hud;
pub mod item_type_info;
pub mod prestige;
pub mod resources;
pub mod save_file;
pub mod upgrade_storege;
pub mod world_components;

pub struct SchemaPlugin;

impl Plugin for SchemaPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<common::GameState>();
        app.init_state::<common::CurrentWorld>();

        app.init_resource::<global_inventory::GlobalInventory>()
            .init_resource::<economy_inventory::Economy>()
            .init_resource::<item_type_info::ItemTypeInfo>()
            .init_resource::<upgrade_storege::UpgradeStorege>()
            .init_resource::<save_file::SaveSlotInv>()
            .init_resource::<global_settings::GlobalSettings>()
            .init_resource::<prestige::PrestigeRoom>()
            .init_resource::<global_inventory::DragItem>()
            .init_resource::<common::WorldScale>()
            .init_resource::<hud::FeedState>()
            .init_resource::<upgrade_storege::UpgradeState>()
            .init_resource::<hud::VisualCounter>()
            .init_resource::<hud::MenuCurPage>()
            .init_resource::<hud::TradeState>();

        app.add_plugins(Material2dPlugin::<config::LightShaderMaterial>::default());
        app.add_plugins(Material2dPlugin::<config::BreezeShaderMaterial>::default());

        app.add_systems(
            OnEnter(common::GameState::Loading),
            (load_atlas,
                load_shaders,
                load_assets,
                load_font, 
            ),
        );
    }
}

