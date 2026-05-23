use crate::schema::types_and_states::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use serde::{Serialize, Deserialize};


pub fn default_static_slice<T>() -> &'static [T] {
    &[]
}

pub fn default_static_str() -> &'static str {
    ""
}

// Конфиги
#[derive(Component, Clone, Copy)]
pub struct ScaleBackground { // Мир
    pub wh: Vec2,
}

#[derive(Clone)]
pub struct ButtonCFG { // Кнопка
    pub pos: Vec2,
    pub text: &'static str,
    pub b_type: TypeButton,
    pub text_pos: Vec2,
}

#[derive(Component, Default)]
pub struct WorldSettingsSlot { // Слот инвенторя
    pub slot_start_pos: Vec2,
    pub step_x: Vec2,
    pub step_y: Vec2,
    pub slot_grid_scale: u8,
}



pub struct PlantResource {
    pub plant0: ResourceType,
    pub plant1: ResourceType,
    pub plant2: ResourceType,
    pub plant3: ResourceType,
    pub plant_icon0: &'static str,
    pub plant_icon1: &'static str,
    pub plant_icon2: &'static str,
    pub plant_icon3: &'static str,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ShaderMaterial { // Конфиг шейдеров
    #[uniform(0)] pub color: LinearRgba,
    #[uniform(0)] pub scale: f32,
    #[uniform(0)] pub shader_type: u32,

    #[uniform(0)] pub light_shaders: u32,
    #[uniform(0)] pub dust_particles: u32,
    #[uniform(0)] pub dust_amount: f32,

    pub original_scale: f32,
    pub mesh_scale: f32,
}



pub struct SlotPrices {
    pub prices: &'static [f64],
}

#[derive(Clone, Copy, Default)]
pub struct SaveSlot {
    pub stage: SlotTextureState,
    pub click: usize,
    pub last_data_text: &'static str,
}

pub struct PrestigeCost {
    pub cost: &'static [(ResourceType, f64)],
}