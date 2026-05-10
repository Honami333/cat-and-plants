use crate::schema::{config::Upgrade, resources::*, types_and_states::*};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use strum::IntoEnumIterator;

pub fn show_upgrade_grid(
    mut contexts: EguiContexts,
    mut upgrade_storege: ResMut<UpgradeStorege>,
    mut upgrade_state: ResMut<UpgradeState>,
    mut economy: ResMut<Economy>,
    mut fonts_loaded: Local<bool>,
    world_scale: Res<WorldScale>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    if !*fonts_loaded {
        if let Some(font_data) = all_fonts.get(&font.emoji_font) {
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

            *fonts_loaded = true;
        };
    };

    let s = world_scale.scale / 2.0;

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 150),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new("Garden Upgrades")
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0 * s, -10.0 * s])
        .fixed_size([880.0 * s, 380.0 * s])
        .resizable(false)
        .constrain(true)
        // .collapsible(false)
        .frame(my_frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::vec2(710.0 * s, 380.0 * s), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Upgrades");
                        ui.separator();

                        egui::Grid::new("upgrades_grid")
                            .spacing([10.0 * s, 10.0 * s])
                            .show(ui, |ui| {
                                let upgrade_storege_clone = upgrade_storege.clone();

                                let cur_storage = match upgrade_state.selected_categories {
                                    EGUISelectedCategories::Global => &mut upgrade_storege.global,
                                    EGUISelectedCategories::SunlitNursery => {
                                        &mut upgrade_storege.sunlit_nursery
                                    }
                                };

                                for row in 0..4 {
                                    for col in 0..8 {
                                        if let Some(upgrade) = cur_storage.get_mut(&(row, col)) {
                                            if !upgrade.get_dependencies(&upgrade_storege_clone) {
                                                add_space(ui, s.clone());
                                                continue;
                                            };

                                            add_upgrade(&upgrade_storege_clone, ui, upgrade, &mut economy, s.clone());
                                        } else {
                                            add_space(ui, s.clone())
                                        };
                                    }

                                    ui.end_row();
                                }
                            });
                    });
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Categories");
                        ui.separator();

                        ui.vertical(|ui| {
                            for categories in EGUISelectedCategories::iter() {
                                let is_selected = categories == upgrade_state.selected_categories;

                                if ui
                                    .selectable_label(is_selected, categories.to_string())
                                    .clicked()
                                {
                                    upgrade_state.selected_categories = categories;
                                };
                            }
                        });
                    });
                });
            });

            ui.allocate_space(ui.available_size());
        });
}

fn upgrade_lvl_up(upgrade_stiorege: &UpgradeStorege, upgrade: &mut Upgrade, economy: &mut Economy) {
    let (value, _) = upgrade_stiorege.get_global_modifier(UpgradeUID::CardboardBox);

    if upgrade.current_level == upgrade.levels.len() {
        return;
    };

    let level = upgrade.levels[upgrade.current_level];

    for (i, res) in level.resource_types.iter().enumerate() {
        if economy.get_item(*res as usize) < (level.costs[i] * value).ceil() {
            return;
        }
    }

    for (i, res) in level.resource_types.iter().enumerate() {
        economy.add(*res as usize, (-level.costs[i] * value).ceil());
    }

    upgrade.current_level += 1;
}

fn add_space(ui: &mut egui::Ui, s: f32) {
    ui.allocate_ui(egui::vec2(80.0 * s, 80.0 * s), |ui| {
        ui.group(|ui| {
            ui.allocate_space(ui.available_size());
        });
    });
}

fn add_upgrade(upgrade_stiorege: &UpgradeStorege, ui: &mut egui::Ui, upgrade: &mut Upgrade, economy: &mut Economy, s: f32) {
    let is_max = upgrade.current_level == upgrade.levels.len();

    let response = ui.add_sized(
        [80.0 * s, 80.0 * s],
        egui::Button::new(upgrade.icon).selected(is_max),
    );

    if response.clicked() {
        upgrade_lvl_up(upgrade_stiorege, upgrade, economy);
    }
}
