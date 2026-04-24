use bevy::prelude::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};
use crate::schema::types_and_states::{CurrentWorld, Economy, GlobalInventory, PlantStateGrowth, PlantStateUpdate, ResourceType, SlotState};
use crate::schema::config::{Plant, ShaderMaterial};


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
            CurrentWorld::WarmPawsPorch => return,
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
            CurrentWorld::WarmPawsPorch => return,
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
        "shaders/combined_window.wgsl".into()
    } 

    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        AlphaMode2d::Blend
    }
}


impl CurrentWorld {
    pub fn get_inv<'a>(
        &self,
        inv: &'a Res<GlobalInventory>,
    ) -> Option<&'a [SlotState; 16]> {
        match self {
            CurrentWorld::SunlitNursery => Some(&inv.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_inv_mut<'a>(
        &self,
        inv: &'a mut ResMut<GlobalInventory>,
    ) -> Option<&'a mut [SlotState; 16]> {
        match self {
            CurrentWorld::SunlitNursery => Some(&mut inv.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
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
