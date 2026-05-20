use crate::schema::config::{Upgrade, UpgradeLevel};
use crate::schema::types_and_states::*;

pub const FERTILE_SOIL: Upgrade = Upgrade {
    name: "Fertile Soil",
    description: "Enriches the garden beds with minerals. Increases crop yield with each level.",

    id: UpgradeUID::FertileSoil,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[SOIL_1, SOIL_2, SOIL_3],
    dependencies: &[],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::Global,
    grid_pos: (0, 0),
};

const SOIL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[80.0, 30.0],
    value: Some(2.0),
};

const SOIL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[600.0, 100.0, 40.0],
    value: Some(3.0),
};

const SOIL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[2000.0, 300.0, 150.0, 60.0],
    value: Some(4.0),
};

pub const GROWTH_SPEED: Upgrade = Upgrade {
    name: "Growth Catalysts",
    description: "A special watering formula that forces plants to develop faster. Reduces time to harvest",

    id: UpgradeUID::SelectiveBreeding,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[SPD_1, SPD_2, SPD_3],
    dependencies: &[],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::Global,
    grid_pos: (0, 1),
};

const SPD_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[120.0, 50.0],
    value: Some(1.2),
};

const SPD_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[600.0, 150.0, 50.0],
    value: Some(1.5),
};

const SPD_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Corn,
    ],
    costs: &[2500.0, 400.0, 150.0, 80.0],
    value: Some(2.0),
};

pub const JOY_BOOST: Upgrade = Upgrade {
    name: "Catnip Infusion",
    description: "A secret ingredient in the soil makes cats much happier when they visit your garden.",

    id: UpgradeUID::WholesaleSupply,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[JOY_1, JOY_2, JOY_3],
    dependencies: &[],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::Global,
    grid_pos: (0, 2),
};

const JOY_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[200.0, 80.0],
    value: Some(1.25),
};

const JOY_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[1000.0, 200.0, 80.0],
    value: Some(1.75),
};

const JOY_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Pumpkin,
    ],
    costs: &[5000.0, 500.0, 200.0, 50.0],
    value: Some(2.5),
};

pub const CARDBOARD_BOX: Upgrade = Upgrade {
    name: "Wholesale Supplies",
    description: "We've arranged for shipments in big boxes! Reduces the cost of all future upgrades.",

    id: UpgradeUID::CardboardBox,
    texture_stage: UpgradeStage::Locked,
    current_level: 0,
    levels: &[BOX_LVL_1, BOX_LVL_2, BOX_LVL_3],
    dependencies: &[
        UpgradeUID::FertileSoil,
        UpgradeUID::WholesaleSupply,
        UpgradeUID::SelectiveBreeding,
    ],
    location_prestige_req: (None, None),
    category: EGUISelectedCategories::Global,
    grid_pos: (3, 0),
};

const BOX_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[1000.0, 200.0],
    value: Some(0.95),
};

const BOX_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Cucumbers],
    costs: &[2000.0, 200.0],
    value: Some(0.9),
};

const BOX_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Corn],
    costs: &[4000.0, 200.0],
    value: Some(0.85),
};

// Стеклянная теплица: Повышает доход в локации Sunlit Nursery
// Полка для рассады: Позволяет хранить один слот «в запасе». что иметь возможность собирать уражай с какого либо растения на любой локации
// Селекция семян: +1 к gather_amount для кукурузы.
// Гормональный буст: +1 к gather_amount для тыквы
// Возможность перетаскивать растения
// Все виды суперс улучшений на каждое растение
