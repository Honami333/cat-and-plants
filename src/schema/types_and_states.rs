use bevy::prelude::*;
use strum_macros::{AsRefStr, EnumIter};
use crate::schema::config::{Plant};


// Типы и Состояния
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState { // Стадии загрузки
    #[default]
    Loading,
    Playing
}

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CurrentWorld { // Выбраный мир
    #[default]
    SunlitNursery,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TypeButton { // Тип кнопки
    TomatoButton,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlotState { // Состояние слота
    Locked,
    Empty,
    Occupied(Plant),
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TypePlant { // Тип растения
    Tomato,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum PlantStateGrowth {
    Seed(PlantStateUpdate),
    Sprout(PlantStateUpdate),
    Sapling(PlantStateUpdate),
    Mature(PlantStateUpdate),
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum PlantStateUpdate {
    Growth,
    Idle,
}

#[derive(Debug, Clone, Copy, EnumIter, AsRefStr, PartialEq)]
pub enum ResourceType {
    CatHappiness,
    Tomatoes,
    Cucumbers,
    Corn,
    Pumpkin,
}

// Список инвенторей
#[derive(Resource)]
pub struct GlobalInventory {
    pub sunlit_nursery_inv: [SlotState; 16],
}

#[derive(Resource, Default)]
pub struct Economy {
    pub storage: [f64; 5],
}

// Глобальные действия
#[derive(Resource, Default)]
pub struct DragItem { // Обьект курсора
    pub entity: Option<Entity>,
}
