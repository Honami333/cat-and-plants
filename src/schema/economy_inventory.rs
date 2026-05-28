use strum_macros::{AsRefStr, Display, EnumIter};
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};
use crate::schema::hud::TypePage;
use crate::schema::upgrade_storege::{UpgradeStorege, PlantGGM};
use super::common::*;
use super::global_inventory::TypePlant;


#[derive(Resource, Clone, Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Economy {
    pub vault : HashMap<ResourceType, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TradeWell {
    pub well: &'static [(ResourceType, f64)],
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr, PartialEq, Display, Serialize, Deserialize, Eq, Hash)]
pub enum ResourceType {
    #[strum(serialize = "res-cat-happiness")] CatHappiness,
    
    #[strum(serialize = "res-tomatoes")] Tomatoes,
    #[strum(serialize = "res-cucumbers")] Cucumbers,
    #[strum(serialize = "res-corn")] Corn,
    #[strum(serialize = "res-pumpkin")] Pumpkin,



    #[strum(serialize = "res-sun-sparks")] SunSparks,
}

impl From<TypePlant> for ResourceType {
    fn from(value: TypePlant) -> Self {
        match value {
            TypePlant::Tomato => ResourceType::Tomatoes,
            TypePlant::Cucumber => ResourceType::Cucumbers,
            TypePlant::Corn => ResourceType::Corn,
            TypePlant::Pumpkin => ResourceType::Pumpkin,
        }
    }
}

impl TryFrom<ResourceType> for TypePlant {
    type Error = ();

    fn try_from(value: ResourceType) -> Result<Self, Self::Error> {
        match value {
            ResourceType::Tomatoes => Ok(TypePlant::Tomato),
            ResourceType::Cucumbers => Ok(TypePlant::Cucumber),
            ResourceType::Corn => Ok(TypePlant::Corn),
            ResourceType::Pumpkin => Ok(TypePlant::Pumpkin),
            _ => Err(())
        }
    }
}

impl TryFrom<TypePage> for TypePlant {
    type Error = ();

    fn try_from(value: TypePage) -> Result<Self, Self::Error> {
        match value {
            TypePage::TomatoBuy => Ok(TypePlant::Tomato),
            TypePage::CucumberBuy => Ok(TypePlant::Cucumber),
            TypePage::CornBuy => Ok(TypePlant::Corn),
            TypePage::PumpkinBuy => Ok(TypePlant::Pumpkin),
            _ => Err(())
        }
    }
}

impl Economy {
    pub fn get_res(&self, res: ResourceType) -> f64 {
        *self.vault.get(&res).unwrap_or(&0.0)
    }

    pub fn add_res(&mut self, res: ResourceType, amount: f64) {
        *self.vault.entry(res).or_insert(0.0) += amount;
    }

    pub fn feed_res_list(&mut self, percent: f64, select_item_list: &Vec<ResourceType>,) {
        let factor = 1.0 - (percent / 100.0).clamp(0.0, 1.0);

        for res in select_item_list.iter() {
            let Some(val) = self.vault.get_mut(res) else { continue; };

            if !(*res == ResourceType::CatHappiness) {
                *val = (*val * factor).floor();
            }
        }
    }

    pub fn egui_get_res_list(
        &self,
        well: TradeWell,
        percent: f64,
        upgrade_storege: &UpgradeStorege,
        select_item_list: &Vec<ResourceType>,
    ) -> f64 {
        let mut all_trade = 0.0;

        let factor = percent / 100.0;

        for res in select_item_list.iter() {
            let item_count = self.get_res(*res);

            if *res == ResourceType::CatHappiness { continue; };

            if item_count <= 0.0 { continue; };

            let Some(&price) = well.well.iter().find(|(r, _)| *r == *res) else { continue; };

            let mut up_value_2 =  1.0;

            if let Ok(type_plant) = (*res).try_into() {
                if let (Some(value), _) = upgrade_storege.get_plant_global_modifier(&type_plant, PlantGGM::Joy) {up_value_2 = value};
            };

            all_trade += (item_count * factor * price.1 * up_value_2).floor() ;
        }

        all_trade
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
            item_res.push(self.get_res(*res));
        };

        (res_vec, item_res)
    }
}