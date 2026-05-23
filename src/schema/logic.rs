use crate::content::upgrades::{prestige::*, global::*, sunlit_nursery::*};
use crate::content::world::sunlit_nursery::*;
use crate::schema::config::{Plant, ShaderMaterial, Upgrade};
use crate::schema::resources::AtlasAssets;
use crate::schema::types_and_states::*;
use crate::schema::save_file::*;
use crate::systems::locales::Language;
use bevy::ecs::world;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::shader::ShaderRef;
use std::borrow::Borrow;
use bevy::sprite_render::{AlphaMode2d, Material2d};


pub trait MapStore<T> {
    fn get_for_world (&self, world: &State<CurrentWorld>) -> Option<&T>;
    fn get_for_world_mut (&mut self, world: &State<CurrentWorld>) -> Option<&mut T>;
}


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
            language: Language::En,
        }
    }
}




impl Default for ItemTypeInfo {
    fn default() -> Self {
        let sn_plant_ability_map = HashMap::from([
            (
                TypePlant::Tomato, 
                HashMap::from([
                    (PlantAbility::TomatoClickCombo, [0_usize, 1_usize])
                ])
            ),
            (
                TypePlant::Corn, 
                HashMap::from([
                    (PlantAbility::CornBoomHarvet, [0_usize, 1_usize])
                ])
            ),
        ]);

        Self {
            sunlit_nursery_inv: [0; 4],
            sn_plant_ability: sn_plant_ability_map,
        }
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


impl CurrentWorld {
    pub fn get_prestige_cost(&self) -> Option<&[(ResourceType, f64)]> {
        match self {
            CurrentWorld::SunlitNursery => Some(SN_FIRST_PRESTIGE_COST.cost),
            CurrentWorld::WarmPawsPorch => None
        }
    }

    pub fn get_cost(&self, res: ResourceType, pr_room: usize) -> Option<f64> {
        let Some(res_cost) = self.get_prestige_cost() else { return None; };

        let Some((_, cost)) = res_cost.iter().find(|(r, _)| *r == res) else { return None; };

        if matches!(res, ResourceType::CatHappiness) {
            Some(*cost * (pr_room as f64 + 1.0).powf(1.8))
        } else {
            Some(*cost * (pr_room as f64 + 1.0).powf(1.2))
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


impl ItemTypeInfo {
    pub fn get_inv<'a>(
        &self,
        current_world: &State<CurrentWorld>,
    ) -> Option<&[usize; 4]> {
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

    pub fn add_to_plant_ability(
        &mut self,
        type_plant: &TypePlant,
        plant_ability_type: PlantAbility,
        new_value: usize,
        action: ModifierOperation,
    ) {
        let Some(plant_ability) = self.sn_plant_ability.get_mut(type_plant) else { return; };

        let Some(ability_case) = plant_ability.get_mut(&plant_ability_type) else { return; };

        let Some(ability_count) = ability_case.get_mut(0) else { return; };

        match action {
            ModifierOperation::Set => *ability_count = new_value,
            ModifierOperation::Add => *ability_count += new_value,
        }
    }

    pub fn get_value_plant_ability(
        &self,
        type_plant: &TypePlant,
        plant_ability_type: PlantAbility,
    ) -> f64 {
        let Some(plant_ability) = self.sn_plant_ability.get(type_plant) else { return 0.0; };

        let Some(ability_case) = plant_ability.get(&plant_ability_type) else { return 0.0; };

        let Some(ability_count) = ability_case.get(0) else { return 0.0; };

        let Some(ability_marker) = ability_case.get(1) else { return 0.0; };

        if *ability_marker == 0 { return  0.0; };

        *ability_count as f64 / *ability_marker as f64
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


impl EGUIResourceType {
    pub fn into_plant(&self) -> Vec<TypePlant> {
        match self {
            EGUIResourceType::All => vec![TypePlant::Tomato, TypePlant::Cucumber, TypePlant::Corn, TypePlant::Pumpkin],
            EGUIResourceType::Tomatoes => vec![TypePlant::Tomato],
            EGUIResourceType::Cucumbers => vec![TypePlant::Cucumber],
            EGUIResourceType::Corn =>vec![TypePlant::Corn],
            EGUIResourceType::Pumpkin => vec![TypePlant::Pumpkin],
            EGUIResourceType::None => Vec::new(),
        }
    }
}