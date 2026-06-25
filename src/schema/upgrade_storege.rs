use crate::content::upgrades::{prestige::*, global::*, sunlit_nursery::*};
use strum_macros::{Display, EnumIter};
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};
use super::save_file::*;
use super::hud::EGUISelectedCategories;
use super::common::CurrentWorld;
use super::global_inventory::TypePlant;
use super::economy_inventory::ResourceType;
use super::prestige::PrestigeRoom;


#[derive(Clone, Copy, PartialEq, Display, EnumIter, Serialize, Deserialize, Debug)]
pub enum UpgradeUID {
    #[strum(serialize = "purr-profit-name")] PurrProfit,
    #[strum(serialize = "over-blooming-name")] OverBlooming,
    #[strum(serialize = "fertile-soil-name")] FertileSoil,
    #[strum(serialize = "growth-catalysts-name")] WholesaleSupply,
    #[strum(serialize = "catnip-infusion-name")] SelectiveBreeding,
    #[strum(serialize = "wholesale-supplies-name")] CardboardBox,
    #[strum(serialize = "unlock-tomato-name")] UnlockTomato,
    #[strum(serialize = "unlock-cucumber-name")] UnlockCucumber,
    #[strum(serialize = "unlock-corn-name")] UnlockCorn,
    #[strum(serialize = "unlock-pumpkin-name")] UnlockPumpkin,
    #[strum(serialize = "concentrated-nectar-name")] ConcentratedNectar,
    #[strum(serialize = "tomato-bounty-name")] TomatoBounty,
    #[strum(serialize = "tomato-growth-name")] TomatoGrowth,
    #[strum(serialize = "tomato-joy-name")] TomatoJoy,
    #[strum(serialize = "cucumber-bounty-name")] CucumberBounty,
    #[strum(serialize = "cucumber-growth-name")] CucumberGrowth,
    #[strum(serialize = "cucumber-joy-name")] CucumberJoy,
    #[strum(serialize = "corn-bounty-name")] CornBounty,
    #[strum(serialize = "corn-growth-name")] CornGrowth,
    #[strum(serialize = "corn-joy-name")] CornJoy,
    PumpkinBounty,
    PumpkinGrowth,
    PumpkinJoy,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum UpgradeStage {
    Locked,
    Available,
    Growing,
    Max,
}

#[derive(Resource)]
pub struct UpgradeState {
    pub selected_categories: EGUISelectedCategories,
}

impl Default for UpgradeState {
    fn default() -> Self {
        Self {
            selected_categories: EGUISelectedCategories::Global,
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

pub enum PlantGGM {
    Bounty,
    Growth,
    Joy
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

        let Some(prestige) = prestige_inv.get_room(world) else { return true; };

        prestige >= req_level
    }
}

#[derive(Resource, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct UpgradeStorege {
    pub sparcks: HashMap<(usize, usize), Upgrade>,
    pub global: HashMap<(usize, usize), Upgrade>,
    pub sunlit_nursery: HashMap<(usize, usize), Upgrade>,
}

impl Default for UpgradeStorege {
    fn default() -> Self {
        let sparcks_item = [
            (PURR_PROFIT.grid_pos, PURR_PROFIT),
            (OVER_BLOOMING.grid_pos, OVER_BLOOMING),
        ];

        let global_item = [
            (FERTILE_SOIL.grid_pos, FERTILE_SOIL),
            (GROWTH_SPEED.grid_pos, GROWTH_SPEED),
            (JOY_BOOST.grid_pos, JOY_BOOST),
            (CARDBOARD_BOX.grid_pos, CARDBOARD_BOX),
        ];

        let sunlit_nursery_item = [
            (UNLOCK_TOMATO.grid_pos, UNLOCK_TOMATO),
            (UNLOCK_CUCUMBER.grid_pos, UNLOCK_CUCUMBER),
            (UNLOCK_CORN.grid_pos, UNLOCK_CORN),
            (UNLOCK_PUMPKIN.grid_pos, UNLOCK_PUMPKIN),
            (CONCENTRATED_NECTAR.grid_pos, CONCENTRATED_NECTAR),
            (TOMATO_BOUNTY.grid_pos, TOMATO_BOUNTY),
            (TOMATO_GROWTH.grid_pos, TOMATO_GROWTH),
            (TOMATO_JOY.grid_pos, TOMATO_JOY),
            (CUCUMBER_BOUNTY.grid_pos, CUCUMBER_BOUNTY),
            (CUCUMBER_GROWTH.grid_pos, CUCUMBER_GROWTH),
            (CUCUMBER_JOY.grid_pos, CUCUMBER_JOY),
            (CORN_BOUNTY.grid_pos, CORN_BOUNTY),
            (CORN_GROWTH.grid_pos, CORN_GROWTH),
            (CORN_JOY.grid_pos, CORN_JOY),
        ];

        Self {
            sparcks: sparcks_item.into_iter().collect(),
            global: global_item.into_iter().collect(),
            sunlit_nursery: sunlit_nursery_item.into_iter().collect(),
        }
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
                    TypePlant::Corn => self.get_global_modifier(UpgradeUID::CucumberGrowth),
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