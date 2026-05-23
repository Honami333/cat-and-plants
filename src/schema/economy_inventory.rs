use crate::schema::{types_and_states::*};
use strum_macros::{AsRefStr, Display, EnumIter};
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};


#[derive(Resource, Clone, Default, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Economy {
    pub vault : HashMap<ResourceType, f64>,
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr, PartialEq, Display, Serialize, Deserialize, Eq, Hash)]
pub enum ResourceType {
    #[strum(serialize = "res-cat-happiness")] CatHappiness,
    
    #[strum(serialize = "res-tomatoes")] Tomatoes,
    #[strum(serialize = "res-cucumbers")] Cucumbers,
    #[strum(serialize = "res-corn")] Corn,
    #[strum(serialize = "res-pumpkin")] Pumpkin,

    #[strum(serialize = "res-none")] None,

    #[strum(serialize = "res-sun-sparks")] SunSparks,
}


impl Economy {
    pub fn get_res(&self, res: ResourceType) -> f64 {
        *self.vault.get(&res).unwrap_or(&0.0)
    }

    pub fn add_res(&mut self, res: ResourceType, amount: f64) {
        *self.vault.entry(res).or_insert(0.0) += amount;
    }

    pub fn sell_all(&mut self, percent: f64) {
        let factor = 1.0 - (percent / 100.0).clamp(0.0, 1.0);

        for (res, val) in self.vault.iter_mut() {
            if !matches!(res, ResourceType::CatHappiness | ResourceType::None) {
                *val = (*val * factor).floor();
            }
        }
    }

    pub fn egui_get_res(&self, res: EGUIResourceType) -> f64 {
        match res {
            EGUIResourceType::All => self.vault.iter()
                    .filter(|(k, _)| !matches!(k, ResourceType::CatHappiness | ResourceType::None))
                    .map(|(_, v)| v).sum(),
            _ => self.get_res(res.into()),
        }
    }

    pub fn egui_get_res_all(&self, well: TradeWell, percent: f64) -> f64 {
        let mut all_trade = 0.0;
        let factor = percent / 100.0;

        for (res, item_count) in self.vault.iter() {
            if matches!(res, ResourceType::CatHappiness | ResourceType::None) { continue; };

            if *item_count < 0.0 { continue; };

            let well_idx = (*res as usize).saturating_sub(1);

            let Some(&price) = well.well.get(well_idx) else { continue; };

            all_trade += (item_count * factor).floor() * price;
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