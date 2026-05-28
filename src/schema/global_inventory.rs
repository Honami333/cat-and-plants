
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

#[derive(Component, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlantStateGrowth {
    Seed,
    Sprout,
    Sapling,
    Mature,
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

#[derive(Resource, Default)]
pub struct DragItem {
    // Обьект курсора
    pub entity: Option<Entity>,
}

#[derive(Resource, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct GlobalInventory {
    pub sunlit_nursery_inv: HashMap<usize, SlotState>,
}

impl Default for GlobalInventory {
    fn default() -> Self {
        let mut sn_inventory = HashMap::new();

        for i in 0..16 {
            if i < 4 {
                sn_inventory.insert(i, SlotState::Empty);
                continue;
            }
            sn_inventory.insert(i, SlotState::Locked);
        };

        Self {
            sunlit_nursery_inv: sn_inventory,
        }
    }
}

impl MapStore<HashMap<usize, SlotState>> for GlobalInventory {
    fn get_for_world (&self, world: &State<CurrentWorld>) -> Option<&HashMap<usize, SlotState>> {
        match world.get() {
            CurrentWorld::SunlitNursery => Some(&self.sunlit_nursery_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    fn get_for_world_mut (&mut self, world: &State<CurrentWorld>) -> Option<&mut HashMap<usize, SlotState>> {
        match world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.sunlit_nursery_inv),
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
    ) {
        let Some(inventory) = self.get_for_world_mut(current_world) else { return; };

        if let Some(i) = idx {
            if let Some(slot_state) = inventory.get_mut(&i) {
                if *slot_state == SlotState::Empty {
                    *slot_state = SlotState::Occupied(new_plant);
                    return;
                };
            };
        };

        for i in 0..16 {
            let Some(slot_state) = inventory.get_mut(&i) else { continue; };

            if *slot_state == SlotState::Empty {
                *slot_state = SlotState::Occupied(new_plant);
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
        let Some(inv) = self.get_for_world(world) else { return None; };

        let min_lock_id = inv
            .iter()
            .filter(|(_, s)| matches!(s, SlotState::Locked))
            .map(|(&id, _)| id)
            .min()?;

        Some(min_lock_id)
    }
}