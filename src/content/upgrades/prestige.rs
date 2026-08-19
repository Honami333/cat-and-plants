use crate::schema::{upgrade_storege::*, economy_inventory::ResourceType, hud::EGUISelectedCategories, common::CurrentWorld};


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

pub const STATIC_SHOCKWAVE : Upgrade = Upgrade {
    name: "static-shockwave-name",
    description: "static-shockwave-desc",

    id: UpgradeUID::StaticShockWave,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[STSHW_1],
    dependencies: &[UpgradeUID::PurrProfit, UpgradeUID::OverBlooming],
    location_prestige_req: (Some(CurrentWorld::SunlitNursery), Some(1)),
    category: EGUISelectedCategories::Sparcks,
    grid_pos: (2, 3),
}; 

const STSHW_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks],
    costs: &[2.0],
    value: Some(0.01),
};