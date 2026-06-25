use std::collections::HashMap;
use bevy::prelude::*;

use super::cat_dataset::*;

use super::cat_chain_setup::*;

const CAT_WALK_SPEED: f32 = 0.35;
const CAT_RUN_SPEED: f32 = 0.8;


pub fn rooney_dataset_setup(
    mut commands: Commands,
) {
    let cat_manager = CatStages::meowphosis_manager();
    let purr_chain_map = HashMap::new();

    // run_setup::run_purr_chain(&mut cat_manager, &mut purr_chain_map);

    // walk_setup::walk_purr_chain(&mut cat_manager, &mut purr_chain_map);
    
    let run_chain_ani= run_setup::cat_run_ani_setup();
    
    let walk_chain_ani= walk_setup::cat_walk_ani_setup();

    let animator_meta_data = purrgress_macros::abyssal_grimoire!(
        !!PurrChain : <
            run_chain_ani,
            walk_chain_ani
        >
    );

    let cat_paramets = CatParamets {
        cat_walk_speed: CAT_WALK_SPEED,
        cat_run_speed: CAT_RUN_SPEED,

        cat_stage_prioriti: PurrChain::RunChain,
    };

    let rooney = CatInfo {
        cat_name: CatName::Rooney,
        cat_paramets,

        cat_manager,
        purr_chain_map,
        
        cat_animator: animator_meta_data
    };

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 20.0),
        rooney,
    ));
}
