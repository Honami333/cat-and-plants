use crate::schema::config::{Upgrade, UpgradeLevel};
use crate::schema::types_and_states::{ResourceType, UpgradeUID};

pub const FERTILE_SOIL: Upgrade = Upgrade {
    id: UpgradeUID::FertileSoil,
    icon: "SOIL",
    current_level: 0,
    levels: &[SOIL_1, SOIL_2, SOIL_3],
};

pub const SOIL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[80.0, 30.0],
    value: 2.0,
};

pub const SOIL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[600.0, 100.0, 40.0],
    value: 3.0,
};

pub const SOIL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[2000.0, 300.0, 150.0, 60.0],
    value: 4.0,
};

pub const GROWTH_SPEED: Upgrade = Upgrade {
    id: UpgradeUID::SelectiveBreeding,
    icon: "SPD",
    current_level: 0,
    levels: &[SPD_1, SPD_2, SPD_3],
};

pub const SPD_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[120.0, 50.0],
    value: 1.2,
};

pub const SPD_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[600.0, 150.0, 50.0],
    value: 1.5,
};

pub const SPD_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[250.0, 400.0, 150.0, 80.0],
    value: 2.0,
};

pub const JOY_BOOST: Upgrade = Upgrade {
    id: UpgradeUID::WholesaleSupply,
    icon: "JOY",
    current_level: 0,
    levels: &[JOY_1, JOY_2, JOY_3],
};

pub const JOY_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[200.0, 80.0],
    value: 1.25,
};

pub const JOY_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[1000.0, 200.0, 80.0],
    value: 1.75,
};

pub const JOY_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Pumpkin,
    ],
    costs: &[5000.0, 500.0, 200.0, 50.0],
    value: 2.5,
};
