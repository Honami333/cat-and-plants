use bevy::prelude::*;
use crate::schema::config::ShaderMaterial;


// Ассеты
#[derive(Resource, Debug, Clone)]
pub struct GameAssets { // Изображения
    pub pot_stands: Handle<Image>,
    pub sunlit_nursery: Handle<Image>,
    pub button_buy_tomato: Handle<Image>,
    pub warm_paws_porch: Handle<Image>,
}

#[derive(Resource)]
pub struct ShaderAssets { // Общий список шейдеров
    pub sn_window_light: Handle<ShaderMaterial>,
    pub wpp_window_light: Handle<ShaderMaterial>,
}

#[derive(Resource)]
pub struct AtlasAssets {
    pub tomato_pot_atlas: Handle<Image>,
    pub common_layot: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Debug, Clone)]
pub struct FontAssets {
    pub emoji_font: Handle<Font>,
}




