use crate::schema::{config::{SaveSlot, Upgrade}, types_and_states::*};
use bevy::{math::f64, platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};
use crate::systems::locales::Language;


#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SaveDataContainer {
    pub up_storege: UpgradeStorege,
    pub global_storege: GlobalInventory,
    pub eco_storege: Economy,
    pub cit_storege: ItemTypeInfo,
    pub world: CurrentWorld,
    pub prestige: PrestigeRoom,
}

// Список инвенторей





#[derive(Resource, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct ItemTypeInfo {
    pub sunlit_nursery_inv: [usize; 4],
    pub sn_plant_ability: HashMap<TypePlant, HashMap<PlantAbility, [usize; 2]>>,
}

#[derive(Resource, Clone, Copy, Default)]
pub struct SaveSlotInv {
    pub slot: [SaveSlot; 3],
    pub active_slot: Option<usize>,
    pub deleting_slot: Option<usize>,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalSettings {
    pub fps: MaxFPS,
    pub display: DisplaySettings,
    pub shader: ShaderSettings,
    pub autosave_interval: f64,
    pub language: Language,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub screen_mode: ScreenMode,
    pub resolution: [f32; 2],
    pub max_display: [f32; 2],
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct ShaderSettings {
    pub light_shaders: bool,
    pub dust_particles: bool,
    pub dust_amount: f32,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct MaxFPS {
    pub limit: bool,
    pub max_fps: f64,
    pub foces_fps: f64,
    pub unfoces_fps: f64,
}

#[derive(Resource, Clone, Copy, Default,  Serialize, Deserialize, Debug)]
pub struct PrestigeRoom {
    pub sunlit_nursery: usize,
}