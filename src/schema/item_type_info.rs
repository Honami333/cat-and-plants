
use serde::{Serialize, Deserialize};
use bevy::{platform::collections::HashMap, prelude::*};
use super::global_inventory::TypePlant;
use super::common::*;


#[derive(Resource, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct ItemTypeInfo {
    pub last_plant_uid: usize,
    pub item_count_inv: HashMap<TypePlant, usize>,
}

impl Default for ItemTypeInfo {
    fn default() -> Self {
        let sn_inv_map = HashMap::from([
            (TypePlant::Tomato, 0),
            (TypePlant::Cucumber, 0),
            (TypePlant::Corn, 0),
            (TypePlant::Pumpkin, 0),
        ]);

        Self {
            last_plant_uid: 0,
            item_count_inv: sn_inv_map,
        }
    }
}

impl MapStore<HashMap<TypePlant, usize>> for ItemTypeInfo {
    fn get_for_world(
        &self,
        current_world: &State<CurrentWorld>,
    ) -> Option<&HashMap<TypePlant, usize>> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&self.item_count_inv),
            CurrentWorld::ShadowGreenhouse => Some(&self.item_count_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    fn get_for_world_mut(
        &mut self,
        current_world: &State<CurrentWorld>,
    ) -> Option<&mut HashMap<TypePlant, usize>> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.item_count_inv),
            CurrentWorld::ShadowGreenhouse => Some(&mut self.item_count_inv),
            CurrentWorld::WarmPawsPorch => None,
        }
    }
}

impl ItemTypeInfo {
    pub fn add(&mut self, res: TypePlant, current_world: &State<CurrentWorld>) {
        let Some(count_inv) = self.get_for_world_mut(current_world) else { return; };

        let Some(res_count) = count_inv.get_mut(&res) else { return; };

        *res_count += 1;
    }
}
