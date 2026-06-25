
use serde::{Serialize, Deserialize};
use bevy::{platform::collections::HashMap, prelude::*};
use super::global_inventory::TypePlant;
use super::common::*;

pub enum ModifierOperation {
    Set,
    Add,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum PlantAbility {
    TomatoClickCombo,
    CornBoomHarvet,
}

#[derive(Resource, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct ItemTypeInfo {
    pub item_count_inv: HashMap<TypePlant, usize>,
    pub sn_plant_ability: HashMap<TypePlant, HashMap<PlantAbility, Vec<[usize; 2]>>>,
}

impl Default for ItemTypeInfo {
    fn default() -> Self {
        let sn_inv_map = HashMap::from([
            (TypePlant::Tomato, 0),
            (TypePlant::Cucumber, 0),
            (TypePlant::Corn, 0),
            (TypePlant::Pumpkin, 0),
        ]);

        let sn_plant_ability_map = HashMap::from([
            (
                TypePlant::Tomato, 
                HashMap::from([
                    (PlantAbility::TomatoClickCombo, Vec::from([([0_usize, 1_usize])]))
                ])
            ),
            (
                TypePlant::Corn, 
                HashMap::from([
                    (PlantAbility::CornBoomHarvet, Vec::from([([0_usize, 1_usize])]))
                ])
            ),
        ]);

        Self {
            item_count_inv: sn_inv_map,
            sn_plant_ability: sn_plant_ability_map,
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
            CurrentWorld::WarmPawsPorch => None,
        }
    }

    fn get_for_world_mut(
        &mut self,
        current_world: &State<CurrentWorld>,
    ) -> Option<&mut HashMap<TypePlant, usize>> {
        match current_world.get() {
            CurrentWorld::SunlitNursery => Some(&mut self.item_count_inv),
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

    pub fn add_to_plant_ability(
        &mut self,
        type_plant: &TypePlant,
        plant_ability_type: PlantAbility,
        new_value: usize,
        action: ModifierOperation,
        i: usize
    ) {
        let Some(plant_ability) = self.sn_plant_ability.get_mut(type_plant) else { return; };

        let Some(ability_vec) = plant_ability.get_mut(&plant_ability_type) else { return; };

        let Some(ability_case) = ability_vec.get_mut(i) else { return; };

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
        i: usize,
    ) -> f64 {
        let Some(plant_ability) = self.sn_plant_ability.get(type_plant) else { return 0.0; };

        let Some(ability_vec) = plant_ability.get(&plant_ability_type) else { return 0.0; };

        let Some(ability_case) = ability_vec.get(i) else { return 0.0; };

        let Some(ability_count) = ability_case.first() else { return 0.0; };

        let Some(ability_marker) = ability_case.get(1) else { return 0.0; };

        if *ability_marker == 0 { return  0.0; };

        *ability_count as f64 / *ability_marker as f64
    }

    pub fn get_plant_ability_vec_len(
        &self,
        type_plant: &TypePlant,
        plant_ability_type: PlantAbility,
    ) -> usize {
        let Some(plant_ability) = self.sn_plant_ability.get(type_plant) else { return 0; };

        let Some(ability_vec) = plant_ability.get(&plant_ability_type) else { return 0; };

        ability_vec.len()
    }
}
