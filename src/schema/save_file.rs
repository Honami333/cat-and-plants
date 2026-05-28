use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use super::{upgrade_storege::*, global_inventory::*, economy_inventory::*, item_type_info::*, common::CurrentWorld, prestige::*};


#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SaveDataContainer {
    pub up_storege: UpgradeStorege,
    pub global_storege: GlobalInventory,
    pub eco_storege: Economy,
    pub iti_storege: ItemTypeInfo,
    pub world: CurrentWorld,
    pub prestige: PrestigeRoom,
}

#[derive(Resource, Clone, Copy, Default)]
pub struct SaveSlotInv {
    pub slot: [SaveSlot; 3],
    pub active_slot: Option<usize>,
    pub deleting_slot: Option<usize>,
}

#[derive(Clone, Copy, Default)]
pub struct SaveSlot {
    pub stage: SlotTextureState,
    pub click: usize,
    pub last_data_text: &'static str,
}


#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
pub enum SlotTextureState {
    #[default]
    Empty = 0,
    Occupied = 1,
}

pub fn default_static_slice<T>() -> &'static [T] {
    &[]
}

pub fn default_static_str() -> &'static str {
    ""
}