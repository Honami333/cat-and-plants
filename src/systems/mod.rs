use bevy::{prelude::*, time::common_conditions::on_timer};
use crate::schema::{types_and_states::*};
use std::time::Duration;

mod interaction;
mod lifecycle;
mod simulation;
mod visials;


pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            lifecycle::spawm_world_system);
        
        app.add_systems(
            Update, (
            visials::update_plant_appearance,
            visials::update_scene_scale,
            visials::sync_inventory_visuals,
            visials::animate_counters,
            visials::update_resourse_text,
            visials::grag_item_anim_and_zsort,
            visials::shader_animation,

            interaction::state_dragg_item,
            
            simulation::plant_growth.run_if(on_timer(Duration::from_secs(1))),
        )
        .run_if(in_state(GameState::Playing)));

        app.add_observer(interaction::end_drag_item);
        app.add_observer(interaction::start_drag_item);
        app.add_observer(interaction::button_check);
        app.add_observer(interaction::harvest);
    }
}


