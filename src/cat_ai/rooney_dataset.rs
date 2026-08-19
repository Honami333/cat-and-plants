use purrgress::cat_malloc::purr_train;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_types;
use purrgress::cat_malloc::train_siding;
use purrgress::condition;

use bevy::prelude::*;

use anyhow::Result;

use super::cat_dataset::*;
use super::cat_chain_setup;

const CAT_WALK_SPEED: f32 = 0.35;
const CAT_RUN_SPEED: f32 = 0.8;
const CAT_SLEEP_TIME: f32 = 20.0;


pub fn rooney_dataset_setup(
    mut commands: Commands,
) {
    let cat_train = purr_train::PurrTrain::new();

    let mut train_route = train_route::PurrRoute::new();

    let train_siding = train_siding::PurrSiding::new();

    let cat_paramets = CatParamets {
        cat_walk_speed: CAT_WALK_SPEED,
        cat_run_speed: CAT_RUN_SPEED,
        cat_sleep_time: CAT_SLEEP_TIME,

        cat_stage_prioriti: CatStages::RunChain
    };

    if let Err(error) = setup_action(&mut train_route, &cat_paramets) {
        println!("{error}")
    };

    let rooney = CatInfo {
        cat_name: CatName::Rooney,
        cat_paramets,

        cat_train,
        train_route,
        train_siding
    };

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 20.0),
        rooney,
    ));
}


fn setup_action(
    train_route: &mut train_route::PurrRoute<CatStages, train_types::StandardRules>,
    cat_paramets: &CatParamets,
) -> Result<()> {
    let idle_timer = condition::PurrTimer::new(1.0);
    let run_proximity = condition::PurrProximity::default();

    cat_chain_setup::run_setup::run_purr_chain(train_route, idle_timer, run_proximity)?;

    let idle_timer = condition::PurrTimer::new(1.5);
    let sleep_timer = condition::PurrTimer::new(cat_paramets.cat_sleep_time);

    cat_chain_setup::sleep_setup::sleep_purr_chain(train_route, idle_timer, sleep_timer)?;

    let idle_timer = condition::PurrTimer::new(1.5);
    let walk_proximity = condition::PurrProximity::default();

    cat_chain_setup::walk_setup::walk_purr_chain(train_route, idle_timer, walk_proximity)?;

    Ok(())
}