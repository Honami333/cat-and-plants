use crate::schema::{config::Upgrade, types_and_states::*};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use strum::IntoEnumIterator;

pub fn show_upgrade_grid(
    mut contexts: EguiContexts,
    mut upgrade_storege: ResMut<UpgradeStorege>,
    mut upgrade_state: ResMut<UpgradeState>,
    mut economy: ResMut<Economy>,
    world_scale: Res<WorldScale>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
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
        .fixed_size([880.0 * s, 360.0 * s])
        .resizable(false)
        .constrain(true)
        // .collapsible(false)
        .frame(my_frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::vec2(710.0 * s, 340.0 * s), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Upgrades");
                        ui.separator();

                        egui::Grid::new("upgrades_grid")
                            .spacing([10.0 * s, 10.0 * s])
                            .show(ui, |ui| {
                                let cur_storage = match upgrade_state.selected_categories {
                                    EGUISelectedCategories::Global => &mut upgrade_storege.global,
                                    EGUISelectedCategories::SunlitNursery => {
                                        &mut upgrade_storege.sunlit_nursery
                                    }
                                };

                                for (i, upgrade) in cur_storage.iter_mut().enumerate() {
                                    let is_max = upgrade.current_level == upgrade.levels.len();

                                    if ui
                                        .add_sized(
                                            [80.0 * s, 80.0 * s],
                                            egui::Button::new(upgrade.icon).selected(is_max),
                                        )
                                        .clicked()
                                    {
                                        upgrade_lvl_up(&mut *upgrade, &mut economy);
                                    };

                                    if i % 9 == 0 && i > 0 {
                                        ui.end_row();
                                    }
                                    if i == 36 {
                                        break;
                                    }
                                }
                            });
                    });
                });

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

fn upgrade_lvl_up(upgrade: &mut Upgrade, economy: &mut Economy) {
    if upgrade.current_level == upgrade.levels.len() {
        return;
    };

    let level = upgrade.levels[upgrade.current_level];

    for (i, res) in level.resource_types.iter().enumerate() {
        if economy.get_item(*res as usize) < level.costs[i] {
            return;
        }
    }

    for (i, res) in level.resource_types.iter().enumerate() {
        economy.add(*res as usize, -level.costs[i]);
    }

    upgrade.current_level += 1;
}
