use crate::schema::config::LightShaderMaterial;
use bevy::prelude::*;

// Ассеты
#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    // Изображения
    pub pot_stands: Handle<Image>,

    pub sunlit_nursery: Handle<Image>,
    pub warm_paws_porch: Handle<Image>,
    pub dark_storage: Handle<Image>,
    pub shadow_greenhouse_base: Handle<Image>,
    pub shadow_greenhouse_shelf: Handle<Image>,
    pub shadow_greenhouse_photos: Handle<Image>,

    pub button_buy_tomato: Handle<Image>,
    pub button_buy_cucumber: Handle<Image>,
    pub button_buy_corn: Handle<Image>,
    pub button_buy_pumpkin: Handle<Image>,
    pub button_slots_unlocking: Handle<Image>,

    pub plant_stand_rack: Handle<Image>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum LightShaderType {
    SNWindow = 0,
    WPPWindow = 1,
    DSMenu = 2,
}

#[derive(Resource)]
pub struct ShaderAssets { // Общий список шейдеров
    pub sn_window_light: Handle<LightShaderMaterial>,
    pub wpp_window_light: Handle<LightShaderMaterial>,
    pub ds_light: Handle<LightShaderMaterial>,
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
    pub common_layout_x64: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Debug, Clone)]
pub struct FontAssets {
    pub emoji_font: Handle<Font>,
}
