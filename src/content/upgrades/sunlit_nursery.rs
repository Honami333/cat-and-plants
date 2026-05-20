use crate::schema::config::{Upgrade, UpgradeLevel};
use crate::schema::types_and_states::*;


pub const UNLOCK_TOMATO: Upgrade = Upgrade {
    name: "Juicy Red Ball",
    description: "Your starting crop! Cats adore these bright red treats for their rich flavor and how fun they are to roll around.",

    id: UpgradeUID::UnlockTomato,
    texture_stage: UpgradeStage::Max,
    current_level: 0,
    levels: &[],
    dependencies: &[],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::SunlitNursery,
    grid_pos: (0, 0),
};

pub const UNLOCK_CUCUMBER: Upgrade = Upgrade {
    name: "Crunchy Snack",
    description: "Allows you to grow fresh cucumbers. Many cats love them for their juiciness and fun crunch!",

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
    name: "Sweet Kernels",
    description: "Unlocks corn. These yellow kernels are a real treat that will make the cats purr with delight.",

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
    name: "Festive Feast",
    description: "Allows you to feed the cats hearty pumpkins. It's the healthiest and grandest dish on your menu!",

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
    name: "Concentrated Nectar",
    description: "Unlocks the Tomato's special trait. Constant care squeezes out the best! Each harvest click builds up a permanent cumulative yield bonus for all crops, and applies a x2 stronger effect specifically for Tomatoes.",

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
    name: "Heavy Vines",
    description: "Strengthens the tomato vines so they can hold way more fruit. Every harvest yields a much larger amount of juicy red balls.",

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
    name: "Greenhouse Warmth",
    description: "Creates the perfect cozy climate for your tomatoes. The sprouts absorb heat better and mature much faster.",

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
    name: "Gourmet Puree",
    description: "Teaches you how to mash tomatoes into a smooth, delicious treat. Feeding this to your kittens fills them with ultimate happiness.",

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
    name: "Crisp Rows",
    description: "Improves cucumber trellises for optimal layout. Every harvest yields a much larger amount of crunchy snacks.",

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
    name: "Moisture Control",
    description: "Sets up automatic misting system for hydration. The cucumber sprouts absorb water better and mature much faster.",

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
    name: "Chilled Slices",
    description: "Teaches you how to serve refreshing cold cucumber snacks. Feeding this to your kittens fills them with ultimate happiness.",

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

