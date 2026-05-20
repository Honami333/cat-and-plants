use crate::schema::{config::Upgrade, resources::*, types_and_states::*, save_file::*};
use bevy::prelude::*;
use crate::systems::{ui::*, visuals::*};
use bevy_egui::{EguiContexts, egui};
use strum::IntoEnumIterator;

pub fn show_upgrade_grid(
    mut contexts: EguiContexts,
    mut upgrade_storege: ResMut<UpgradeStorege>,
    mut upgrade_state: ResMut<UpgradeState>,
    mut economy: ResMut<Economy>,
    mut fonts_loaded: Local<bool>,
    mut assets_loaded: Local<bool>,
    mut handle_texture_id: Local<egui::TextureId>,
    prestige_inv: Res<PrestigeRoom>,
    count_item_type: Res<CountItemType>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    assets: Res<AtlasAssets>,
    world_scale: Res<WorldScale>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
) {
    if count_item_type.sunlit_nursery_inv.iter().sum::<usize>() < 2 && prestige_inv.sunlit_nursery == 0 { return; }

    let (new_bool, Some(atlas_layout), text_id) = 
        func_assets_loaded(
            *assets_loaded,
            *handle_texture_id,
            &mut contexts,
            &layouts,
            assets.pockets_of_improvements.clone(),
            &assets.common_layout_x40,
        ) else { return; };
    
    *assets_loaded = new_bool;
    *handle_texture_id = text_id;

    let Ok(ctx) = contexts.ctx_mut() else { return; };


    *fonts_loaded = func_fonts_loaded(ctx, *fonts_loaded, &all_fonts, &font);

    let s = world_scale.scale / 2.0;

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 150),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new("Garden Upgrades")
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -10.0 * s])
        .fixed_size([920.0 * s, 400.0 * s])
        .resizable(false)
        .constrain(true)
        // .collapsible(false)
        .frame(my_frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::vec2(750.0 * s, 390.0 * s), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Upgrades");
                        ui.separator();

                        egui::Grid::new("upgrades_grid")
                            .spacing([10.0 * s, 10.0 * s])
                            .show(ui, |ui| {
                                let upgrade_storege_clone = upgrade_storege.clone();

                                let cur_storage = match upgrade_state.selected_categories {
                                    EGUISelectedCategories::Sparcks => &mut upgrade_storege.sparcks,
                                    EGUISelectedCategories::Global => &mut upgrade_storege.global,
                                    EGUISelectedCategories::SunlitNursery => &mut upgrade_storege.sunlit_nursery,
                                };

                                for row in 0..4 {
                                    for col in 0..8 {
                                        if let Some(upgrade) = cur_storage.get_mut(&(row, col)) && upgrade.get_location_prestige_req(&prestige_inv) {
                                            let i = upgrade.texture_stage as usize;
                                            
                                            let image = create_image(*handle_texture_id, &atlas_layout, i,  (80.0, 80.0), s);

                                            add_upgrade(ui, upgrade, &mut economy, &upgrade_storege_clone, &count_item_type, s.clone(), image);
                                        } else {
                                            add_space(ui, s.clone())
                                        };
                                    };

                                    ui.end_row();
                                };
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
                                if categories == EGUISelectedCategories::Sparcks && !prestige_inv.first_prestige() { continue; };

                                let is_selected = categories == upgrade_state.selected_categories;

                                if ui
                                    .selectable_label(is_selected, categories.to_string())
                                    .clicked() {
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

fn upgrade_lvl_up(upgrade_storege: &UpgradeStorege, upgrade: &mut Upgrade, economy: &mut Economy) {
    let mut up_value =  1.0;
    
    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::CardboardBox) {up_value = value};

    if upgrade.current_level == upgrade.levels.len() { return; };

    let level = upgrade.levels[upgrade.current_level];



    for (i, res) in level.resource_types.iter().enumerate() {
        let is_sparck = if matches!(res, ResourceType::SunSparks) { true } else { false };

        if economy.get_item(*res as usize, is_sparck) < (level.costs[i] * up_value).ceil() { return; };
    };

    for (i, res) in level.resource_types.iter().enumerate() {
        let is_sparck = if matches!(res, ResourceType::SunSparks) { true } else { false };

        economy.add(*res as usize, -(level.costs[i] * up_value).ceil(), is_sparck);
    };

    upgrade.current_level += 1;
    upgrade.texture_stage.next_stage(upgrade.current_level as f32 / upgrade.levels.len() as f32);
}

fn add_space(ui: &mut egui::Ui, s: f32) {
    let size = egui::vec2(80.0 * s, 80.0 * s);

    ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_min_size(size);
        ui.set_max_size(size);
    });
}

fn add_upgrade(
    ui: &mut egui::Ui,
    upgrade: &mut Upgrade,
    economy: &mut Economy,
    upgrade_storege: &UpgradeStorege,
    cit: &CountItemType,
    s: f32, image:
    egui::Image) {
    let mut up_value =  1.0;
    
    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::CardboardBox) &&
        upgrade.id != UpgradeUID::CardboardBox {up_value = value};

    let size = egui::vec2(80.0 * s, 80.0 * s);

    let response = ui.add_sized(size, egui::Button::image(image).fill(egui::Color32::TRANSPARENT));

    let dependencies = upgrade.get_dependencies(upgrade_storege);

    let block_color = egui::Color32::RED;

    if response.clicked() && dependencies { upgrade_lvl_up(upgrade_storege, upgrade, economy); };

    response.on_hover_ui(|ui| {
        ui.set_max_width(400.0 * s);

        ui.heading(upgrade.name);

        ui.label(upgrade.description);

        if !dependencies {
            ui.add_space(5.0 * s);

            ui.colored_label(block_color, format!("Required dependencies: {}", upgrade.dependencies.len()));

            ui.separator();

            for ped in upgrade.dependencies {
                ui.colored_label(block_color, format!("{}", ped));
            };

            return;
        }

        ui.add_space(5.0);

        ui.columns(2, |columns| {
            let next_lvl_color = egui::Color32::GREEN;

            let current_lvl_color = if upgrade.current_level < upgrade.levels.len() {egui::Color32::GRAY } else { egui::Color32::GOLD };

            let is_max = upgrade.current_level == upgrade.levels.len();

            let current_lvl_text = if is_max { "MAX!".to_string() } else { format!("current lvl: {}", upgrade.current_level) };

            columns[0].colored_label(current_lvl_color, current_lvl_text);

            if let Some(level) = upgrade.levels.get(upgrade.current_level.saturating_sub(1)) {
                if let Some(val) = level.value {
                    let display_val = if upgrade.current_level == 0 { 100.0 } else { ((val) * 1000000.0).round() / 10000.0 };

                    columns[0].separator();

                    columns[0].add_space(5.0);

                    columns[0].colored_label(current_lvl_color, format!("current value: {}%", display_val));

                    if upgrade.id == UpgradeUID::ConcentratedNectar {
                        columns[0].colored_label(current_lvl_color, format!("current tomato bunus: {}%", (display_val * cit.sunlit_nursery_click[0] as f64 * 10000.0).floor() / 10000.0));
                    };
                };
                    
                if let Some(unlock_text) = upgrade.get_unlocking() && is_max {
                    columns[0].add_space(5.0);
                        
                    columns[0].colored_label(current_lvl_color, format!("Unlocking: {}", unlock_text));
                };
            };

            if !is_max {
                columns[1].colored_label(next_lvl_color,format!("next lvl: {}", upgrade.current_level + 1));
                if let Some(level) = upgrade.levels.get(upgrade.current_level) {
                        columns[1].separator();

                    if let Some(val) = level.value {
                        let display_val = ((val) * 1000000.0).round() / 10000.0;
                        columns[1].add_space(5.0);

                        columns[1].colored_label(next_lvl_color, format!("next level value: {}%", display_val));
                    };

                    if let Some(unlock_text) = upgrade.get_unlocking() {
                        columns[1].add_space(5.0);
                        
                        columns[1].colored_label(next_lvl_color, format!("Unlocking: {}", unlock_text));
                    };

                    if level.resource_types.len() > 0 {
                        columns[1].add_space(20.0);

                        columns[1].colored_label(next_lvl_color, "Required resources");

                        columns[1].separator();

                        for (res, cost) in (level.resource_types).iter().zip(level.costs) {
                            let is_sparck = if matches!(res, ResourceType::SunSparks) { true } else { false };

                            let cur_eco_res = economy.get_item(*res as usize, is_sparck);

                            let is_economy_color = if cur_eco_res >= *cost * up_value { next_lvl_color } else { block_color };

                            columns[1].add_space(5.0);

                            columns[1].colored_label(is_economy_color, format!("{}: {} / {}", res.to_string(), format_number(cur_eco_res), format_number(*cost * up_value)));
                        }
                    };
                };
            };
        });
    });
}