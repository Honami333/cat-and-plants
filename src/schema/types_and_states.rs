use bevy::{math::f64, prelude::*};
use strum_macros::{AsRefStr, Display, EnumIter};
use serde::{Serialize, Deserialize};
use super::economy_inventory::ResourceType;



impl From<EGUIResourceType> for ResourceType {
    fn from(value: EGUIResourceType) -> Self {
        match value {
            EGUIResourceType::Tomatoes => ResourceType::Tomatoes,
            EGUIResourceType::Cucumbers => ResourceType::Cucumbers,
            EGUIResourceType::Corn => ResourceType::Corn,
            EGUIResourceType::Pumpkin => ResourceType::Pumpkin,
            _ => ResourceType::None,
        }
    }
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

// Типы и Состояния
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    // Стадии загрузки
    #[default]
    Loading,
    Menu,
    LoadGame,
    Playing,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum MenuPage {
    #[default]
    Main,
    SaveSlot,
    Settings,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
pub enum SlotTextureState {
    #[default]
    Empty = 0,
    Occupied = 1,
}

#[derive(States, Hash, Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum SvSlBT {
    #[strum(serialize = "menu-start")] Start,
    #[strum(serialize = "menu-continue")] Continue,
    #[strum(serialize = "menu-delete")] Delete,
}


#[derive(States, Hash, Resource, Default, Clone, Copy, PartialEq, Eq, Debug, Display, Serialize, Deserialize)]
pub enum CurrentWorld {
    #[default]
    #[strum(serialize = "world-warm-paws")] WarmPawsPorch,
    #[strum(serialize = "world-sunlit-nursery")] SunlitNursery,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum EGUICurrntWorld {
    #[strum(serialize = "world-all")] All,
    #[strum(serialize = "world-sunlit-nursery")] SunlitNursery,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum EGUIResourceType {
    #[strum(serialize = "res-type-all")] All,
    #[strum(serialize = "res-type-tomato")] Tomatoes,
    #[strum(serialize = "res-type-cucumber")] Cucumbers,
    #[strum(serialize = "res-type-corn")] Corn,
    #[strum(serialize = "res-type-pumpkin")] Pumpkin,
    #[strum(serialize = "res-type-none")] None,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TradeWell {
    pub well: [f64; 5],
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TypeButton {
    TomatoButton,
    CucumberButton,
    CornButton,
    PumpkinButton,
    SlotsUnLocking,
}



#[derive(Component, Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum TypePlant {
    // Тип растения
    Tomato,
    Cucumber,
    Corn,
    Pumpkin,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Copy)]
pub enum PlantAbility {
    TomatoClickCombo,
    CornBoomHarvet,
}

pub enum ModifierOperation {
    Set,
    Add,
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlantStateGrowth {
    Seed,
    Sprout,
    Sapling,
    Mature,
}



#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum ShaderType {
    SNWindowLight = 0,
    WPPWindowLight = 1,
}

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

pub enum PlantGGM {
    Bounty,
    Growth,
    Joy
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display, Serialize, Deserialize)]
pub enum EGUISelectedCategories {
    #[strum(serialize = "cat-sparks")] Sparcks,
    #[strum(serialize = "cat-global")] Global,
    #[strum(serialize = "cat-nursery")] SunlitNursery,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum ScreenMode  {
    #[strum(serialize = "screen-windowed")] Windowed,
    #[strum(serialize = "screen-fullscreen")] Fullscreen,
}


// Глобальные действия
#[derive(Resource, Default)]
pub struct DragItem {
    // Обьект курсора
    pub entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct WorldScale {
    pub scale: f32,
}

#[derive(Resource)]
pub struct TradeState {
    pub selected_world: EGUICurrntWorld,
    pub selected_item: EGUIResourceType,
    pub selected_percent: u8,
    pub selected_economy: f64,
}

#[derive(Resource)]
pub struct UpgradeState {
    pub selected_categories: EGUISelectedCategories,
}

#[derive(Debug, Resource, Default)]
pub struct MenuCurPage {
    pub page: MenuPage,
    pub game_menu: bool,
}