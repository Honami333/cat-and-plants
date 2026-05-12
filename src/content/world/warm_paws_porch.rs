use crate::schema::{config::*, types_and_states::*};
use bevy::prelude::*;

pub const WPP_DATA: ScaleBackground = ScaleBackground {
    wh: Vec2::new(640.0, 360.0),
};

pub const WPP_PLANT_RES: PlantResource = PlantResource {
    plant0: ResourceType::None,
    plant1: ResourceType::None,
    plant2: ResourceType::None,
    plant3: ResourceType::None,
    plant_icon0: "",
    plant_icon1: "",
    plant_icon2: "",
    plant_icon3: "",
};
