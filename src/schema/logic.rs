use bevy::prelude::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};
use crate::schema::types_and_states::*;
use crate::schema::config::{Plant, ShaderMaterial};
use crate::content::upgrades::global::*;


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

    pub fn egui_get(&self, res: EGUIResourceType) -> f64 {
        if res == EGUIResourceType::All {
            let mut count = 0.0;
            for (i, count_inv) in self.storage.iter().enumerate() {
                if i != 0 && i != self.storage.len() - 1 {
                    count += count_inv;
                }
            }
            return count;
        }
        self.storage[res as usize]
    }

    pub fn get_egui_all(&self, well: TradeWell, percent: f64) -> f64 {
        let mut all_trade = 0.0;

        for (i, item_count) in self.storage.iter().enumerate() {
            if *item_count > 0.0 && i != 0 && i != self.storage.len() - 1 {
                if let Some(cur_well) = well.well.get(i - 1) {
                    let s = (item_count * percent / 100.0).floor() * cur_well;
                    all_trade += s;
                }
            }
        }

        all_trade
    }

    pub fn add(&mut self, res: usize, amount: f64) {
        self.storage[res] += amount;
    }

    pub fn add_all(&mut self, percent: f64) {
        let mut new_inv = self.storage;

        for (i, count_inv) in new_inv.iter_mut().enumerate() {
            if i != 0 && i != self.storage.len() - 0 && *count_inv > 0.0 {
                *count_inv -= (*count_inv * percent / 100.0).floor();
            }
        }

        self.storage = new_inv;
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


impl Default for UpgradeStorege {
    fn default() -> Self {
        Self {
            global: vec![
                FERTILE_SOIL.clone(),
                GROWTH_SPEED.clone(),
                JOY_BOOST.clone()
            ],
            sunlit_nursery: Vec::new(),
        }
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

impl Default for TradeState {
    fn default() -> Self {
        Self {
            selected_world: EGUICurrntWorld::All,
            selected_item: EGUIResourceType::All,
            selected_percent: 100,
            selected_economy: 0.0,
        }
    }
}

impl Default for UpgradeState {
    fn default() -> Self {
        Self {
            selected_categories: EGUISelectedCategories::Global,
        }
    }
}