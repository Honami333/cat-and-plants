use crate::schema::config::{Upgrade, UpgradeLevel};
use crate::schema::types_and_states::*;

pub const UNLOCK_CUCUMBER: Upgrade = Upgrade {
    name: "Crunchy Snack",
    description: "Allows you to grow fresh cucumbers. Many cats love them for their juiciness and fun crunch!",

    id: UpgradeUID::UnlockCucumber,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[UC_LVL_1],
    dependencies: &[],
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 0),
};

pub const UC_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[30.0, 15.0], 
    value: None,
};

pub const UNLOCK_CORN: Upgrade = Upgrade {
    name: "Sweet Kernels",
    description: "Unlocks corn. These yellow kernels are a real treat that will make the cats purr with delight.",

    id: UpgradeUID::UnlockCorn,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[UCORN_LVL_1],
    dependencies: &[UpgradeUID::UnlockCucumber],
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 1),
};

const UCORN_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Cucumbers],
    costs: &[150.0, 60.0],
    value: None,
};

pub const UNLOCK_PUMPKIN: Upgrade = Upgrade {
    name: "Festive Feast",
    description: "Allows you to feed the cats hearty pumpkins. It's the healthiest and grandest dish on your menu!",

    id: UpgradeUID::UnlockPumpkin,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[UPUMP_LVL_1],
    dependencies: &[UpgradeUID::UnlockCorn],
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 2),
};
const UPUMP_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Corn],
    costs: &[400.0, 100.0],
    value: None,
};
