use crate::schema::types_and_states::*;
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_egui::EguiPlugin;
use std::time::Duration;
use ui::UiPlugin;

mod interaction;
mod lifecycle;
mod simulation;
mod ui;
mod visuals;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_plugins(UiPlugin);

        app.add_systems(OnEnter(GameState::Playing), (
            lifecycle::camera_spawn,
            lifecycle::add_start
        ));

        app.add_systems(
            OnExit(CurrentWorld::WarmPawsPorch),
            lifecycle::cleanup_system,
        );
        app.add_systems(
            OnExit(CurrentWorld::SunlitNursery),
            lifecycle::cleanup_system,
        );

        app.add_systems(
            OnEnter(CurrentWorld::WarmPawsPorch),
            lifecycle::spawn_world_system,
        );
        app.add_systems(
            OnEnter(CurrentWorld::SunlitNursery),
            lifecycle::spawn_world_system,
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
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );

        app.add_observer(interaction::end_drag_item);
        app.add_observer(interaction::start_drag_item);
        app.add_observer(interaction::button_check);
        app.add_observer(interaction::harvest);
    }
}
