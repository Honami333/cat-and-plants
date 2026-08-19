use purrgress::cat_malloc::train_design;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_types;
use purrgress::condition;

use purrgress::cat_malloc::train_types::{StandardRules, BufferMode};
use bevy::prelude::*;

use anyhow::Result;

use crate::cat_ai::cat_dataset::*;


pub fn run_purr_chain(
    train_route: &mut train_route::PurrRoute<CatStages, train_types::StandardRules>,
    idle_timer: condition::PurrTimer,
    run_proximity: condition::PurrProximity,
) -> Result<()> {
    let idle_condition = StandardRules::Timer(idle_timer);
    let run_condition = StandardRules::Proximity(run_proximity);

    let mut train_design = train_design::PurrDesign::new();

    train_design.single(CatStages::Idle(RUN_NAMED), idle_condition);

    train_design.single(CatStages::Run(RUN_NAMED), run_condition);

    let run_chain_design = Some(vec![CatStages::Idle(RUN_NAMED), CatStages::Run(RUN_NAMED)]);

    let run_chain_box = train_design::DesignBox::new(
        StandardRules::instant(),
        run_chain_design
    );
    
    train_design.chain(CatStages::RunChain, run_chain_box);

    train_route.construct_schedule(&train_design, BufferMode::Keep)?;

    Ok(())
}