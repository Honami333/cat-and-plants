use crate::schema::{config::*, resources::*};
use bevy::prelude::*;

// Загруста ассетов
pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        pot_stands: asset_server.load("slots/pot_stands.png"),

        sunlit_nursery: asset_server.load("world/sunlit_nursery.png"),
        warm_paws_porch: asset_server.load("world/warm_paws_porch.png"),
        dark_storage: asset_server.load("world/dark_storage.png"),

        button_buy_tomato: asset_server.load("button/button_buy_tomato.png"),
        button_buy_cucumber: asset_server.load("button/button_buy_cucumber.png"),
        button_buy_corn: asset_server.load("button/button_buy_corn.png"),
        button_buy_pumpkin: asset_server.load("button/button_buy_pumpkin.png"),
        button_slots_unlocking: asset_server.load("button/button_slots_unlocking.png"),

        plant_stand_rack: asset_server.load("plant_stand_rack.png"),
    };
    commands.insert_resource(assets);
}

//Загрузка шейдеров
pub fn load_shaders(mut commands: Commands, mut materials: ResMut<Assets<LightShaderMaterial>>) {
    let shaders = ShaderAssets {
        sn_window_light: materials.add(LightShaderMaterial {
            color: LinearRgba::new(1.0, 0.6, 0.5, 0.2),
            scale: 0.004,
            original_scale: 0.004,
            mesh_scale: 650.0,
            shader_type: LightShaderType::SNWindow as u32,
            light_shaders: 1,
            dust_particles: 1,
            dust_amount: 1.0,
        }),
        wpp_window_light: materials.add(LightShaderMaterial {
            color: LinearRgba::new(1.0, 0.6, 0.5, 0.2),
            scale: 0.004,
            original_scale: 0.004,
            mesh_scale: 650.0,
            shader_type: LightShaderType::WPPWindow as u32,
            light_shaders: 1,
            dust_particles: 1,
            dust_amount: 1.0,
        }),
        ds_light: materials.add(LightShaderMaterial {
            color: LinearRgba::new(1.0, 0.6, 0.5, 0.2),
            scale: 0.004,
            original_scale: 0.004,
            mesh_scale: 650.0,
            shader_type: LightShaderType::DSMenu as u32,
            light_shaders: 1,
            dust_particles: 1,
            dust_amount: 1.0,
        }),
    };
    commands.insert_resource(shaders);
}

//Загрузка атласов
pub fn load_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout_4x1x128 = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 128),
        4,
        1,
        None,
        None,
    ));

    let layout_4x1x64 = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        4,
        1,
        None,
        None,
    ));

    let layout_2x1x100x240 = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(100, 240),
        2,
        1,
        None,
        None,
    ));

    let atlas = AtlasAssets {
        pockets_of_improvements: asset_server.load("pockets_of_improvements.png"),
        save_slots_atlas: asset_server.load("save_slots_atlas.png"),

        tomato_pot_atlas: asset_server.load("plant/tomato_pot_atlas.png"),
        cucumber_pot_atlas: asset_server.load("plant/cucumber_pot_atlas.png"),
        corn_pot_atlas: asset_server.load("plant/corn_pot_atlas.png"),
        pumpkin_pot_atlas: asset_server.load("plant/pumpkin_pot_atlas.png"),

        common_layout_x64: layout_4x1x64,
        common_layout_x128: layout_4x1x128,
        common_layout_x100_240: layout_2x1x100x240,
    };
    commands.insert_resource(atlas);
}

pub fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = FontAssets {
        emoji_font: asset_server.load("fonts/segoe-ui-emoji_0.ttf"),
    };
    commands.insert_resource(font);
}
