use purrgress::cat_stage_manager::*;

use bevy::prelude::*;

use crate::cat_ai::cat_dataset::*;

use super::run_purrchain_updata;
use super::walk_purrchain_update;


pub fn update_stage_cat(
    mut cats: Query<&mut CatInfo>,
    time: Res<Time>,
) {
    for mut cat in cats.iter_mut() {
        run_purrchain_updata::update_run_chain(&mut cat, &time);

        walk_purrchain_update::update_walk_chain(&mut cat, &time);

        let cat_manager = &mut cat.cat_manager;

        match cat_manager.update() {
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
}
