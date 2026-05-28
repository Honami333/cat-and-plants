use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};
use super::economy_inventory::ResourceType;


// Конфиги
#[derive(Component, Clone, Copy)]
pub struct Background;

#[derive(Component, Default)]
pub struct WorldSettingsSlot { // Слот инвенторя
    pub slot_start_pos: Vec2,
    pub step_x: Vec2,
    pub step_y: Vec2,
    pub slot_grid_scale: u8,
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

impl Material2d for ShaderMaterial {
    // Настройки шейдеров
    fn fragment_shader() -> ShaderRef {
        "shaders/combined_window.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub struct SlotPrices {
    pub prices: &'static [f64],
}



pub struct PrestigeCost {
    pub cost: &'static [(ResourceType, f64)],
}