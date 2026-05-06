use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use strum::IntoEnumIterator;
use crate::schema::{resources::FontAssets, types_and_states::{EGUISelectedCategories, Economy, UpgradeState, UpgradeStorege}, config::Upgrade};


pub fn show_upgrade_grid(
    mut contexts: EguiContexts,
    mut upgrade_storege: ResMut<UpgradeStorege>,
    mut upgrade_state: ResMut<UpgradeState>,
    mut economy: ResMut<Economy>,
    mut fonts_loaded: Local<bool>,
    all_fonts: Res<Assets<Font>>, 
    font: Res<FontAssets>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    if !*fonts_loaded {
        if let Some(font_data) = all_fonts.get(&font.emoji_font) {
            let mut fonts = egui::FontDefinitions::default();

            let bytes = (*font_data.data).clone(); 

            fonts.font_data.insert(
                "f".to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(bytes)),
            );

            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "f".to_owned());

            ctx.set_fonts(fonts);

            *fonts_loaded = true;
        };
    };

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 150),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new("Garden Upgrades")
    .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -10.0])
    .fixed_size([880.0, 360.0])
    .resizable(false)
    // .collapsible(false)
    .frame(my_frame)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            let left_rect = ui.available_rect_before_wrap().with_max_x(ui.available_rect_before_wrap().right() - 160.0);
            let right_rect = ui.available_rect_before_wrap().with_min_x(ui.available_rect_before_wrap().right() - 160.0);

            ui.scope_builder(egui::UiBuilder::new().max_rect(left_rect), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Upgrades");
                    ui.separator();

                    egui::Grid::new("upgrades_grid")
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            let cur_storage = match upgrade_state.selected_categories {
                                EGUISelectedCategories::Global => &mut upgrade_storege.global,
                                EGUISelectedCategories::SunlitNursery => &mut upgrade_storege.sunlit_nursery,
                            };

                            for (i, upgrade) in cur_storage.iter_mut().enumerate() {
                                let is_max = upgrade.current_level == upgrade.levels.len();

                                if ui.add_sized([80.0, 80.0], egui::Button::new(upgrade.icon).selected(is_max)).clicked() {
                                    upgrade_lvl_up(&mut *upgrade, &mut economy);
                                };

                                if i%9 == 0  && i > 0 { ui.end_row(); }
                                if i == 36 { break; }
                            };
                    });
                });
            });

            ui.scope_builder(egui::UiBuilder::new().max_rect(right_rect), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Categories");
                    ui.separator();

                    ui.vertical(|ui| {
                        for categories in EGUISelectedCategories::iter() {
                            let is_selected = categories == upgrade_state.selected_categories;

                            if ui.selectable_label(is_selected, categories.to_string()).clicked() {
                                upgrade_state.selected_categories = categories;
                            };
                        };
                    });
                });
            });
        });

        ui.allocate_space(ui.available_size());
    });
}

fn upgrade_lvl_up(
    upgrade: &mut Upgrade,
    economy: &mut Economy,

) {
    if upgrade.current_level == upgrade.levels.len() { return; };

    let level = upgrade.levels[upgrade.current_level];

    for(i, res) in level.resource_types.iter().enumerate() {
        if economy.get(*res) < level.costs[i] { return; }
    };
                                    
    for(i, res) in level.resource_types.iter().enumerate() {
        economy.add(*res as usize, -level.costs[i]);
    };

    upgrade.current_level += 1;
}