use purrgress::cat_stage_manager::manager::StageManager;
use purrgress::cat_stage_manager::*;

use bevy::prelude::*;

use crate::cat_ai::cat_dataset::*;
use crate::cat_ai::cat_chain_setup::walk_setup;
use crate::cat_ai::cat_chain_setup::run_setup;

use rand::Rng;
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use std::collections::HashMap;

const ACTION_CHANCE: f32 = 0.4;


pub fn set_action(
    mut cats: Query<&mut CatInfo>,
) {
    let mut rng = rand::thread_rng();

    for mut cat in cats.iter_mut() {
        let save_stage_prioriti = cat.cat_paramets.cat_stage_prioriti;

        let mut purr_chain_map = cat.purr_chain_map.clone();

        let cat_manager = &mut cat.cat_manager;

        if cat_manager.len_vec_query() < 3 {
            let rand: f32 = rng.r#gen();

            let add_action = if rand <= ACTION_CHANCE {
                save_stage_prioriti
            } else {
                PurrChain::iter().choose(&mut rng).unwrap()
            };

            add_rand_action(cat_manager, &mut purr_chain_map, add_action);
        };

        cat.purr_chain_map = purr_chain_map;
    };
}

pub fn add_rand_action(
    cat_manager: &mut StageManager<CatStages>,
    purr_chain_map: &mut HashMap<PurrChain, CatStages>,
    add_action: PurrChain,
) {
    match add_action {
        PurrChain::WalkChain => {
            walk_setup::walk_purr_chain(cat_manager, purr_chain_map);

            let run_chain = purr_chain_map.get(&PurrChain::RunChain).copied();

            if let Some(run_chain) = run_chain {
                add_to_query(cat_manager, run_chain, CatStages::Run);
            };
        },
        PurrChain::RunChain => {
            run_setup::run_purr_chain(cat_manager, purr_chain_map);

            let walk_chain = purr_chain_map.get(&PurrChain::WalkChain).copied();

            if let Some(walk_chain) = walk_chain {
                add_to_query(cat_manager, walk_chain, CatStages::Walk);
            };
        },
    };
}

pub fn add_to_query(
    cat_manager: &mut StageManager<CatStages>,
    sub_stage: CatStages,
    push_stage: CatStages
) {
    purrgress_macros::purr_tentacle!(
        cat_manager : sub_stage,
        CatStages,
        manager_types::PurrAction::Push : push_stage,
        !manager_types::DuplicatePolicy::KeepAll
    );

    purrgress_macros::purr_pounce!(
        cat_manager : sub_stage,
        CatStages,
        manager_types::PurrAction::Push,
        !manager_types::DuplicatePolicy::KeepAll
    );
}