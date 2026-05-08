use crate::schema::types_and_states::TradeWell;

pub const TRADEWELL: TradeWell = TradeWell {
    well: [1.0, 2.0, 5.0, 20.0, 0.0],
};

// Константы для мира Sunlit Nursery
use crate::schema::{config::*, types_and_states::*};
use bevy::prelude::*;

pub const SN_DATA: ScaleBackground = ScaleBackground {
    wh: Vec2::new(640.0, 360.0),
};

pub const SN_SLOT_CFG: WorldSettingsSlot = WorldSettingsSlot {
    slot_start_pos: Vec2::new(-182.0, -22.0),
    step_x: Vec2::new(63.0, 31.5),
    step_y: Vec2::new(63.0, -31.5),
    slot_grid_scale: 4,
};

pub const BUT_TOMATO_CFG: ButtonCFG = ButtonCFG {
    pos: Vec2::new(224.0, -156.0),
    _text: "TOMATO",
    b_type: TypeButton::TomatoButton,
    text_pos: Vec2::new(65.0, 0.0),
};

pub const BUT_CUCUMBER_CFG: ButtonCFG = ButtonCFG {
    pos: Vec2::new(224.0, -108.0),
    _text: "CUCUMBER",
    b_type: TypeButton::CucumberButton,
    text_pos: Vec2::new(65.0, 0.0),
};

pub const BUT_CORN_CFG: ButtonCFG = ButtonCFG {
    pos: Vec2::new(-224.0, -156.0),
    _text: "CORN",
    b_type: TypeButton::CornButton,
    text_pos: Vec2::new(65.0, 0.0),
};

pub const BUT_PUMPKIN_CFG: ButtonCFG = ButtonCFG {
    pos: Vec2::new(-224.0, -108.0),
    _text: "PUMPKIN",
    b_type: TypeButton::PumpkinButton,
    text_pos: Vec2::new(65.0, 0.0),
};

pub const BUT_SLOTSLOCK_CFG: ButtonCFG = ButtonCFG {
    pos: Vec2::new(272.0, -60.0),
    _text: "Slots unlocking",
    b_type: TypeButton::SlotsUnLocking,
    text_pos: Vec2::new(17.0, 0.0),
};

pub const PL_TOMATO: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 10.0,
    growth_rate: 1.0,
    gather_amount: 5.0,
    species_id: TypePlant::Tomato,
    slot_uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[10.0, 20.0, 70.0, 180.0],
    max_count: 4,
};

pub const PL_CUCUMBER: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 10.0,
    growth_rate: 1.0,
    gather_amount: 5.0,
    species_id: TypePlant::Cucumber,
    slot_uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[250.0, 500.0, 900.0, 1500.0],
    max_count: 4,
};

pub const PL_CORN: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 15.0,
    growth_rate: 1.0,
    gather_amount: 3.0,
    species_id: TypePlant::Corn,
    slot_uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[1800.0, 3000.0, 4500.0, 6500.0],
    max_count: 4,
};

pub const PL_PUMPKIN: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 20.0,
    growth_rate: 1.0,
    gather_amount: 1.0,
    species_id: TypePlant::Pumpkin,
    slot_uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[8000.0, 12000.0, 18000.0, 25000.0],
    max_count: 4,
};

pub const SN_PLANT_RES: PlantResource = PlantResource {
    plant0: ResourceType::Tomatoes,
    plant1: ResourceType::Cucumbers,
    plant2: ResourceType::Corn,
    plant3: ResourceType::Pumpkin,
    plant_icon0: "🍅",
    plant_icon1: "🥒",
    plant_icon2: "🌽",
    plant_icon3: "🎃",
};

pub const SLOT_PRICES: SlotPrices = SlotPrices {
    prices: &[
        50.0, 100.0, 200.0, 350.0, 500.0, 750.0, 1000.0, 1350.0, 1800.0, 2400.0, 3200.0, 4500.0,
    ],
};

// pub const UNLOCK_PRICES: [f64; 3] = [
//     50.0,   // Разблокировать Огурец
//     600.0,  // Разблокировать Кукурузу
//     3500.0, // Разблокировать Тыкву
// ];
