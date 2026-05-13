use crate::schema::{config::{SaveSlot, Upgrade}, types_and_states::*};
use bevy::{math::f64, platform::collections::HashMap, prelude::*};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct SaveDataContainer {
    pub up_storege: UpgradeStorege,
    pub global_storege: GlobalInventory,
    pub eco_storege: Economy,
    pub cit_storege: CountItemType,
}

// Список инвенторей
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct UpgradeStorege {
    pub global: HashMap<(usize, usize), Upgrade>,
    pub sunlit_nursery: HashMap<(usize, usize), Upgrade>,
}

#[derive(Resource, Clone, Copy, Serialize, Deserialize)]
pub struct GlobalInventory {
    pub sunlit_nursery_inv: [SlotState; 16],
}

#[derive(Resource, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Economy {
    pub storage: [f64; 6],
}

#[derive(Resource, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CountItemType {
    pub sunlit_nursery_inv: [usize; 4],
}

#[derive(Resource, Clone, Copy, Default)]
pub struct SaveSlotInv {
    pub slot: [SaveSlot; 3],
    pub active_slot: Option<usize>,
    pub deleting_slot: Option<usize>,
}
