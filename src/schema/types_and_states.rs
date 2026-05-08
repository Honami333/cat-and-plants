use crate::schema::config::{Plant, Upgrade};
use bevy::{math::f64, prelude::*};
use strum_macros::{AsRefStr, Display, EnumIter};

// Типы и Состояния
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    // Стадии загрузки
    #[default]
    Loading,
    Playing,
}

#[derive(States, Hash, Resource, Default, Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum CurrentWorld {
    // Выбраный мир
    WarmPawsPorch,
    #[default]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlotState {
    // Состояние слота
    Locked,
    Empty,
    Occupied(Plant),
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TypePlant {
    // Тип растения
    Tomato,
    Cucumber,
    Corn,
    Pumpkin,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum PlantStateGrowth {
    Seed,
    Sprout,
    Sapling,
    Mature,
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr, PartialEq)]
pub enum ResourceType {
    CatHappiness,
    Tomatoes,
    Cucumbers,
    Corn,
    Pumpkin,
    None,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum ShaderType {
    SNWindowLight = 0,
    WPPWindowLight = 1,
}

#[derive(Clone, Copy, PartialEq)]
pub enum UpgradeUID {
    FertileSoil,
    WholesaleSupply,
    SelectiveBreeding,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum EGUISelectedCategories {
    Global,
    SunlitNursery,
}

// Список инвенторей
#[derive(Resource)]
pub struct GlobalInventory {
    pub sunlit_nursery_inv: [SlotState; 16],
}

#[derive(Resource, Default)]
pub struct Economy {
    pub storage: [f64; 6],
}

#[derive(Resource)]
pub struct UpgradeStorege {
    pub global: Vec<Upgrade>,
    pub sunlit_nursery: Vec<Upgrade>,
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

#[derive(Resource, Default)]
pub struct CountItemType {
    pub sunlit_nursery_inv: [usize; 4],
}
