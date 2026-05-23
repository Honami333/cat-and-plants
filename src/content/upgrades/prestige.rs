use crate::schema::config::{Upgrade, UpgradeLevel};
use crate::schema::types_and_states::*;


pub const PURR_PROFIT : Upgrade = Upgrade {
    name: "purr-profit-name",
    description: "purr-profit-desc",

    id: UpgradeUID::PurrProfit,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[PROFIT_1, PROFIT_2],
    dependencies: &[],
    location_prestige_req: (Some(CurrentWorld::SunlitNursery), Some(1)),
    category: EGUISelectedCategories::Sparcks,
    grid_pos: (1, 2),
};

const PROFIT_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks],
    costs: &[0.0],
    value: Some(0.25),
};

const PROFIT_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks],
    costs: &[3.0],
    value: Some(0.35),
};


pub const OVER_BLOOMING : Upgrade = Upgrade {
    name: "over-blooming-name",
    description: "over-blooming-desc",

    id: UpgradeUID::OverBlooming,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[BLOOM_1, BLOOM_2],
    dependencies: &[],
    location_prestige_req: (Some(CurrentWorld::SunlitNursery), Some(1)),
    category: EGUISelectedCategories::Sparcks,
    grid_pos: (1, 5),
};

const BLOOM_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks],
    costs: &[0.0],
    value: Some(0.3),
};

const BLOOM_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks],
    costs: &[3.0],
    value: Some(0.4),
};
