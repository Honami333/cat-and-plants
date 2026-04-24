pub mod sunlit_nursery {
    // Константы для мира Sunlit Nursery 
    use bevy::prelude::*;
    use crate::schema::{types_and_states::*, config::*};

    
    pub const SN_DATA: ScaleBackground = ScaleBackground { wh: Vec2::new(640.0, 360.0) };

    pub const SN_SLOT_CFG: WorldSettingsSlot = WorldSettingsSlot {
        slot_start_pos: Vec2::new(-182.0, -22.0),
        step_x: Vec2::new(63.0, 31.5),
        step_y: Vec2::new(63.0, -31.5),
        slot_grid_scale: 4,
    };

    pub const BUT_TOMATO_CFG: ButtonCFG = ButtonCFG {
        pos: Vec2::new(224.0, -156.0),
        _text: "TOMATO",
         b_type: TypeButton::TomatoButton,
    };
    

    pub const PL_TOMATO: Plant = Plant {
        growth_score: 0,
        growth_thereshold: 10,
        growth_rate: 1,
        gather_amount: 5.0,
        species_id: TypePlant::Tomato,
        slot_uid: 0,
        state: PlantStateGrowth::Seed(PlantStateUpdate::Idle),
    };


    pub const SN_PLANT_RES: PlantResource = PlantResource {
        plant0: ResourceType::Tomatoes,
        plant1: ResourceType::Cucumbers,
        plant2: ResourceType::Corn,
        plant3: ResourceType::Pumpkin,
        plant_icon0: "🍅",
        plant_icon1: "🥒",
        plant_icon2: "🌽",
        plant_icon3: "🎃",
    };
}

pub mod warm_paws_porch {
    use bevy::prelude::*;
    use crate::schema::{config::*};

    pub const WPP_DATA: ScaleBackground = ScaleBackground { wh: Vec2::new(640.0, 360.0) };
}

