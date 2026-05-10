use crate::schema::config::{Upgrade, UpgradeLevel};
use crate::schema::types_and_states::{EGUISelectedCategories, ResourceType, UpgradeUID};

pub const UNLOCK_CUCUMBER: Upgrade = Upgrade {
    id: UpgradeUID::UnlockCucumber,
    icon: "🥒+",
    current_level: 0,
    levels: &[UC_LVL_1],
    dependencies: &[],
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 0),
};

pub const UC_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[50.0, 20.0],
    value: 1.0,
};

pub const UNLOCK_CORN: Upgrade = Upgrade {
    id: UpgradeUID::UnlockCorn,
    icon: "🌽+",
    current_level: 0,
    levels: &[UCORN_LVL_1],
    dependencies: &[UpgradeUID::UnlockCucumber],
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 1),
};

const UCORN_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Cucumbers],
    costs: &[600.0, 100.0],
    value: 1.0,
};

pub const UNLOCK_PUMPKIN: Upgrade = Upgrade {
    id: UpgradeUID::UnlockPumpkin,
    icon: "🎃+",
    current_level: 0,
    levels: &[UPUMP_LVL_1],
    dependencies: &[UpgradeUID::UnlockCorn],
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 2),
};
const UPUMP_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Corn],
    costs: &[3500.0, 250.0],
    value: 1.0,
};
