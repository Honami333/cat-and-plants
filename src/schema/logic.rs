use crate::content::upgrades::{global::*, sunlit_nursery::*};
use crate::content::world::sunlit_nursery::*;
use crate::schema::config::{Plant, ShaderMaterial, Upgrade};
use crate::schema::resources::AtlasAssets;
use crate::schema::types_and_states::*;
use crate::schema::save_file::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};


impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            fps: MaxFPS {
                limit: true,
                max_fps: 60.0,
                foces_fps: 60.0,
                unfoces_fps: 5.0,
            },
            display: DisplaySettings {
                screen_mode: ScreenMode::Fullscreen,
                resolution: [1920.0, 1080.0],
                max_display: [0.0, 0.0],
            },
            shader: ShaderSettings {
                light_shaders: true,
                dust_particles: true,
                dust_amount: 0.5,
            },
            autosave_interval: 180.0,
        }
    }
}


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
    pub fn get_inv(&self, world: &State<CurrentWorld>) -> Option<&[SlotState; 16]> {
        match world.get() {
            CurrentWorld::SunlitNursery => Some(&self.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_inv_mut(&mut self, world: &State<CurrentWorld>) -> Option<&mut [SlotState; 16]> {
        match world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn add_plant(
        // Добавление предмета в инвентарь
        &mut self,
        current_world: &State<CurrentWorld>,
        new_plant: Plant,
    ) {
        let Some(invetory_array) = self.get_inv_mut(current_world) else {
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
        let Some(invetory_array) = self.get_inv_mut(current_world) else { return; };

        if invetory_array[new_id] == SlotState::Locked { return }

        invetory_array.swap(old_id, new_id);
    }

    pub fn slots_unlocking(
        &mut self,
        economy: &Economy,
        current_world: &State<CurrentWorld>,
        prices: &'static [f64],
    ) -> (bool, Option<usize>) {
        let Some(invetory_array) = self.get_inv_mut(current_world) else {
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
        let Some(invetory_array) = self.get_inv(current_world) else {
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
        let Some(invetory_array) =  self.get_inv(current_world) else {
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

    pub fn get_prestige_res(& self, current_world: &State<CurrentWorld>) -> (Vec<ResourceType>, Vec<f64> ){
        let mut item_res = Vec::new();

        let mut res_vec = Vec::new();

        match current_world.get() {
            CurrentWorld::SunlitNursery => {
                res_vec.push(ResourceType::CatHappiness);
                res_vec.push(ResourceType::Tomatoes);
                res_vec.push(ResourceType::Cucumbers);
                res_vec.push(ResourceType::Corn);
                res_vec.push(ResourceType::Pumpkin);
            },
            CurrentWorld::WarmPawsPorch => (),
        };

        for res in &res_vec {
            item_res.push(self.get_item(*res as usize));
        };

        (res_vec, item_res)
    }

    pub fn add_sparcks(&mut self, res: usize, amount: f64) {
        self.prestige_sparks[res.saturating_sub(self.storage.len())] += amount;
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
        let global_item = [
            (FERTILE_SOIL.grid_pos, FERTILE_SOIL.clone()),
            (GROWTH_SPEED.grid_pos, GROWTH_SPEED.clone()),
            (JOY_BOOST.grid_pos, JOY_BOOST.clone()),
            (CARDBOARD_BOX.grid_pos, CARDBOARD_BOX.clone()),
        ];

        let sunlit_nursery_item = [
            (UNLOCK_CUCUMBER.grid_pos, UNLOCK_CUCUMBER.clone()),
            (UNLOCK_CORN.grid_pos, UNLOCK_CORN.clone()),
            (UNLOCK_PUMPKIN.grid_pos, UNLOCK_PUMPKIN.clone()),
        ];

        Self {
            global: global_item.into_iter().collect(),
            sunlit_nursery: sunlit_nursery_item.into_iter().collect(),
        }
    }
}

impl CurrentWorld {
    pub fn get_prestige_cost(&self) -> Option<&[f64]> {
        match self {
            CurrentWorld::SunlitNursery => Some(SN_FIRST_PRESTIGE_COST.cost),
            CurrentWorld::WarmPawsPorch => None
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
    fn all_upgrages(&self) -> impl Iterator<Item = &Upgrade> {
        self.global.values().chain(self.sunlit_nursery.values())
    }

    pub fn get_global_modifier(&self, upgrade_id: UpgradeUID) -> (Option<f64>, bool) {

         self.all_upgrages()
         .find(|u| u.id == upgrade_id && u.current_level > 0 )
         .map(|u| (u.levels[u.current_level - 1].value, true))
         .unwrap_or((Some(1.0), false))
    }

    pub fn get_storege_category(
        &self,
        category: EGUISelectedCategories,
    ) -> &HashMap<(usize, usize), Upgrade> {
        match category {
            EGUISelectedCategories::Global => &self.global,
            EGUISelectedCategories::SunlitNursery => &self.sunlit_nursery,
        }
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

    pub fn get_dependencies_upgrade(&self) -> Option<UpgradeUID> {
        return match *self {
            TypeButton::CucumberButton => Some(UpgradeUID::UnlockCucumber),
            TypeButton::CornButton => Some(UpgradeUID::UnlockCorn),
            TypeButton::PumpkinButton => Some(UpgradeUID::UnlockPumpkin),
            _ => None,
        };
    }
}

impl TypePlant {
    pub fn get_plant_image(
        &self,
        assets: &AtlasAssets,
    ) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        let image_layout = match self {
            TypePlant::Tomato => assets.tomato_pot_atlas.clone(),
            TypePlant::Cucumber => assets.cucumber_pot_atlas.clone(),
            TypePlant::Corn => assets.corn_pot_atlas.clone(),
            TypePlant::Pumpkin => assets.pumpkin_pot_atlas.clone(),
        };

        (image_layout, assets.common_layout_x128.clone())
    }
}

impl Upgrade {
    pub fn get_dependencies(&self, upgrade_storege: &UpgradeStorege) -> bool {
        self.dependencies.iter().all(|def_ip| {
            let storege = upgrade_storege.get_storege_category(self.category);

            storege
                .values()
                .find(|u| u.id == *def_ip)
                .is_some_and(|u| u.current_level > 0)
        })
    }

    pub fn get_unlocking(&self) -> Option<String> {
        match self.id {
            UpgradeUID::UnlockCucumber => Some("cucumber".to_string()),
            UpgradeUID::UnlockCorn => Some("corn".to_string()),
            UpgradeUID::UnlockPumpkin => Some("pumpkin".to_string()),
            _ => None
        }
    }
}

impl UpgradeStage {
    pub fn next_stage(&mut self, sp: f32) {
        let stage = match sp {
            _  if sp <= 0.0 => UpgradeStage::Locked,
            _  if sp < 0.50 => UpgradeStage::Available,
            _  if sp < 1.0 => UpgradeStage::Growing,
            _  => UpgradeStage::Max,
        };

        *self = stage;
    }
}

impl PrestigeRoom {
    pub fn get_room<'a>(&self, current_world: &State<CurrentWorld>) -> Option<&usize> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&self.sunlit_nursery),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_mut_room<'a>(&mut self, current_world: &State<CurrentWorld>) -> Option<&mut usize> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.sunlit_nursery),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    pub fn get_sparks_res(&self, current_world: &State<CurrentWorld>) -> Option<ResourceType>{
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(ResourceType::SunSparks),
            CurrentWorld::WarmPawsPorch => None,
        }
    }
}