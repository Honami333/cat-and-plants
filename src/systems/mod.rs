
use crate::schema::types_and_states::*;
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_egui::EguiPlugin;
use std::time::Duration;
use ui::UiPlugin;

mod interaction;
mod lifecycle;
mod save;
mod simulation;
mod ui;
mod visuals;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(UiPlugin);

        app.add_systems(Startup,  (
            lifecycle::camera_spawn,
        ));  

        app.add_systems(
            OnEnter(GameState::Loading),
            save::save_setting_maneger
        );            

        app.add_systems(OnEnter(GameState::Menu), (
            lifecycle::cleanup_system,
        ));

        app.add_systems(OnEnter(GameState::LoadGame), (
            save::final_load_game,
        ));

        app.add_systems(OnEnter(GameState::Playing), (
            lifecycle::spawn_world_system,
        ));
        
        app.add_systems(Update, simulation::set_global_scale);

        app.add_systems(
            Update, (
                lifecycle::cleanup_system,
                lifecycle::spawn_world_system,
        )
            .chain()
            .run_if(state_changed::<CurrentWorld>.and(in_state(GameState::Playing)))
        );

        app.add_systems(
            Update,
            (
                visuals::update_plant_appearance,
                visuals::update_scene_scale,
                visuals::sync_inventory_visuals,
                visuals::animate_counters,
                visuals::update_resourse_text,
                visuals::grad_item_anim_and_zsort,
                visuals::shader_animation,
                visuals::price_button_text,
                interaction::state_dragg_item,
                simulation::plant_growth.run_if(on_timer(Duration::from_secs(1))),
                simulation::update_shader_settings,
            )
                .run_if(in_state(GameState::Playing)),
        );

        app.add_systems(Update, save::event_save_system.run_if(in_state(GameState::Playing)));
        app.add_systems(Last, save::event_save_system.run_if(in_state(GameState::Playing)));

        app.add_systems(Update, simulation::max_fps_sync);
        app.add_systems(PostUpdate, simulation::fps_limiter_system);

        app.add_observer(interaction::end_drag_item);
        app.add_observer(interaction::start_drag_item);
        app.add_observer(interaction::button_check);
        app.add_observer(interaction::harvest);
    }
}
