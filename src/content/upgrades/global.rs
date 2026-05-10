use crate::schema::config::{Upgrade, UpgradeLevel};
use crate::schema::types_and_states::{EGUISelectedCategories, ResourceType, UpgradeUID};

pub const FERTILE_SOIL: Upgrade = Upgrade {
    id: UpgradeUID::FertileSoil,
    icon: "SOIL",
    current_level: 0,
    levels: &[SOIL_1, SOIL_2, SOIL_3],
    dependencies: &[],
    category: EGUISelectedCategories::Global,
    grid_pos: (0, 0),
};

const SOIL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[80.0, 30.0],
    value: 2.0,
};

const SOIL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[600.0, 100.0, 40.0],
    value: 3.0,
};

const SOIL_3: UpgradeLevel = UpgradeLevel {
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
    dependencies: &[],
    category: EGUISelectedCategories::Global,
    grid_pos: (0, 1),
};

const SPD_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[120.0, 50.0],
    value: 1.2,
};

const SPD_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[600.0, 150.0, 50.0],
    value: 1.5,
};

const SPD_3: UpgradeLevel = UpgradeLevel {
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
    dependencies: &[],
    category: EGUISelectedCategories::Global,
    grid_pos: (0, 2),
};

const JOY_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[200.0, 80.0],
    value: 1.25,
};

const JOY_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
    ],
    costs: &[1000.0, 200.0, 80.0],
    value: 1.75,
};

const JOY_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[
        ResourceType::CatHappiness,
        ResourceType::Tomatoes,
        ResourceType::Cucumbers,
        ResourceType::Pumpkin,
    ],
    costs: &[5000.0, 500.0, 200.0, 50.0],
    value: 2.5,
};

pub const CARDBOARD_BOX: Upgrade = Upgrade {
    id: UpgradeUID::CardboardBox,
    icon: "BOX",
    current_level: 0,
    levels: &[BOX_LVL_1, BOX_LVL_2, BOX_LVL_3],
    dependencies: &[
        UpgradeUID::FertileSoil,
        UpgradeUID::WholesaleSupply,
        UpgradeUID::SelectiveBreeding,
    ],
    category: EGUISelectedCategories::Global,
    grid_pos: (3, 0),
};

const BOX_LVL_1: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Tomatoes],
    costs: &[1000.0, 200.0],
    value: 0.95,
};

const BOX_LVL_2: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Cucumbers],
    costs: &[2000.0, 200.0],
    value: 0.9,
};

const BOX_LVL_3: UpgradeLevel = UpgradeLevel {
    resource_types: &[ResourceType::CatHappiness, ResourceType::Corn],
    costs: &[4000.0, 200.0],
    value: 0.85,
};

// Стеклянная теплица: Повышает доход в локации Sunlit Nursery
// Полка для рассады: Позволяет хранить один слот «в запасе». что иметь возможность собирать уражай с какого либо растения на любой локации
// Усиленное удобрение: +1 к gather_amount для томатов.
// Минеральные соли: +1 к gather_amount для огурцов.
// Селекция семян: +1 к gather_amount для кукурузы.
// Гормональный буст: +1 к gather_amount для тыквы
// Возможность перетаскивать растения
// Все виды суперс улучшений на каждое растение

// Информация о улучшении,
