use crate::schema::{hud::*, resources::*, global_settings::*, common::*, global_inventory::*, item_type_info::*, economy_inventory::*, prestige::*, upgrade_storege::*};
use bevy::prelude::*;
use crate::systems::{ui::*, visuals::*, locales::*};
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
    settings: Res<GlobalSettings>,
    prestige_inv: Res<PrestigeRoom>,
    count_item_type: Res<ItemTypeInfo>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    assets: Res<AtlasAssets>,
    scale: Res<WorldScale>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
) {
    if count_item_type.item_count_inv.values().sum::<usize>() < 2 && prestige_inv.sunlit_nursery == 0 { return; }

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

    let s = scale.0;

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 150),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };
    egui::Window::new(translate("ui-garden-upgrades", &settings.language))
        .default_open(false) 
        .anchor(egui::Align2::RIGHT_BOTTOM, [-5.0 * s.x, -5.0 * s.y])
        .fixed_size([460.0 * s.x, 200.0 * s.y])
        .resizable(false)
        .constrain(true)
        .frame(my_frame)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::vec2(375.0 * s.x, 195.0 * s.y), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(translate("ui-upgrades", &settings.language));
                        ui.separator();

                        egui::Grid::new("upgrades_grid")
                            .spacing([5.0 * s.x, 5.0 * s.y])
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
                                            
                                            let Some(image) = create_image(*handle_texture_id,
                                                &atlas_layout,
                                                i,
                                                (40.0, 40.0),
                                                s
                                            ) else { continue; };

                                            add_upgrade(ui, upgrade, &mut economy, &upgrade_storege_clone, &settings, &count_item_type, s, image);
                                        } else {
                                            add_space(ui, s)
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
                        ui.label(translate("ui-categories", &settings.language));
                        ui.separator();

                        ui.vertical(|ui| {
                            for categories in EGUISelectedCategories::iter() {
                                if categories == EGUISelectedCategories::Sparcks && !prestige_inv.first_prestige() { continue; };

                                let is_selected = categories == upgrade_state.selected_categories;

                                if ui
                                    .selectable_label(is_selected, translate(categories.to_string().as_str(), &settings.language))
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
        if economy.get_res(*res) < (level.costs[i] * up_value).ceil() { return; };
    };

    for (i, res) in level.resource_types.iter().enumerate() {
        economy.add_res(*res, -(level.costs[i] * up_value).ceil());
    };

    upgrade.current_level += 1;
    upgrade.texture_stage.next_stage(upgrade.current_level as f32 / upgrade.levels.len() as f32);
}

fn add_space(ui: &mut egui::Ui, s: Vec2) {
    let size = egui::vec2(40.0 * s.x, 40.0 * s.y);

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
    settings: &GlobalSettings,
    cit: &ItemTypeInfo,
    s: Vec2,
    image: egui::Image
) {
    let mut up_value =  1.0;
    
    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::CardboardBox) &&
        upgrade.id != UpgradeUID::CardboardBox {up_value = value};

    let size = egui::vec2(40.0 * s.x, 40.0 * s.y);

    let response = ui.add_sized(size, egui::Button::image(image).fill(egui::Color32::TRANSPARENT));

    let dependencies = upgrade.get_dependencies(upgrade_storege);

    let block_color = egui::Color32::RED;

    if response.clicked() && dependencies { upgrade_lvl_up(upgrade_storege, upgrade, economy); };

    response.on_hover_ui(|ui| {
        ui.set_max_width(200.0 * s.x);

        ui.heading(translate(upgrade.name, &settings.language));

        ui.label(translate(upgrade.description, &settings.language));

        if !dependencies {
            ui.add_space(2.5 * (s.x).min(s.y));

            ui.colored_label(block_color, format!("{} {}", translate("ui-req-dependencies", &settings.language), upgrade.dependencies.len()));

            ui.separator();

            for ped in upgrade.dependencies {
                ui.colored_label(block_color, format!("{}",translate(&ped.to_string(), &settings.language)));
            };

            return;
        }

        ui.add_space(2.5 * (s.x).min(s.y));

        ui.columns(2, |columns| {
            let next_lvl_color = egui::Color32::GREEN;

            let is_max = upgrade.texture_stage == UpgradeStage::Max;

            let current_lvl_color = if  !is_max {egui::Color32::GRAY } else { egui::Color32::GOLD };

            let current_lvl_text = if is_max { translate("ui-max", &settings.language) } else { format!("{} {}", translate("ui-current-lvl", &settings.language), upgrade.current_level) };

            columns[0].colored_label(current_lvl_color, current_lvl_text);

            if let Some(level) = upgrade.levels.get(upgrade.current_level.saturating_sub(1)) {
                if let Some(val) = level.value {
                    let display_val = if upgrade.current_level == 0 {
                        match upgrade.id {
                            UpgradeUID::ConcentratedNectar => 0.0,
                            _ => 100.0
                        }
                    } else { ((val) * 1000000.0).round() / 10000.0 };

                    columns[0].separator();

                    columns[0].add_space(5.0);

                    columns[0].colored_label(current_lvl_color, format!("{} {}%", translate("ui-current-value", &settings.language), display_val));

                    let text = match upgrade.id {
                        UpgradeUID::ConcentratedNectar => {
                            format!("{} {}%",
                                translate("ui-current-tomato-bonus", &settings.language),
                                (display_val * cit.get_value_plant_ability(&TypePlant::Tomato, PlantAbility::TomatoClickCombo, 0) * 10000.0).floor() / 10000.0)
                            },
                        _ => "".to_string(),
                    };
                    
                    if !text.is_empty() {
                        columns[0].colored_label(current_lvl_color, text);
                    };
                };
                    
                if let Some(unlock_text) = upgrade.get_unlocking() && is_max {
                    columns[0].add_space(2.5 * (s.x).min(s.y));
                        
                    columns[0].colored_label(current_lvl_color, format!("{} {}", translate("ui-unlocking", &settings.language), translate(unlock_text, &settings.language)));
                };
            };

            if !is_max {
                columns[1].colored_label(next_lvl_color,format!("{} {}", translate("ui-next-lvl", &settings.language), upgrade.current_level + 1));
                if let Some(level) = upgrade.levels.get(upgrade.current_level) {
                        columns[1].separator();

                    if let Some(val) = level.value {
                        let display_val = ((val) * 1000000.0).round() / 10000.0;
                        columns[1].add_space(2.5 * (s.x).min(s.y));

                        columns[1].colored_label(next_lvl_color, format!("{} {}%", translate("ui-next-level-value", &settings.language), display_val));
                        
                        let text = match upgrade.id {
                            UpgradeUID::ConcentratedNectar => {
                                format!("{} {}%",
                                    translate("ui-current-tomato-bonus", &settings.language),
                                    (display_val * cit.get_value_plant_ability(&TypePlant::Tomato, PlantAbility::TomatoClickCombo, 0) * 10000.0).floor() / 10000.0)
                                }
                            _ => "".to_string(),
                        };
                    
                        if !text.is_empty() {
                            columns[1].colored_label(next_lvl_color, text);
                        };
                    };



                    if let Some(unlock_text) = upgrade.get_unlocking() {
                        columns[1].add_space(2.5 * (s.x).min(s.y));
                        
                        columns[1].colored_label(next_lvl_color, format!("{} {}", translate("ui-unlocking", &settings.language), translate(unlock_text, &settings.language)));
                    };

                    if level.resource_types.len() > 0 {
                        columns[1].add_space(10.0 * (s.x).min(s.y));

                        columns[1].colored_label(next_lvl_color, translate("ui-req-resources", &settings.language));

                        columns[1].separator();

                        for (res, cost) in (level.resource_types).iter().zip(level.costs) {
                            let cur_eco_res = economy.get_res(*res);

                            let is_economy_color = if cur_eco_res >= *cost * up_value { next_lvl_color } else { block_color };

                            columns[1].add_space(2.5 * (s.x).min(s.y));

                            columns[1].colored_label(is_economy_color, format!("{}: {} / {}", translate(res.to_string().as_str(), &settings.language), format_number(cur_eco_res), format_number(*cost * up_value)));
                        }
                    };
                };
            };
        });
    });
}
