use crate::schema::config::Plant;
use bevy::{math::f64, prelude::*};
use strum_macros::{AsRefStr, Display, EnumIter};
use serde::{Serialize, Deserialize};

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
    Start,
    Continue,
    Delete
}

#[derive(States, Hash, Resource, Default, Clone, Copy, PartialEq, Eq, Debug, Display, Serialize, Deserialize)]
pub enum CurrentWorld { // Выбраный мир
    #[default]
    WarmPawsPorch,
    SunlitNursery,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum EGUICurrntWorld {
    All,
    SunlitNursery,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum EGUIResourceType {
    All,
    #[strum(serialize = "🍅")]
    Tomatoes,
    #[strum(serialize = "🥒")]
    Cucumbers,
    #[strum(serialize = "🌽")]
    Corn,
    #[strum(serialize = "🎃")]
    Pumpkin,
    #[strum(serialize = "")]
    None,
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SlotState {
    // Состояние слота
    Locked,
    Empty,
    Occupied(Plant),
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum TypePlant {
    // Тип растения
    Tomato,
    Cucumber,
    Corn,
    Pumpkin,
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlantStateGrowth {
    Seed,
    Sprout,
    Sapling,
    Mature,
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr, PartialEq, Display)]
pub enum ResourceType {
    #[strum(serialize = "😸")] CatHappiness,
    
    #[strum(serialize = "🍅")] Tomatoes,
    #[strum(serialize = "🥒")] Cucumbers,
    #[strum(serialize = "🌽")] Corn,
    #[strum(serialize = "🎃")] Pumpkin,

    None,

    SunSparks,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum ShaderType {
    SNWindowLight = 0,
    WPPWindowLight = 1,
}

#[derive(Clone, Copy, PartialEq, Display, EnumIter, Serialize, Deserialize)]
pub enum UpgradeUID {
    #[strum(serialize = "Fertile Soil")]
    FertileSoil,
    #[strum(serialize = "Growth Catalysts")]
    WholesaleSupply,
    #[strum(serialize = "Catnip Infusion")]
    SelectiveBreeding,
    #[strum(serialize = "Wholesale Supplies")]
    CardboardBox,
    #[strum(serialize = "Crunchy Snack")]
    UnlockCucumber,
    #[strum(serialize = "Sweet Kernels")]
    UnlockCorn,
    #[strum(serialize = "Festive Feast")]
    UnlockPumpkin,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeStage {
    Locked,
    Available,
    Growing,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display, Serialize, Deserialize)]
pub enum EGUISelectedCategories {
    Global,
    SunlitNursery,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum ScreenMode  {
    Windowed,
    Fullscreen,
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