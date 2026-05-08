use crate::content::upgrades::global::*;
use crate::content::world::sunlit_nursery::*;
use crate::schema::config::{Plant, ShaderMaterial};
use crate::schema::resources::AtlasAssets;
use crate::schema::types_and_states::*;
use bevy::prelude::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

// Логика
impl Default for GlobalInventory {
    // Все Инвентари по умолчанию
    fn default() -> Self {
        let mut slots = [SlotState::Locked; 16];

        for i in 0..4 {
            slots[i] = SlotState::Empty;
        }

        Self {
            sunlit_nursery_inv: slots,
        }
    }
}

impl GlobalInventory {
    pub fn add_plant(
        // Добавление предмета в инвентарь
        &mut self,
        current_world: &State<CurrentWorld>,
        new_plant: Plant,
    ) {
        let Some(invetory_array) = current_world.get_inv_mut(self) else {
            return;
        };

        for slot in invetory_array.iter_mut() {
            if *slot == SlotState::Empty {
                *slot = SlotState::Occupied(new_plant);
                break;
            }
        }
    }

    pub fn move_plant(
        // Перемещение предмета в инвентаре
        &mut self,
        current_world: &State<CurrentWorld>,
        old_id: usize,
        new_id: usize,
    ) {
        let Some(invetory_array) = current_world.get_inv_mut(self) else {
            return;
        };

        if invetory_array[new_id] == SlotState::Locked {}

        if matches!(
            invetory_array[new_id],
            SlotState::Occupied(_) | SlotState::Empty
        ) {
            invetory_array.swap(old_id, new_id);
        }
    }

    pub fn slots_unlocking(
        &mut self,
        economy: &Economy,
        current_world: &State<CurrentWorld>,
        prices: &'static [f64],
    ) -> (bool, Option<usize>) {
        let Some(invetory_array) = current_world.get_inv_mut(self) else {
            return (false, None);
        };

        if let Some((i, _)) = invetory_array
            .iter()
            .enumerate()
            .find(|(_, i)| matches!(i, SlotState::Locked))
        {
            if economy.get_item(ResourceType::CatHappiness as usize) < prices[i - 4] {
                return (false, None);
            };

            invetory_array[i] = SlotState::Empty;
            return (true, Some(i - 4));
        };

        (false, None)
    }

    pub fn get_slots_unlocking(&self, current_world: &State<CurrentWorld>) -> Option<usize> {
        let Some(invetory_array) = current_world.get_inv(self) else {
            return None;
        };

        if let Some((i, _)) = invetory_array
            .iter()
            .enumerate()
            .find(|(_, i)| matches!(i, SlotState::Locked))
        {
            return Some(i - 4);
        }

        return None;
    }

    pub fn get_slots_empty(&self, current_world: &State<CurrentWorld>) -> bool {
        let Some(invetory_array) = current_world.get_inv(self) else {
            return false;
        };

        if let Some(_) = invetory_array
            .iter()
            .find(|i| matches!(i, SlotState::Empty))
        {
            return true;
        }

        return false;
    }
}

impl Economy {
    pub fn get_item(&self, res: usize) -> f64 {
        self.storage[res]
    }

    pub fn egui_get_item(&self, res: EGUIResourceType) -> f64 {
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

    pub fn egui_get_item_all(&self, well: TradeWell, percent: f64) -> f64 {
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

impl Material2d for ShaderMaterial {
    // Настройки шейдеров
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
                JOY_BOOST.clone(),
            ],
            sunlit_nursery: Vec::new(),
        }
    }
}

impl CurrentWorld {
    pub fn get_inv(self, inv: &GlobalInventory) -> Option<&[SlotState; 16]> {
        match self {
            CurrentWorld::SunlitNursery => Some(&inv.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_inv_mut(self, inv: &mut GlobalInventory) -> Option<&mut [SlotState; 16]> {
        match self {
            CurrentWorld::SunlitNursery => Some(&mut inv.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }
}

impl PlantStateGrowth {
    pub fn atlas_texture_id(&self) -> u32 {
        match self {
            Self::Seed => 0,
            Self::Sprout => 1,
            Self::Sapling => 2,
            Self::Mature => 3,
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

impl UpgradeStorege {
    pub fn get_global_modifier(&self, upgrade_id: UpgradeUID) -> f64 {
        for upgrade in self.global.iter() {
            if upgrade.id == upgrade_id && upgrade.current_level > 0 {
                return upgrade.levels[upgrade.current_level - 1].value;
            };
        }

        1.0
    }
}

impl CountItemType {
    pub fn get_inv<'a>(&self, current_world: &State<CurrentWorld>) -> Option<&[usize; 4]> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&self.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_inv_mut<'a>(
        &mut self,
        current_world: &State<CurrentWorld>,
    ) -> Option<&mut [usize; 4]> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn add(&mut self, res: usize, current_world: &State<CurrentWorld>) {
        let Some(count_inv) = self.get_inv_mut(current_world) else {
            return;
        };
        count_inv[res] += 1;
    }
}

impl TypeButton {
    pub fn get_plant_cfg(&self) -> Option<Plant> {
        return match *self {
            TypeButton::TomatoButton => Some(PL_TOMATO),
            TypeButton::CucumberButton => Some(PL_CUCUMBER),
            TypeButton::CornButton => Some(PL_CORN),
            TypeButton::PumpkinButton => Some(PL_PUMPKIN),
            TypeButton::SlotsUnLocking => None,
        };
    }
}

impl TypePlant {
    pub fn get_plant_image(&self, assets: &AtlasAssets) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        return match self {
            TypePlant::Tomato => (
                assets.tomato_pot_atlas.clone(),
                assets.common_layout.clone(),
            ),
            TypePlant::Cucumber => (
                assets.cucumber_pot_atlas.clone(),
                assets.common_layout.clone(),
            ),
            TypePlant::Corn => (
                assets.corn_pot_atlas.clone(),
                assets.common_layout.clone(),
            ),
            TypePlant::Pumpkin => (
                assets.pumpkin_pot_atlas.clone(),
                assets.common_layout.clone(),
            ),
        };
    }
}
