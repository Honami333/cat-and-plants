use crate::schema::{upgrade_storege::*, economy_inventory::ResourceType, hud::EGUISelectedCategories, common::CurrentWorld};


pub const UNLOCK_TOMATO: Upgrade = Upgrade {
    name: "unlock-tomato-name",
    description: "unlock-tomato-desc",

    id: UpgradeUID::UnlockTomato,
    texture_stage: UpgradeStage::Max,
    current_level: 1,
    levels: &[UT_LVL_1],
    dependencies: &[],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 0),
};

const UT_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[],
    costs: &[], 
    value: None,
};

pub const UNLOCK_CUCUMBER: Upgrade = Upgrade {
    name: "unlock-cucumber-name",
    description: "unlock-cucumber-desc",

    id: UpgradeUID::UnlockCucumber,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[UC_LVL_1],
    dependencies: &[],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (1, 0),
};

const UC_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[30.0, 15.0], 
    value: None,
};

pub const UNLOCK_CORN: Upgrade = Upgrade {
    name: "unlock-corn-name",
    description: "unlock-corn-desc",

    id: UpgradeUID::UnlockCorn,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[UCORN_LVL_1],
    dependencies: &[UpgradeUID::UnlockCucumber],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (2, 0),
};

const UCORN_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Cucumbers],
    costs: &[150.0, 60.0],
    value: None,
};

pub const UNLOCK_PUMPKIN: Upgrade = Upgrade {
    name: "unlock-pumpkin-name",
    description: "unlock-pumpkin-desc",

    id: UpgradeUID::UnlockPumpkin,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[UPUMP_LVL_1],
    dependencies: &[UpgradeUID::UnlockCorn],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (3, 0),
};

const UPUMP_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Corn],
    costs: &[400.0, 100.0],
    value: None,
};

pub const CONCENTRATED_NECTAR: Upgrade = Upgrade {
    name: "concentrated-nectar-name",
    description: "concentrated-nectar-desc",

    id: UpgradeUID::ConcentratedNectar,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[CN_LVL_1, CN_LVL_2, CN_LVL_3],
    dependencies: &[UpgradeUID::UnlockTomato],
    location_prestige_req: (Some(CurrentWorld::SunlitNursery), Some(1)),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 7),
};

const CN_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks, ResourceType::Tomatoes],
    costs: &[1.0, 300.0],
    value: Some(0.0001),
};

const CN_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks, ResourceType::Tomatoes],
    costs: &[3.0, 1000.0],
    value: Some(0.00015),
};

const CN_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::SunSparks, ResourceType::Tomatoes],
    costs: &[5.0, 5000.0],
    value: Some(0.0002),
};

pub const TOMATO_BOUNTY: Upgrade = Upgrade {
    name: "tomato-bounty-name",
    description: "tomato-bounty-desc",

    id: UpgradeUID::TomatoBounty,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[TB_LVL_1, TB_LVL_2, TB_LVL_3],
    dependencies: &[UpgradeUID::UnlockTomato],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 1),
};

const TB_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[80.0, 30.0],
    value: Some(1.15),
};

const TB_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[650.0, 200.0, 80.0],
    value: Some(1.30),
};

const TB_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Corn,
    ],
    costs: &[4000.0, 2000.0, 400.0],
    value: Some(1.5),
};


pub const TOMATO_GROWTH: Upgrade = Upgrade {
    name: "tomato-growth-name",
    description: "tomato-growth-desc",

    id: UpgradeUID::TomatoGrowth,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[TG_LVL_1, TG_LVL_2, TG_LVL_3],
    dependencies: &[UpgradeUID::TomatoBounty],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 2),
};

const TG_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[100.0, 50.0],
    value: Some(1.15),
};

const TG_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[900.0, 350.0, 180.0],
    value: Some(1.30),
};

const TG_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Corn,
    ],
    costs: &[5500.0, 2500.0, 650.0],
    value: Some(1.5),
};

pub const TOMATO_JOY: Upgrade = Upgrade {
    name: "tomato-joy-name",
    description: "tomato-joy-desc",

    id: UpgradeUID::TomatoJoy,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[TJ_LVL_1, TJ_LVL_2, TJ_LVL_3],
    dependencies: &[UpgradeUID::TomatoGrowth],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 3),
};

const TJ_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers
    ],
    costs: &[180.0, 120.0, 50.0],
    value: Some(1.15),
};

const TJ_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Corn
    ],
    costs: &[1200.0, 500.0, 300.0, 80.0],
    value: Some(1.30),
};

const TJ_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Pumpkin
    ],
    costs: &[1.0, 8000.0, 3500.0, 100.0],
    value: Some(1.5),
};


pub const CUCUMBER_BOUNTY: Upgrade = Upgrade {
    name: "cucumber-bounty-name",
    description: "cucumber-bounty-desc",

    id: UpgradeUID::CucumberBounty,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[CUB_LVL_1, CUB_LVL_2, CUB_LVL_3],
    dependencies: &[UpgradeUID::UnlockCucumber],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (1, 1),
};

const CUB_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Cucumbers],
    costs: &[150.0, 50.0],
    value: Some(1.15),
};

const CUB_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[800.0, 300.0, 100.0],
    value: Some(1.30),
};

const CUB_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[4500.0, 1200.0, 450.0],
    value: Some(1.5),
};


pub const CUCUMBER_GROWTH: Upgrade = Upgrade {
    name: "cucumber-growth-name",
    description: "cucumber-growth-desc",

    id: UpgradeUID::CucumberGrowth,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[CUG_LVL_1, CUG_LVL_2, CUG_LVL_3],
    dependencies: &[UpgradeUID::CucumberBounty],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (1, 2),
};

const CUG_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Cucumbers],
    costs: &[200.0, 120.0],
    value: Some(1.15),
};

const CUG_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[1300.0, 450.0, 180.0],
    value: Some(1.30),
};

const CUG_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[1.0, 6500.0, 1500.0, 650.0, 90.0],
    value: Some(1.5),
};

pub const CUCUMBER_JOY: Upgrade = Upgrade {
    name: "cucumber-joy-name",
    description: "cucumber-joy-desc",

    id: UpgradeUID::CucumberJoy,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[CUJ_LVL_1, CUJ_LVL_2, CUJ_LVL_3],
    dependencies: &[UpgradeUID::CucumberGrowth],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (1, 3),
};

const CUJ_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Corn
    ],
    costs: &[350.0, 180.0, 30.0],
    value: Some(1.15),
};

const CUJ_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Corn,
        ResourceType::Pumpkin
    ],
    costs: &[1500.0, 600.0, 300.0, 20.0],
    value: Some(1.30),
};

const CUJ_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Pumpkin
    ],
    costs: &[1.0, 10000.0, 2300.0, 120.0],
    value: Some(1.5),
};



pub const CORN_BOUNTY: Upgrade = Upgrade {
    name: "corn-bounty-name",
    description: "corn-bounty-desc",

    id: UpgradeUID::CornBounty,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[COB_LVL_1, COB_LVL_2, COB_LVL_3],
    dependencies: &[UpgradeUID::UnlockCorn],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (2, 1),
};

const COB_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Corn,
    ],
    costs: &[450.0, 50.0],
    value: Some(1.15),
};

const COB_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[2000.0, 200.0, 20.0],
    value: Some(1.30),
};

const COB_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin
    ],
    costs: &[1.0, 10000.0, 400.0, 80.0],
    value: Some(1.5),
};


pub const CORN_GROWTH: Upgrade = Upgrade {
    name: "corn-growth-name",
    description: "corn-growth-desc",

    id: UpgradeUID::CornGrowth,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[COG_LVL_1, COG_LVL_2, COG_LVL_3],
    dependencies: &[UpgradeUID::CornBounty],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (2, 2),
};

const COG_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType:: Corn],
    costs: &[800.0, 80.0],
    value: Some(1.15),
};

const COG_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[2400.0, 550.0, 120.0],
    value: Some(1.30),
};

const COG_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[1.0, 12000.0, 400.0, 45.0],
    value: Some(1.5),
};


pub const CORN_JOY: Upgrade = Upgrade {
    name: "corn-joy-name",
    description: "corn-joy-desc",

    id: UpgradeUID::CornJoy,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[COJ_LVL_1, COJ_LVL_2, COJ_LVL_3],
    dependencies: &[UpgradeUID::CornGrowth],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (2, 3),
};

const COJ_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Corn],
    costs: &[1000.0, 150.0],
    value: Some(1.15),
};

const COJ_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[3500.0, 450.0, 60.0],
    value: Some(1.30),
};

const COJ_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[1.0, 15000.0, 700.0, 150.0],
    value: Some(1.5),
};


pub const PUMPKIN_BOUNTY: Upgrade = Upgrade {
    name: "pumpkin-bounty-name",
    description: "pumpkin-bounty-desc",

    id: UpgradeUID::PumpkinBounty,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[PMB_LVL_1, PMB_LVL_2, PMB_LVL_3],
    dependencies: &[UpgradeUID::UnlockPumpkin],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (3, 1),
};

const PMB_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Pumpkin,
    ],
    costs: &[600.0, 20.0],
    value: Some(1.3),
};

const PMB_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[3500.0, 300.0, 50.0],
    value: Some(1.5),
};

const PMB_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin
    ],
    costs: &[1.0, 12000.0, 600.0, 150.0],
    value: Some(1.8),
};


pub const PUMPKIN_GROWTH: Upgrade = Upgrade {
    name: "pumpkin-growth-name",
    description: "pumpkin-growth-desc",

    id: UpgradeUID::PumpkinGrowth,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[PMG_LVL_1, PMG_LVL_2, PMG_LVL_3],
    dependencies: &[UpgradeUID::PumpkinBounty],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (3, 2),
};

const PMG_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType:: Pumpkin],
    costs: &[1200.0, 40.0],
    value: Some(1.15),
};

const PMG_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[5000.0, 500.0, 80.0],
    value: Some(1.30),
};

const PMG_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[1.0, 15000.0, 800.0, 180.0],
    value: Some(1.5),
};


pub const PUMPKIN_JOY: Upgrade = Upgrade {
    name: "pumpkin-joy-name",
    description: "pumpkin-joy-desc",

    id: UpgradeUID::PumpkinJoy,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[PMJ_LVL_1, PMJ_LVL_2, PMJ_LVL_3],
    dependencies: &[UpgradeUID::PumpkinGrowth],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (3, 3),
};

const PMJ_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Pumpkin],
    costs: &[2000.0, 80.0],
    value: Some(1.2),
};

const PMJ_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[6000.0, 800.0, 150.0],
    value: Some(1.4),
};

const PMJ_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::SunSparks,
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Corn,
        ResourceType::Pumpkin,
    ],
    costs: &[2.0, 20000.0, 8000.0, 4000.0, 1500.0, 300.0],
    value: Some(1.8),
};
