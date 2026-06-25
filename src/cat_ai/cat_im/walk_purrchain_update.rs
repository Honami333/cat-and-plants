use purrgress::cat_stage_manager::*;

use purrgress::cat_stage_manager::condition::PurrCondition;

use bevy::prelude::*;

use crate::cat_ai::cat_dataset::*;


pub fn update_walk_chain(
    cat: &mut CatInfo,
    time: &Time
) {
    let delta = time.delta_secs();

    let walk_speed = cat.cat_paramets.cat_walk_speed;

    if let Some(run_chain) = cat.purr_chain_map.get(&PurrChain::RunChain).copied() {
        let manager = &mut cat.cat_manager;

        let raw_data = purrgress_macros::purr_rumble!(
            manager : run_chain,
            CatStages,
            update_walk_chain_condition : delta, walk_speed
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

fn update_walk_chain_condition(
    manager: &mut manager::StageManager<CatStages>,
    delta: f32,
    walk_speed: f32
) {
    if let Some(idle_timer) = manager.get_condition_mut::<condition::PurrTimer>(CatStages::Idle)
        && !idle_timer.is_finished() {
        
            idle_timer.tick(delta);
    };

    if let Some(walk_proximity) = manager.get_condition_mut::<condition::PurrProximity>(CatStages::Walk)
        && !walk_proximity.is_finished() {
        
        let target_pos = walk_proximity.get_target_pos();

        let pos = walk_proximity.get_pos_mut();

        let sign_x = (target_pos.x - pos.x).signum();
        let sign_y = (target_pos.y - pos.y).signum();

        pos.x = (pos.x + walk_speed * sign_x).clamp(pos.x.min(target_pos.x), pos.x.max(target_pos.x));
        pos.y = (pos.y + walk_speed * sign_y).clamp(pos.y.min(target_pos.y), pos.y.max(target_pos.y));
    };
}