use crate::schema::config::ShaderMaterial;
use bevy::prelude::*;

// Ассеты
#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    // Изображения
    pub pot_stands: Handle<Image>,

    pub sunlit_nursery: Handle<Image>,
    pub warm_paws_porch: Handle<Image>,

    pub button_buy_tomato: Handle<Image>,
    pub button_buy_cucumber: Handle<Image>,
    pub button_buy_corn: Handle<Image>,
    pub button_buy_pumpkin: Handle<Image>,
    pub button_slots_unlocking: Handle<Image>,
}

#[derive(Resource)]
pub struct ShaderAssets {
    // Общий список шейдеров
    pub sn_window_light: Handle<ShaderMaterial>,
    pub wpp_window_light: Handle<ShaderMaterial>,
}

#[derive(Resource)]
pub struct AtlasAssets {
    pub pockets_of_improvements: Handle<Image>,
    pub save_slots_atlas: Handle<Image>,

    pub tomato_pot_atlas: Handle<Image>,
    pub cucumber_pot_atlas: Handle<Image>,
    pub corn_pot_atlas: Handle<Image>,
    pub pumpkin_pot_atlas: Handle<Image>,

    pub common_layout_x128: Handle<TextureAtlasLayout>,
    pub common_layout_x100_240: Handle<TextureAtlasLayout>,
    pub common_layout_x40: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Debug, Clone)]
pub struct FontAssets {
    pub emoji_font: Handle<Font>,
}
