use bevy::prelude::*;
use crate::schema::{resources::*, config::*, types_and_states::*};


// Загруста ассетов
pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let assets = GameAssets {
        pot_stands: asset_server.load("pot_stands.png"),
        button_buy_tomato: asset_server.load("button_buy_tomato.png"),
        sunlit_nursery: asset_server.load("world/sunlit_nursery.png"),
        warm_paws_porch: asset_server.load("world/warm_paws_porch.png"),
    };
    commands.insert_resource(assets);
}


//Загрузка шейдеров
pub fn load_shaders(
    mut commands: Commands,
    mut materials: ResMut<Assets<ShaderMaterial>>
) {
    let shaders = ShaderAssets {
        sn_window_light: materials.add(ShaderMaterial {
            color: LinearRgba::new(1.0, 0.6, 0.5, 0.2),
            scale: 0.004,
            original_scale: 0.004,
            mash_scale: 650.0,
            shader_type: ShaderType::SNWindowLight as u32,
            }),
        wpp_window_light: materials.add(ShaderMaterial {
            color: LinearRgba::new(1.0, 0.6, 0.5, 0.2),
            scale: 0.004,
            original_scale: 0.004,
            mash_scale: 650.0,
            shader_type: ShaderType::WPPWindowLight as u32,
            }),
    };
    commands.insert_resource(shaders);
}


//Загрузка атласов
pub fn load_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts : ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout_4x1 = layouts.add(TextureAtlasLayout::from_grid(UVec2::new(128, 128), 4, 1, None, None));

    let atlas = AtlasAssets {
        tomato_pot_atlas: asset_server.load("tomato_pot_atlas.png"),
        common_layot: layout_4x1,
    };
    commands.insert_resource(atlas);
}


pub fn load_font(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = FontAssets {
        emoji_font: asset_server.load("fonts/segoe-ui-emoji_0.ttf")
    };
    commands.insert_resource(font);
}