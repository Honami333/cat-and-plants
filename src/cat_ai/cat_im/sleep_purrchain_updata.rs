use purrgress::cat_malloc::purr_train;
use purrgress::cat_malloc::train_types;

use purrgress::cat_malloc::train_types::PurrRule;
use purrgress::condition;

use bevy::prelude::*;

use crate::cat_ai::cat_dataset::*;


pub fn update_sleep_chain(
    cat: &mut CatInfo,
    time: &Time
) {
    let delta = time.delta_secs();

    let train = &mut cat.cat_train;

    update_sleep_chain_condition(train, delta);
}

fn update_sleep_chain_condition(
    train: &mut purr_train::PurrTrain<CatStages, train_types::StandardRules>,
    delta: f32,
) {
    if let Some(current) = train.get_current_mut()
        && let Some(timer) = current.rule.as_mut_rule::<condition::PurrTimer>()
            && matches!(current.carriage, CatStages::Idle(SLEEP_NAMED) | CatStages::CurledSleep(SLEEP_NAMED)) {

         timer.tick(delta);
    };
}
