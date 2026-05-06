use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_egui::EguiPlugin;
use crate::schema::{types_and_states::*};
use std::time::Duration;
use ui::UiPlugin;

mod ui;
mod interaction;
mod lifecycle;
mod simulation;
mod visials;


pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(UiPlugin);

        app.add_systems(
            OnEnter(GameState::Playing),
            lifecycle::camera_spawn
        );

        app.add_systems(OnExit(CurrentWorld::WarmPawsPorch), lifecycle::cleanup_system);
        app.add_systems(OnExit(CurrentWorld::SunlitNursery), lifecycle::cleanup_system);


        app.add_systems(OnEnter(CurrentWorld::WarmPawsPorch), lifecycle::spawm_world_system);
        app.add_systems(OnEnter(CurrentWorld::SunlitNursery), lifecycle::spawm_world_system);
        
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

        app.add_observer(visials::sync_plant_state);
    }
}


