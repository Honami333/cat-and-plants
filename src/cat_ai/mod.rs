use bevy::prelude::*;

mod cat_chain_setup {
    pub mod run_setup;
    pub mod sleep_setup;
    pub mod walk_setup;
}
mod cat_im {
    pub mod ai;
    mod run_purrchain_updata;
    mod sleep_purrchain_updata;
    mod walk_purrchain_update;
}

mod cat_dataset;
mod rooney_dataset;


pub struct CatAIPlugin;

impl Plugin for CatAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, rooney_dataset::rooney_dataset_setup);

        app.add_systems(Update, cat_im::ai::update_stage_cat);
    }
}