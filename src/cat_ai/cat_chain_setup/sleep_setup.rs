use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_types;
use purrgress::condition;

use purrgress::cat_malloc::train_types::{StandardRules, BufferMode};
use bevy::prelude::*;

use anyhow::Result;

use crate::cat_ai::cat_dataset::*;


pub fn sleep_purr_chain(
    train_route: &mut train_route::PurrRoute<CatStages, train_types::StandardRules>,
    idle_timer: condition::PurrTimer,
    sleep_timer: condition::PurrTimer,
) -> Result<()> {
    let idle_condition = StandardRules::Timer(idle_timer);
    let sleep_condition = StandardRules::Timer(sleep_timer);

    let mut train_design = train_design::PurrDesign::new();

    train_design.single(CatStages::Idle(SLEEP_NAMED), idle_condition);

    train_design.single(CatStages::CurledSleep(SLEEP_NAMED), sleep_condition);

    let sleep_chain_design = Some(vec![CatStages::Idle(SLEEP_NAMED), CatStages::CurledSleep(SLEEP_NAMED)]);

    let sleep_chain_box = train_design::DesignBox::new(
        StandardRules::instant(),
        sleep_chain_design
    );
    
    train_design.chain(CatStages::SleepChain, sleep_chain_box);

    train_route.construct_schedule(&train_design, BufferMode::Keep)?;

    Ok(())
}