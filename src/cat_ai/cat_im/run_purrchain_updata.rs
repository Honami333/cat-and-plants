use purrgress::cat_stage_manager::*;

use purrgress::cat_stage_manager::condition::PurrCondition;

use bevy::prelude::*;

use crate::cat_ai::cat_dataset::*;


pub fn update_run_chain(
    cat: &mut CatInfo,
    time: &Time
) {
    let delta = time.delta_secs();

    let run_speed = cat.cat_paramets.cat_run_speed;

    if let Some(run_chain) = cat.purr_chain_map.get(&PurrChain::RunChain).copied() {
        let manager = &mut cat.cat_manager;

        let raw_data = purrgress_macros::purr_rumble!(
            manager : run_chain,
            CatStages,
            update_run_chain_condition : delta, run_speed
        );

        if let Some(data) = raw_data {
            match data {
                manager_types::PurrEvent::Idle => (),
                manager_types::PurrEvent::Running(stage) => println!("Stage {:?}", stage),
                manager_types::PurrEvent::Transition { from, to } => {
                    println!("CurStage {:?}", from);

                    if let Some(to) = to {
                        println!("NextStage {:?}", to);
                    };
                },
            };
        };
    };
}

fn update_run_chain_condition(
    manager: &mut manager::StageManager<CatStages>,
    delta: f32,
    run_speed: f32
) {
    if let Some(idle_timer) = manager.get_condition_mut::<condition::PurrTimer>(CatStages::Idle)
        && !idle_timer.is_finished() {
        
            idle_timer.tick(delta);
    };

    if let Some(run_proximity) = manager.get_condition_mut::<condition::PurrProximity>(CatStages::Run)
        && !run_proximity.is_finished() {
        
        let target_pos = run_proximity.get_target_pos();

        let pos = run_proximity.get_pos_mut();

        let sign_x = (target_pos.x - pos.x).signum();
        let sign_y = (target_pos.y - pos.y).signum();

        pos.x = (pos.x + run_speed * sign_x).clamp(pos.x.min(target_pos.x), pos.x.max(target_pos.x));
        pos.y = (pos.y + run_speed * sign_y).clamp(pos.y.min(target_pos.y), pos.y.max(target_pos.y));
    };
}