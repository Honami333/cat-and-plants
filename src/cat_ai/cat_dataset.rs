use purrgress::cat_malloc::purr_train;
use purrgress::cat_malloc::train_route;
use purrgress::cat_malloc::train_siding;
use purrgress::cat_malloc::train_types;

use purrgress_macros::PurrStep;

use bevy::prelude::*;

pub const RUN_NAMED: usize = 0;
pub const WALK_NAMED: usize = 1;
pub const SLEEP_NAMED: usize = 2;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum CatStages {
    Idle(usize),
    Walk(usize),
    Run(usize),
    InBox(usize),
    CurledSleep(usize),
    InHands(usize),
    Petting(usize),
    RunChain,
    WalkChain,
    SleepChain,
}

pub enum CatName {
    Rooney
}

#[derive(Component)]
pub struct CatInfo {
    pub cat_name: CatName,
    pub cat_paramets: CatParamets,

    pub cat_train: purr_train::PurrTrain<CatStages, train_types::StandardRules>,
    pub train_route: train_route::PurrRoute<CatStages, train_types::StandardRules>,
    pub train_siding: train_siding::PurrSiding<CatStages, train_types::StandardRules>
}

pub struct CatParamets {
    pub cat_walk_speed: f32,
    pub cat_run_speed: f32,
    pub cat_sleep_time: f32,

    pub cat_stage_prioriti: CatStages,
}

