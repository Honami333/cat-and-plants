use crate::schema::{config::*, economy_inventory::*, global_inventory::*};
use bevy::prelude::*;

pub const TRADEWELL: TradeWell = TradeWell {
    well: &[
    (ResourceType::Tomatoes, 1.0),
    (ResourceType::Cucumbers, 2.0),
    (ResourceType::Corn, 8.0),
    (ResourceType::Pumpkin, 30.0)
    ],
};

pub const SN_SLOT_CFG: WorldSettingsSlot = WorldSettingsSlot {
    slot_start_pos: Vec2::new(-182.0, -22.0),
    step_x: Vec2::new(63.0, 31.5),
    step_y: Vec2::new(63.0, -31.5),
    slot_grid_scale: 4,
};

pub const PL_TOMATO: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 8.0,
    growth_rate: 1.0,
    gather_amount: 5.0,
    species_id: TypePlant::Tomato,
    slot_uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[0.0, 20.0, 70.0, 180.0],
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
    price: &[50.0, 120.0, 300.0, 600.0], 
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
    price: &[400.0, 800.0, 1500.0, 3000.0],
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
    price: &[1000.0, 2500.0, 5500.0, 12000.0],
    max_count: 4,
};

pub const SLOT_PRICES: SlotPrices = SlotPrices {
    prices: &[0.0, 0.0, 0.0, 0.0, 50.0, 100.0, 200.0, 350.0, 500.0, 750.0, 1000.0, 1350.0, 1800.0, 2400.0, 3200.0, 4500.0],
};

pub const SN_FIRST_PRESTIGE_COST: PrestigeCost = PrestigeCost {
    cost: &[
        (ResourceType::CatHappiness, 20000.0),
        (ResourceType::Tomatoes, 1200.0),
        (ResourceType::Cucumbers, 800.0),
        (ResourceType::Corn, 300.0),
        (ResourceType::Pumpkin, 50.0)
    ],
};