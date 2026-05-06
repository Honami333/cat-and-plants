use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

mod map;
mod nurturing;
mod upgrades;


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, (
            nurturing::trading_ui_system,
            map::map_ui_system,
            upgrades::show_upgrade_grid,
        ).chain());

    }
}