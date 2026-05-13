use crate::schema::types_and_states::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use serde::{Serialize, Deserialize};


fn default_static_slice<T>() -> &'static [T] {
    &[]
}

fn default_static_str() -> &'static str {
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plant { // Растение
    pub growth_score: f64,
    pub growth_thereshold: f64,
    pub growth_rate: f64,
    pub gather_amount: f64,
    pub species_id: TypePlant,
    pub slot_uid: usize,
    pub state: PlantStateGrowth,
    pub max_count: usize,

    #[serde(skip, default = "default_static_slice")]
    pub price: &'static [f64],
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
    #[uniform[0]]
    pub color: LinearRgba,
    #[uniform[0]]
    pub scale: f32,
    #[uniform[0]]
    pub shader_type: u32,
    pub original_scale: f32,
    pub mesh_scale: f32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Upgrade {
    #[serde(skip, default = "default_static_str")]
    pub name: &'static str,
    #[serde(skip, default = "default_static_str")]
    pub description: &'static str,

    #[serde(skip, default = "default_static_slice")]
    pub levels: &'static [UpgradeLevel],
    #[serde(skip, default = "default_static_slice")]
    pub dependencies: &'static [UpgradeUID],


    pub id: UpgradeUID,
    pub texture_stage: UpgradeStage,
    pub current_level: usize,
    pub category: EGUISelectedCategories,
    pub grid_pos: (usize, usize),
}

#[derive(Clone, Copy)]
pub struct UpgradeLevel {
    pub resource_types: &'static [ResourceType],
    pub costs: &'static [f64],
    pub value: Option<f64>,
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