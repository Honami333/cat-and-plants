use crate::schema::{config::*, economy_inventory::*, global_inventory::*};
use bevy::prelude::*;

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
    uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[0.0, 20.0, 70.0, 180.0],
    max_count: 4,
    plant_ability: PlantAbility { ability_type: AbilityType::Global, data: PlantAbilityData::TomatoClickCombo { combo: 0 } }
};

pub const PL_CUCUMBER: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 10.0,
    growth_rate: 1.0,
    gather_amount: 5.0,
    species_id: TypePlant::Cucumber,
    slot_uid: 0,
    uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[50.0, 120.0, 300.0, 600.0], 
    max_count: 4,
    plant_ability: PlantAbility { ability_type: AbilityType::None, data: PlantAbilityData::None }
};

pub const PL_CORN: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 15.0,
    growth_rate: 1.0,
    gather_amount: 3.0,
    species_id: TypePlant::Corn,
    slot_uid: 0,
    uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[400.0, 800.0, 1500.0, 3000.0],
    max_count: 4,
    plant_ability: PlantAbility { ability_type: AbilityType::Single, data: PlantAbilityData::CornBoomHarvet { max_boom: 0, current_boom: 0, neighbours: [None; 4] } }
};

pub const PL_PUMPKIN: Plant = Plant {
    growth_score: 0.0,
    growth_thereshold: 20.0,
    growth_rate: 1.0,
    gather_amount: 1.0,
    species_id: TypePlant::Pumpkin,
    slot_uid: 0,
    uid: 0,
    state: PlantStateGrowth::Seed,
    price: &[1000.0, 2500.0, 5500.0, 12000.0],
    max_count: 4,
    plant_ability: PlantAbility { ability_type: AbilityType::None, data: PlantAbilityData::None }
};

pub const SLOT_PRICES: SlotPrices = SlotPrices {
    prices: &[0.0, 0.0, 0.0, 1000.0, 2000.0, 4000.0, 8000.0, 14000.0, 20000.0, 35000.0, 45000.0, 60000.0, 80000.0, 105000.0, 140000.0],
};

pub const SG_FIRST_PRESTIGE_COST: PrestigeCost = PrestigeCost {
    cost: &[
        (ResourceType::CatHappiness, 200000.0),
        (ResourceType::CatnipBall, 1500.0),
        (ResourceType::AmberBerry, 800.0),
        (ResourceType::HoveringCherries, 650.0),
        (ResourceType::FlowerMilkSyrup, 300.0),
        (ResourceType::PowerRoot, 80.0)
    ],
};