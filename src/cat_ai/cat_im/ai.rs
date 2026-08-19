use purrgress::cat_malloc::train_types::BufferMode;
use purrgress::condition;
use purrgress::types;

use bevy::prelude::*;

use rand::{Rng, rngs::ThreadRng};
use rand::seq::IteratorRandom;

use anyhow::{anyhow, Result};

use crate::cat_ai::cat_dataset::*;

use super::run_purrchain_updata;
use super::sleep_purrchain_updata;
use super::walk_purrchain_update;

const ACTION_CHANCE: f32 = 0.4;


pub fn update_stage_cat(
    mut cats: Query<&mut CatInfo>,
    time: Res<Time>,
) {
    let chain_list = [
        CatStages::RunChain, CatStages::WalkChain, CatStages::SleepChain
    ];

    let mut rng = rand::thread_rng();

    for mut cat in cats.iter_mut() {
        run_purrchain_updata::update_run_chain(&mut cat, &time);

        walk_purrchain_update::update_walk_chain(&mut cat, &time);

        sleep_purrchain_updata::update_sleep_chain(&mut cat, &time);

        let cat_train = &mut cat.cat_train;

        let event = cat_train.advance_train();

        if matches!(event, types::PurrEvent::Transition { .. }) { println!("{:?}", cat_train.get_line()); };
        if matches!(event, types::PurrEvent::Transition { .. }) { println!("{:?}", event); };

        if matches!(event, types::PurrEvent::Idle) { set_action(&mut cat, &mut rng, &chain_list); };
    };
}

pub fn update_proximity(proximity: &mut condition::PurrProximity, speed: f32) {
    let target_pos = proximity.get_target_pos();

    let pos = proximity.get_pos_mut();

    let sign_x = (target_pos.x - pos.x).signum();
    let sign_y = (target_pos.y - pos.y).signum();

    pos.x = (pos.x + speed * sign_x).clamp(pos.x.min(target_pos.x), pos.x.max(target_pos.x));
    pos.y = (pos.y + speed * sign_y).clamp(pos.y.min(target_pos.y), pos.y.max(target_pos.y));
}


pub fn set_action(
    cat: &mut CatInfo,
    rng: &mut ThreadRng,
    chain_list: &[CatStages],
) {
    let save_stage_prioriti = cat.cat_paramets.cat_stage_prioriti;


    let rand: f32 = rng.r#gen();

    let rng_action = if rand <= ACTION_CHANCE {
        save_stage_prioriti
    } else {
        *chain_list.iter().choose(rng).unwrap()
    };

    if let Err(error) = add_rand_action(cat, rng_action) {
        println!("{error}");
    };
}

fn add_rand_action(
    cat_train: &mut CatInfo,
    rng_action: CatStages,
) -> Result<()> {
    match rng_action {
        CatStages::RunChain => { 
            cat_train.train_siding.launch(CatStages::RunChain, BufferMode::Clear, &cat_train.train_route)?;
        },
        CatStages::SleepChain => { 
            cat_train.train_siding.launch(CatStages::SleepChain, BufferMode::Clear, &cat_train.train_route)?;
        },
        CatStages::WalkChain => { 
            cat_train.train_siding.launch(CatStages::WalkChain, BufferMode::Clear, &cat_train.train_route)?;
        },
        _ => return Err( anyhow!("CatStage not found") )
    };

    println!("{:?}", cat_train.train_siding.main_train);

    cat_train.cat_train.attach(&mut cat_train.train_siding);

    Ok(())
}