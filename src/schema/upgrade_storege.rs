use crate::schema::{types_and_states::*};
use strum_macros::{AsRefStr, Display, EnumIter};
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};
use super::config::*;
use super::economy_inventory::ResourceType;



#[derive(Resource, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct UpgradeStorege {
    pub sparcks: HashMap<(usize, usize), Upgrade>,
    pub global: HashMap<(usize, usize), Upgrade>,
    pub sunlit_nursery: HashMap<(usize, usize), Upgrade>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Upgrade {
    #[serde(skip, default = "default_static_str")]
    pub name: &'static str,
    #[serde(skip, default = "default_static_str")]
    pub description: &'static str,

    #[serde(skip, default = "default_static_slice")]
    pub levels: &'static [UpgradeLevel],
    #[serde(skip, default = "default_static_slice")]
    pub dependencies: &'static [UpgradeUID],
    pub location_prestige_req: (Option<CurrentWorld>, Option<usize>),

    pub id: UpgradeUID,
    pub texture_stage: UpgradeStage,
    pub current_level: usize,
    pub category: EGUISelectedCategories,
    pub grid_pos: (usize, usize),
}

#[derive(Clone, Copy, Debug)]
pub struct UpgradeLevel {
    pub resource_types: &'static [ResourceType],
    pub costs: &'static [f64],
    pub value: Option<f64>,
}

impl Default for UpgradeStorege {
    fn default() -> Self {
        let sparcks_item = [
            (PURR_PROFIT.grid_pos, PURR_PROFIT.clone()),
            (OVER_BLOOMING.grid_pos, OVER_BLOOMING.clone()),
        ];

        let global_item = [
            (FERTILE_SOIL.grid_pos, FERTILE_SOIL.clone()),
            (GROWTH_SPEED.grid_pos, GROWTH_SPEED.clone()),
            (JOY_BOOST.grid_pos, JOY_BOOST.clone()),
            (CARDBOARD_BOX.grid_pos, CARDBOARD_BOX.clone()),
        ];

        let sunlit_nursery_item = [
            (UNLOCK_TOMATO.grid_pos, UNLOCK_TOMATO.clone()),
            (UNLOCK_CUCUMBER.grid_pos, UNLOCK_CUCUMBER.clone()),
            (UNLOCK_CORN.grid_pos, UNLOCK_CORN.clone()),
            (UNLOCK_PUMPKIN.grid_pos, UNLOCK_PUMPKIN.clone()),
            (CONCENTRATED_NECTAR.grid_pos, CONCENTRATED_NECTAR.clone()),
            (TOMATO_BOUNTY.grid_pos, TOMATO_BOUNTY.clone()),
            (TOMATO_GROWTH.grid_pos, TOMATO_GROWTH.clone()),
            (TOMATO_JOY.grid_pos, TOMATO_JOY.clone()),
            (CUCUMBER_BOUNTY.grid_pos, CUCUMBER_BOUNTY.clone()),
            (CUCUMBER_GROWTH.grid_pos, CUCUMBER_GROWTH.clone()),
            (CUCUMBER_JOY.grid_pos, CUCUMBER_JOY.clone()),
            (CORN_BOUNTY.grid_pos, CORN_BOUNTY.clone()),
            (CORN_GROWTH.grid_pos, CORN_GROWTH.clone()),
            (CORN_JOY.grid_pos, CORN_JOY.clone()),
        ];

        Self {
            sparcks: sparcks_item.into_iter().collect(),
            global: global_item.into_iter().collect(),
            sunlit_nursery: sunlit_nursery_item.into_iter().collect(),
        }
    }
}

impl Upgrade {
    pub fn get_dependencies(&self, upgrade_storege: &UpgradeStorege) -> bool {
        self.dependencies.iter().all(|def_ip| {
            let storege = upgrade_storege.get_storege_category(self.category);

            storege
                .values()
                .find(|u| u.id == *def_ip)
                .is_some_and(|u| u.texture_stage != UpgradeStage::Locked )
        })
    }

    pub fn get_unlocking(&self) -> Option<&'static str> {
        match self.id {
            UpgradeUID::UnlockTomato => Some("plant-name-tomato"),
            UpgradeUID::UnlockCucumber => Some("plant-name-cucumber"),
            UpgradeUID::UnlockCorn => Some("plant-name-corn"),
            UpgradeUID::UnlockPumpkin => Some("plant-name-pumpkin"),
            _ => None
        }
    }

    pub fn get_location_prestige_req(&self, prestige_inv: &PrestigeRoom) -> bool {
        let Some(world) = self.location_prestige_req.0 else { return true; };

        let Some(req_level) = self.location_prestige_req.1 else { return true; };

        let Some(prestige) = prestige_inv.get_room(&world) else { return true; };

        prestige >= req_level
    }
}


impl UpgradeStorege {
    fn all_upgrages(&self) -> impl Iterator<Item = &Upgrade> {
        self.global.values().chain(self.sunlit_nursery.values()).chain(self.sparcks.values())
    }

    pub fn get_global_modifier(&self, upgrade_id: UpgradeUID) -> (Option<f64>, bool) {
         self.all_upgrages()
         .find(|u| u.id == upgrade_id && u.current_level > 0 )
         .map(|u| (u.levels[u.current_level.saturating_sub(1)].value, true))
         .unwrap_or((None, false))
    }

    pub fn get_plant_global_modifier(&self, type_plant: &TypePlant, mode: PlantGGM) -> (Option<f64>, bool) {
        match mode {
            PlantGGM::Bounty => {
                match type_plant {
                    TypePlant::Tomato => self.get_global_modifier(UpgradeUID::TomatoBounty),
                    TypePlant::Cucumber => self.get_global_modifier(UpgradeUID::CucumberBounty),
                    TypePlant::Corn => self.get_global_modifier(UpgradeUID::CornBounty),
                    TypePlant::Pumpkin => self.get_global_modifier(UpgradeUID::PumpkinBounty),
                }
            }
            PlantGGM::Growth => {
                match type_plant {
                    TypePlant::Tomato => self.get_global_modifier(UpgradeUID::TomatoGrowth),
                    TypePlant::Cucumber => self.get_global_modifier(UpgradeUID::CucumberGrowth),
                    TypePlant::Corn => self.get_global_modifier(UpgradeUID::CucumberJoy),
                    TypePlant::Pumpkin => self.get_global_modifier(UpgradeUID::PumpkinGrowth),
                }
            }
            PlantGGM::Joy => {
                match type_plant {
                    TypePlant::Tomato => self.get_global_modifier(UpgradeUID::TomatoJoy),
                    TypePlant::Cucumber => self.get_global_modifier(UpgradeUID::CucumberJoy),
                    TypePlant::Corn => self.get_global_modifier(UpgradeUID::CornJoy),
                    TypePlant::Pumpkin => self.get_global_modifier(UpgradeUID::PumpkinJoy),
                }
            }
        }
    }

    pub fn get_storege_category(
        &self,
        category: EGUISelectedCategories,
    ) -> &HashMap<(usize, usize), Upgrade> {
        match category {
            EGUISelectedCategories::Sparcks => &self.sparcks,
            EGUISelectedCategories::Global => &self.global,
            EGUISelectedCategories::SunlitNursery => &self.sunlit_nursery,
        }
    }
}