use purrgress::cat_stage_manager::*;
use purrgress::cat_motion_blur::*;

use purrgress::cat_motion_blur::pandemonium_types::PurrFrameStage;
use purrgress_macros::{meowphosis, PurrStep};

use std::collections::HashMap;
use bevy::prelude::*;
use strum_macros::EnumIter; 


#[meowphosis]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep)]
pub enum CatStages {
    Idle,
    Walk,
    Run,
    InBox,
    CurledSleep,
    InHands,
    Petting,
    PurrChain(usize)
}

#[meowphosis]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PurrStep, EnumIter)]
pub enum PurrChain {
    RunChain,
    WalkChain,
}

pub enum CatAni {
    Idle(AniSub),
    Walk(AniSub),
    Run(AniSub),
    InBox(AniSub),
    CurledSleep(AniSub),
    InHands(AniSub),
    Petting(AniSub),
}

pub enum AniSub {
    Start,
    Loop,
    End,
}

pub enum CatName {
    Rooney
}

#[derive(Component)]
pub struct CatInfo {
    pub cat_name: CatName,
    pub cat_paramets: CatParamets,

    pub cat_manager: manager::StageManager<CatStages>,
    pub purr_chain_map: HashMap<PurrChain, CatStages>,

    pub cat_animator: memory_demonium::PurrAnimator<PurrFrameStage, PurrChain, CatAni>
}

pub struct CatParamets {
    pub cat_walk_speed: f32,
    pub cat_run_speed: f32,

    pub cat_stage_prioriti: PurrChain,
}

