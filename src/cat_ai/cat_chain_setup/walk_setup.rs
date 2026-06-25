use purrgress::cat_stage_manager::*;
use purrgress::cat_motion_blur::*;

use purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage;

use std::collections::HashMap;
use bevy::prelude::*;

use crate::cat_ai::cat_dataset::*;


pub fn walk_purr_chain(
    cat_manager: &mut manager::StageManager<CatStages>,
    purr_chain_map: &mut HashMap<PurrChain, CatStages>,
) {
    let idle_condition = condition::PurrTimer::default();
    let walk_condition = condition::PurrProximity::default();

    let run_purr_chain = purrgress_macros::new_purr_chain!(
        cat_manager,
        CatStages,
        CatStages::Idle : idle_condition =>
        CatStages::Walk : walk_condition
    );

    purr_chain_map.insert(PurrChain::WalkChain, run_purr_chain);
}

pub fn cat_walk_ani_setup() -> (PurrChain, manager::StageManager<PurrFrameStage>, memory_demonium::PurrAnimateMetaData<CatAni>) {
    purrgress_macros::purr_pandemonium!(
        !!PurrChain::WalkChain : <
            CatAni::Idle(AniSub::Start), [1, 1] => 
            CatAni::Idle(AniSub::Loop), [1, 1] => 
            CatAni::Idle(AniSub::End), [1, 1] =>
            CatAni::Walk(AniSub::Start), [1, 1] => 
            CatAni::Walk(AniSub::Loop), [1, 1] => 
            CatAni::Walk(AniSub::End), [1, 1]
        >
    )
}