use purrgress::cat_malloc::purr_train;
use purrgress::cat_malloc::train_types;

use purrgress::cat_malloc::train_types::PurrRule;
use purrgress::condition;

use bevy::prelude::*;

use crate::cat_ai::cat_dataset::*;

use super::ai;


pub fn update_run_chain(
    cat: &mut CatInfo,
    time: &Time
) {
    let delta = time.delta_secs();

    let run_speed = cat.cat_paramets.cat_run_speed;

    let train = &mut cat.cat_train;

    update_run_chain_condition(train, delta, run_speed);
}

fn update_run_chain_condition(
    train: &mut purr_train::PurrTrain<CatStages, train_types::StandardRules>,
    delta: f32,
    run_speed: f32
) {
    if let Some(current) = train.get_current_mut() {
        if let Some(timer) = current.rule.as_mut_rule::<condition::PurrTimer>()
            && matches!(current.carriage, CatStages::Idle(RUN_NAMED)) {

            timer.tick(delta);
        };

        if let Some(proximity) = current.rule.as_mut_rule::<condition::PurrProximity>()
            && matches!(current.carriage, CatStages::Run(RUN_NAMED)) {

            ai::update_proximity(proximity, run_speed);
        };
    };
}
