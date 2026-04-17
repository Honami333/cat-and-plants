use bevy::prelude::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};
use crate::world::{SUNLIT_NURSERY_SLOT_CFG, SUNLIT_NURSERY_BUTTON_CFG, SUNLIT_NURSERY_DATA};
use crate::schema::types_and_states::{CurrentWorld, Economy, GlobalInventory, PlantStateGrowth, PlantStateUpdate, ResourceType, SlotState};
use crate::schema::config::{ButtonCFG, Plant, ScaleBackground, ShaderMaterial, WorldSettingsSlot};
use crate::schema::resources::{GameAssets, ShaderAssets};


// Логика
impl Default for GlobalInventory { // Все Инвентари по умолчанию
    fn default() -> Self {
        let mut slots = [SlotState::Locked; 16];
        
        for i in 0..4 {
            slots[i] = SlotState::Empty;
        }

        Self {
            sunlit_nursery_inv: slots
        }
    }
}

impl GlobalInventory { 
    pub fn add_plant( // Добавление предмета в инвентарь
        &mut self,
        loc: CurrentWorld,
        new_plant: Plant
    ) {
        let invetory_array = match loc {
            CurrentWorld::SunlitNursery => &mut self.sunlit_nursery_inv,
        };
        
        for  slot in invetory_array.iter_mut() {
        if *slot == SlotState::Empty {
            *slot = SlotState::Occupied(new_plant);
            break;
        }}
    }

    pub fn move_plant( // Перемещение предмета в инвентаре
        &mut self,
        loc: CurrentWorld,
        old_id: usize,
        new_id: usize,
    ) {
        let invetory_array = match loc {
            CurrentWorld::SunlitNursery => &mut self.sunlit_nursery_inv,
        };
        if invetory_array[new_id] == SlotState::Locked {
        }

        if matches!(invetory_array[new_id], SlotState::Occupied(_) | SlotState::Empty) {
            invetory_array.swap(old_id, new_id);
        }
    }
}

impl Economy {
    pub fn get(&self, res: ResourceType) -> f64 {
        self.storage[res as usize]
    }

    pub fn add(&mut self, res: usize, amount: f64) {
        self.storage[res] += amount;
    }
}

impl Material2d for ShaderMaterial { // Настройки шейдеров
    fn fragment_shader() -> ShaderRef {
        "shaders/window_light.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl CurrentWorld { // Поставщик настроек миров
    pub fn get_config(
        &self,
        assets: &Res<GameAssets>,
        shaders: &Res<ShaderAssets>,
    ) -> (
        &ScaleBackground,
        &WorldSettingsSlot,
        &ButtonCFG,
        Handle<Image>,
        Handle<Image>,
        Handle<ShaderMaterial>,
    ) {
        match self {
            CurrentWorld::SunlitNursery => (
            &SUNLIT_NURSERY_DATA,
            &SUNLIT_NURSERY_SLOT_CFG,
            &SUNLIT_NURSERY_BUTTON_CFG,

            assets.sunlit_nursery.clone(),
            assets.pot_stands.clone(),
            shaders.window_light.clone(),
            ),
        }
    }
}

impl CurrentWorld {
    pub fn get_inv<'a>(
        &self,
        inv: &'a Res<GlobalInventory>,
    ) -> &'a [SlotState; 16] {
        match self {
            CurrentWorld::SunlitNursery => &inv.sunlit_nursery_inv,
        }
    }

    pub fn get_inv_mut<'a>(
        &self,
        inv: &'a mut ResMut<GlobalInventory>,
    ) -> &'a mut [SlotState; 16] {
        match self {
            CurrentWorld::SunlitNursery => &mut inv.sunlit_nursery_inv,
        }
    }
}

impl PlantStateGrowth {
    pub fn check_state(&self) -> PlantStateUpdate {
        match self {
            Self::Seed(s) | Self::Sprout(s) | Self::Sapling(s) | Self::Mature(s) => *s
        }
    }

    pub fn atlas_texture_id(&self) -> u32 {
        match self {
            Self::Seed(_) => 0,
            Self::Sprout(_) => 1,
            Self::Sapling(_) => 2,
            Self::Mature(_) => 3,
        }
    }

    pub fn next_state(&self) -> PlantStateGrowth {
        match self {
            Self::Seed(_) => Self::Sprout(PlantStateUpdate::Idle),
            Self::Sprout(_) => Self::Sapling(PlantStateUpdate::Idle),
            Self::Sapling(_) => Self::Mature(PlantStateUpdate::Idle),
            Self::Mature(_) => Self::Mature(PlantStateUpdate::Idle),
        }
    }
}