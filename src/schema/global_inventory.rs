
use crate::schema::{types_and_states::*};
use super::logic::MapStore;
use super::config::default_static_slice;
use super::economy_inventory::*;
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SlotState {
    // Состояние слота
    Locked,
    Empty,
    Occupied(Plant),
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
    ) {
        let Some(inventory) = self.get_for_world_mut(current_world) else { return; };

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
        prices: &'static [f64],
    ) -> (bool, Option<usize>) {
        let Some(inv) = self.get_for_world_mut(world) else { return (false, None);};

        let Some(&id) = inv.iter()
            .filter(|(_, s)| matches!(s, SlotState::Locked))
            .map(|(k, _)| k).min() else { return (false, None); };

        let Some(price) = prices.get(id) else { return (false, None); };

        if cat_happiness >= *price {
            inv.insert(id, SlotState::Empty);
            return (true, Some(id));
        };

        (false, None)
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