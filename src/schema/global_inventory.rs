use crate::schema::economy_inventory::Economy;

use super::common::MapStore;
use super::save_file::default_static_slice;
use super::resources::*;
use super::common::CurrentWorld;
use super::economy_inventory::ResourceType;
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SlotState {
    // Состояние слота
    Locked,
    Empty,
    Occupied(Plant),
}

#[derive(Component, Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum TypePlant {
    // Тип растения
    Tomato,
    Cucumber,
    Corn,
    Pumpkin,
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

    pub fn get_soil_line(&self) -> f32 {
        match self {
            TypePlant::Tomato => 0.4,
            TypePlant::Cucumber => 0.4,
            TypePlant::Corn => 0.4,
            TypePlant::Pumpkin => 0.0,
        }
    }

    pub fn get_wind_speed(&self) -> f32 {
        match self {
            TypePlant::Tomato => 0.6,
            TypePlant::Cucumber => 0.4,
            TypePlant::Corn => 0.6,
            TypePlant::Pumpkin => 0.5,
        }
    }

    pub fn get_wind_strength(&self) -> f32 {
        match self {
            TypePlant::Tomato => 0.03,
            TypePlant::Cucumber => 0.02,
            TypePlant::Corn => 0.04,
            TypePlant::Pumpkin => 0.03,
        }
    }
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
    pub uid: usize,
    pub plant_ability: PlantAbility,

    #[serde(skip, default = "default_static_slice")]
    pub price: &'static [f64],
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum PlantAbilityData {
    None,
    TomatoClickCombo { combo: usize },
    CornBoomHarvet { max_boom: usize, current_boom: usize, neighbours: [Option<usize>; 4] }, 
}

impl PlantAbilityData {
    pub fn merge_max(&mut self, other: &Self) {
        match (self, other) {
            (
                PlantAbilityData::TomatoClickCombo { combo: self_combo },
                PlantAbilityData::TomatoClickCombo { combo: other_combo }
            ) => {
                *self_combo = (*self_combo).max(*other_combo);
            },
            (
                PlantAbilityData::CornBoomHarvet { max_boom: self_max_boom,.. },
                PlantAbilityData::CornBoomHarvet { max_boom: other_max_boom,.. }
            ) => {
                *self_max_boom = (*self_max_boom).max(*other_max_boom);
            }
            _ => {}
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum PlantAbilityType {
    None,
    TomatoClickCombo,
    CornBoomHarvet, 
}

impl From<PlantAbilityData> for PlantAbilityType {
    fn from(value: PlantAbilityData) -> Self {
        match value {
            PlantAbilityData::None => PlantAbilityType::None,
            PlantAbilityData::TomatoClickCombo { .. } => PlantAbilityType::TomatoClickCombo,
            PlantAbilityData::CornBoomHarvet { .. } => PlantAbilityType::CornBoomHarvet,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AbilityType {
    None,
    Global,
    Single
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlantAbility {
    pub ability_type: AbilityType,
    pub data: PlantAbilityData,
}

impl PlantAbility {
    pub fn check_type(&self, ability_type: AbilityType) -> bool {
        self.ability_type == ability_type
    }
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum PlantStateGrowth {
    #[default]
    Seed,
    Sprout,
    Sapling,
    Mature,
}

impl PlantStateGrowth {
    pub fn atlas_texture_id(&self) -> usize {
        match self {
            Self::Seed => 0,
            Self::Sprout => 1,
            Self::Sapling => 2,
            Self::Mature => 3,
        }
    }
}

#[derive(Resource, Default, PartialEq, Eq)]
pub enum MouseStage {
    #[default]
    Click,
    Dragg,
    Loock,
}

#[derive(Resource, Default)]
pub struct DragItem { // Обьект курсора
    pub entity: Option<Entity>,
    pub mouse_stage: MouseStage,
}

#[derive(Resource, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct GlobalInventory {
    pub sunlit_nursery_inv: HashMap<usize, SlotState>,
    pub shadow_greenhouse_inv: HashMap<usize, SlotState>,
}

impl Default for GlobalInventory {
    fn default() -> Self {
        let mut sn_inventory = HashMap::new();
        let mut sg_inventory = HashMap::new();

        for i in 0..16 {
            if i < 4 {
                sn_inventory.insert(i, SlotState::Empty);
                continue;
            }
            sn_inventory.insert(i, SlotState::Locked);
        };

        for i in 0..15 {
            if i < 3 {
                sg_inventory.insert(i, SlotState::Empty);
                continue;
            }
            sg_inventory.insert(i, SlotState::Locked);
        };

        Self {
            sunlit_nursery_inv: sn_inventory,
            shadow_greenhouse_inv: sg_inventory
        }
    }
}

impl MapStore<HashMap<usize, SlotState>> for GlobalInventory {
    fn get_for_world (&self, world: &State<CurrentWorld>) -> Option<&HashMap<usize, SlotState>> {
        match world.get() {
            CurrentWorld::SunlitNursery => Some(&self.sunlit_nursery_inv),
            CurrentWorld::ShadowGreenhouse => Some(&self.shadow_greenhouse_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    fn get_for_world_mut (&mut self, world: &State<CurrentWorld>) -> Option<&mut HashMap<usize, SlotState>> {
        match world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.sunlit_nursery_inv),
            CurrentWorld::ShadowGreenhouse => Some(&mut self.shadow_greenhouse_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }
}


impl GlobalInventory {
    pub fn add_plant(
        &mut self,
        current_world: &State<CurrentWorld>,
        new_plant: Plant,
        idx: Option<usize>,
        uid: usize,
    ) {
        let Some(inventory) = self.get_for_world_mut(current_world) else { return; };

        let mut plant = new_plant;
        plant.uid = uid;

        if let Some(i) = idx
            && let Some(slot_state) = inventory.get_mut(&i)
                && *slot_state == SlotState::Empty {
                    *slot_state = SlotState::Occupied(plant);
                    return;
                };

        for i in 0..16 {
            let Some(slot_state) = inventory.get_mut(&i) else { continue; };

            if *slot_state == SlotState::Empty {
                *slot_state = SlotState::Occupied(plant);
                break;
            }
        }
    }

    pub fn move_plant(
        &mut self,
        current_world: &State<CurrentWorld>,
        old_id: usize,
        new_id: usize,
    ) {
        let Some(inventory) = self.get_for_world_mut(current_world) else { return; };

        if let Some(SlotState::Locked) = inventory.get(&new_id) { return; };

        let Some (new_val) = inventory.remove(&new_id) else { return; };

        let Some(old_val) = inventory.remove(&old_id) else { 
            inventory.insert(new_id, new_val);
            return; 
        };

        inventory.insert(old_id, new_val);
        inventory.insert(new_id, old_val);
    }

    pub fn try_unlock_slot(
        &mut self,
        world: &State<CurrentWorld>,
        cat_happiness: f64,
        price: f64,
        economy: &mut Economy,
    ) {
        let Some(inv) = self.get_for_world_mut(world) else { return;};

        let Some(&id) = inv.iter()
            .filter(|(_, s)| matches!(s, SlotState::Locked))
            .map(|(k, _)| k).min() else { return; };

        if cat_happiness < price { return; };

        inv.insert(id, SlotState::Empty);

        economy.add_res(ResourceType::CatHappiness, -price);
    }

    pub fn has_empty_slot(&self, world: &State<CurrentWorld>) -> bool {
        self. get_for_world(world)
            .map(|inv| inv.values()
            .any(|s| matches!(s, SlotState::Empty)))
            .unwrap_or(false)
    }

    pub fn get_slots_unlocking(&self, world: &State<CurrentWorld>) -> Option<usize> {
        let inv = self.get_for_world(world)?;

        let min_lock_id = inv
            .iter()
            .filter(|(_, s)| matches!(s, SlotState::Locked))
            .map(|(&id, _)| id)
            .min()?;

        Some(min_lock_id)
    }

    pub fn find_ability_global(&self, plant_ability_type: PlantAbilityType, world: &State<CurrentWorld>) -> Option<PlantAbilityData> {
        if let Some(data) = self.get_for_world(world) {
            for state in data.values() {
                if let SlotState::Occupied(plant) = state
                    && plant.plant_ability.check_type(AbilityType::Global)
                    && PlantAbilityType::from(plant.plant_ability.data) == plant_ability_type {
                    
                    return Some(plant.plant_ability.data);
                };
            };
        };
        None
    }

    pub fn find_ability_single(&self, plant_ability_type: PlantAbilityType, world: &State<CurrentWorld>, buffer: &mut Vec<PlantAbilityData>) {
        if let Some(data) = self.get_for_world(world) {
            for state in data.values() {
                if let SlotState::Occupied(plant) = state
                    && plant.plant_ability.check_type(AbilityType::Single)
                    && PlantAbilityType::from(plant.plant_ability.data) == plant_ability_type {
                    
                    buffer.push(plant.plant_ability.data);
                };
            };
        };
    }

    pub fn find_ability_global_mut(&mut self, plant_ability_type: PlantAbilityType, world: &State<CurrentWorld>) -> Option<&mut PlantAbilityData> {
        if let Some(data) = self.get_for_world_mut(world) {
            for state in data.values_mut() {
                if let SlotState::Occupied(plant) = state
                    && plant.plant_ability.check_type(AbilityType::Global)
                    && PlantAbilityType::from(plant.plant_ability.data) == plant_ability_type {
                    
                    return Some(&mut plant.plant_ability.data);
                };
            };
        };
        None
    }


    pub fn find_ability_single_mut<'a>(&'a mut self, plant_ability_type: PlantAbilityType, world: &State<CurrentWorld>, buffer: &mut Vec<&'a mut PlantAbilityData>) {
        if let Some(data) = self.get_for_world_mut(world) {
            for state in data.values_mut() {
                if let SlotState::Occupied(plant) = state
                    && plant.plant_ability.check_type(AbilityType::Single)
                    && PlantAbilityType::from(plant.plant_ability.data) == plant_ability_type {
                    
                    buffer.push(&mut plant.plant_ability.data);
                };
            };
        };
    }
    
    pub fn find_ability_global_all(&self, plant_ability_type: PlantAbilityType, world: &State<CurrentWorld>, buffer: &mut Vec<PlantAbilityData>) {
        if let Some(data) = self.get_for_world(world) {
            for state in data.values() {
                if let SlotState::Occupied(plant) = state
                    && plant.plant_ability.check_type(AbilityType::Global)
                    && PlantAbilityType::from(plant.plant_ability.data) == plant_ability_type {
                    
                    buffer.push(plant.plant_ability.data);
                };
            };
        };
    }

    pub fn find_ability_global_mut_all<'a>(&'a mut self, plant_ability_type: PlantAbilityType, world: &State<CurrentWorld>, buffer: &mut Vec<&'a mut PlantAbilityData>) {
        if let Some(data) = self.get_for_world_mut(world) {
            for state in data.values_mut() {
                if let SlotState::Occupied(plant) = state
                    && plant.plant_ability.check_type(AbilityType::Global)
                    && PlantAbilityType::from(plant.plant_ability.data) == plant_ability_type {
                    
                    buffer.push(&mut plant.plant_ability.data);
                };
            };
        };
    }

    pub fn ability_global_to_max(buffer: &mut Vec<&mut PlantAbilityData>) {
        let Some(first) = buffer.first() else { return; };

        let mut max_data = **first;
        for data in buffer.iter() {
            max_data.merge_max(data);
        };
        for data in buffer.iter_mut() {
            data.merge_max(&max_data);
        };
    }
}