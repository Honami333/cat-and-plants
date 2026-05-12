use bevy::prelude::*;
use bevy_egui::{EguiPrimaryContextPass, egui::{self, Context}};

use crate::schema::resources::FontAssets;

mod map;
mod nurturing;
mod upgrades;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            (
                nurturing::trading_ui_system,
                map::map_ui_system,
                upgrades::show_upgrade_grid,
            )
                .chain(),
        );
    }
}

pub fn func_fonts_loaded(ctx: &mut Context, fonts_loaded: bool, all_fonts: &Assets<Font>, font: &FontAssets) -> bool {
     if !fonts_loaded {
        let Some(font_data) = all_fonts.get(&font.emoji_font) else { return false; };

        let mut fonts = egui::FontDefinitions::default();

        let bytes = (*font_data.data).clone();

        fonts.font_data.insert(
            "f".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(bytes)),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "f".to_owned());

            ctx.set_fonts(fonts);

        true
    } else { false }
}