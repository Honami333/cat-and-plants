
use bevy::{platform::collections::HashMap, prelude::*};
use strum_macros::{Display, EnumIter};
use serde::{Serialize, Deserialize};
use crate::schema::resources::GameAssets;
use crate::schema::{common::CurrentWorld, economy_inventory::ResourceType, upgrade_storege::UpgradeUID, global_inventory::Plant};
use crate::content::{world::sunlit_nursery::*, trade_tab::*};


#[derive(States, Hash, Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum SvSlBT {
    #[strum(serialize = "menu-start")] Start,
    #[strum(serialize = "menu-continue")] Continue,
    #[strum(serialize = "menu-delete")] Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum EGUICurrntWorld {
    #[strum(serialize = "world-all")] All,
    #[strum(serialize = "world-sunlit-nursery")] SunlitNursery,
    ShadowGreenhouse,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display, Serialize, Deserialize)]
pub enum EGUISelectedCategories {
    #[strum(serialize = "cat-sparks")] Sparcks,
    #[strum(serialize = "cat-global")] Global,
    #[strum(serialize = "cat-nursery")] SunlitNursery,
    ShadowGreenhouse,
}


#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum MenuPage {
    #[default]
    Main,
    SaveSlot,
    Settings,
}

#[derive(Resource)]
pub struct FeedState {
    pub selected_world: EGUICurrntWorld,
    pub selected_item: Vec<ResourceType>,
    pub selected_percent: u8,
    pub selected_economy: f64,
}

impl Default for FeedState {
    fn default() -> Self {
        Self {
            selected_world: EGUICurrntWorld::All,
            selected_item: Vec::new(),
            selected_percent: 100,
            selected_economy: 0.0,
        }
    }
}

#[derive(Debug, Resource, Default)]
pub struct MenuCurPage {
    pub page: MenuPage,
    pub game_menu: bool,
}

#[derive(Debug, Resource, PartialEq)]
pub struct TradeState {
    pub tabs: HashMap<usize, (CurrentWorld, TradeTab)>,
    pub active_tab_index: usize,
    pub points_window_start: usize,
    pub scroll_to_tab: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradeTab {
    pub title_key: &'static str,
    pub description: &'static str,
    pub dragg_info: &'static str,
    pub type_page: TypePage,
}

#[derive(Component, Clone, Copy, PartialEq, Debug, EnumIter, Hash, Eq)]
pub enum TypePage {
    TomatoBuy,
    CucumberBuy,
    CornBuy,
    PumpkinBuy,
    SlotsUnLocking,
}

impl TypePage {
    pub fn get_plant_cfg(&self) -> Option<Plant> {
        match *self {
            TypePage::TomatoBuy => Some(PL_TOMATO),
            TypePage::CucumberBuy => Some(PL_CUCUMBER),
            TypePage::CornBuy => Some(PL_CORN),
            TypePage::PumpkinBuy => Some(PL_PUMPKIN),
            TypePage::SlotsUnLocking => None,
        }
    }

    pub fn get_dependencies_upgrade(&self) -> Option<UpgradeUID> {
        match *self {
            TypePage::CucumberBuy => Some(UpgradeUID::UnlockCucumber),
            TypePage::CornBuy => Some(UpgradeUID::UnlockCorn),
            TypePage::PumpkinBuy => Some(UpgradeUID::UnlockPumpkin),
            _ => None,
        }
    }

    pub fn get_button_sprite(&self, assets: &GameAssets) -> Handle<Image> {
        match self {
            TypePage::TomatoBuy => assets.button_buy_tomato.clone(),
            TypePage::CucumberBuy => assets.button_buy_cucumber.clone(),
            TypePage::CornBuy => assets.button_buy_corn.clone(),
            TypePage::PumpkinBuy => assets.button_buy_pumpkin.clone(),
            TypePage::SlotsUnLocking => assets.button_slots_unlocking.clone(),
        }
    }
}

impl Default for TradeState {
    fn default() -> Self {
        let tabs_hashmap = HashMap::from([
            (
                0_usize,
                (CurrentWorld::SunlitNursery, TOMATO_TAB)
            ),
            (
                1_usize,
                (CurrentWorld::SunlitNursery, CUCUMBER_TAB)
            ),
            (
                2_usize,
                (CurrentWorld::SunlitNursery, CORN_TAB)
            ),
            (
                3_usize,
                (CurrentWorld::SunlitNursery, PUMPKIN_TAB)
            ),
            (
                4_usize,
                (CurrentWorld::SunlitNursery, SN_SLOTSUNLOCKING_TAB)
            ),
            (
                5_usize,
                (CurrentWorld::ShadowGreenhouse, SG_SLOTSUNLOCKING_TAB)
            ),
        ]);

        Self {
             tabs: tabs_hashmap,
             active_tab_index: 0,
             points_window_start: 0,
             scroll_to_tab: None,
        }
    }
}

#[derive(Resource, Default, PartialEq)]
pub struct VisualCounter {
    pub display_value: HashMap<ResourceType, [f64; 2]>,
    pub target_value: HashMap<ResourceType, f64>,
}

#[derive(Component)]
pub struct PageItem {
    pub type_page: TypePage,
}