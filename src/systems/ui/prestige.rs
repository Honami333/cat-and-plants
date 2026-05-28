use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::schema::{common::*, global_inventory::*, item_type_info::*, economy_inventory::*, prestige::*, global_settings::*, resources::*};
use crate::systems::ui::func_fonts_loaded;
use crate::systems::visuals::format_number;
use crate::systems::locales::*;


pub fn prestige_flag(
    mut contexts: EguiContexts,
    mut economy: ResMut<Economy>,
    mut global_inventory: ResMut<GlobalInventory>,
    mut iti_inventory: ResMut<ItemTypeInfo>,
    mut prestige_inv: ResMut<PrestigeRoom>,
    mut fonts_loaded: Local<bool>,
    mut is_prestige: Local<bool>,
    mut confirmation: Local<bool>,
    mut prestige: Local<bool>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
    current_world: Res<State<CurrentWorld>>,
    scale: Res<WorldScale>,
    settings: Res<GlobalSettings>,
) {
    let Some(cit_inv) = iti_inventory.get_for_world_mut(&current_world) else { return; };

    let Some(gl_inv) = global_inventory.get_for_world_mut(&current_world) else { return; };

    if cit_inv.values().sum::<usize>() < gl_inv.len() { return; };

    let Some(sparks) = prestige_inv.get_sparks_res(&current_world) else { return; };

    let Some(pr_room) = prestige_inv.get_mut_room(&current_world) else { return; };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    *fonts_loaded = func_fonts_loaded(ctx, *fonts_loaded, &all_fonts, &font);

    let s = scale.0;

    egui::Window::new(translate("ui-prestige-flag", &settings.language))
        .collapsible(false)
        .fixed_size([120.0* s.x, 50.0 * s.y])
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_CENTER, [-10.0 * s.x, 30.0* s.y])
        .show(ctx, |ui| {
            ui.allocate_ui(egui::vec2(120.0 * s.x, 50.0 * s.y), |ui| {
                if !*confirmation {
                    ui.add_enabled_ui(*is_prestige, |ui| {
                        let response = ui.add_sized([120.0 * s.x, 25.0 * s.y], egui::Button::new(
                            egui::RichText::new(format!("{} {}!", translate("ui-prestige-to", &settings.language), *pr_room + 1)).heading().color(egui::Color32::GOLD))
                        );
                        
                        if response.clicked() {
                            *confirmation = true;
                        };
                    });
                } else {
                    ui.horizontal(|ui| {
                        let sparcks = (1.0 + *pr_room as f64).powf(1.2).floor();

                        let response = ui.add_sized([60.0 * s.x, 25.0 * s.y], egui::Button::new(
                            egui::RichText::new(translate("ui-lets-go", &settings.language)).heading().color(egui::Color32::GOLD)
                        ));

                        if response.clicked() {
                            let default_gl_inv = GlobalInventory::default();
                            let default_cut_inv = ItemTypeInfo::default();

                            let Some(new_inv_gl) = default_gl_inv.get_for_world(&current_world) else { return; };
                            let Some(new_inv_cit) = default_cut_inv.get_for_world(&current_world) else { return; };

                            *gl_inv = new_inv_gl.clone();
                            *cit_inv = new_inv_cit.clone();

                            *prestige = true;
                            *confirmation = false;

                            economy.add_res(sparks, sparcks);
                        };

                        response.on_hover_ui(|ui| {
                            ui.allocate_ui(egui::vec2(120.0 * s.x, 40.0 * s.y), |ui| {
                                ui.label(egui::RichText::new(translate("ui-warning", &settings.language)).color(egui::Color32::RED).heading());

                                ui.separator();

                                ui.label(egui::RichText::new(format!("{} {}", translate("ui-warn-erase", &settings.language), translate(current_world.get().to_string().as_str(), &settings.language)))
                                    .size(10.0)
                                    .color(egui::Color32::GREEN));

                                ui.label(egui::RichText::new(format!("{}", translate("ui-warn-shelter", &settings.language)))
                                    .size(10.0)
                                    .color(egui::Color32::GOLD));

                                ui.label(egui::RichText::new(format!("{} {} {} {}", translate("ui-warn-receive", &settings.language), translate(sparks.to_string().as_str(), &settings.language), translate("ui-warn-amount", &settings.language), sparcks))
                                    .size(10.0)
                                    .color(egui::Color32::GREEN));
                            }); 
                        });

                        let response = ui.add_sized([60.0 * s.x, 25.0 * s.y], egui::Button::new(
                            egui::RichText::new(translate("ui-cancel", &settings.language)).heading().color(egui::Color32::GRAY)
                        ));

                        if response.clicked() {
                            *confirmation = false;
                        }
                    });
                };

                ui.separator();

                let (res_type, res_amont) = economy.get_prestige_res(&current_world);

                *is_prestige = true;

                ui.vertical(|ui| {
                    for (i, res) in res_type.iter().enumerate() {
                        let Some(amount) = res_amont.get(i) else { continue; };

                        let Some(required_amount) = current_world.get_cost(*res, *pr_room) else { continue; };

                        let target_cost = (amount / required_amount).clamp(0.0, 1.0);

                        if target_cost < 1.0 { *is_prestige = false };

                        if *prestige {
                            economy.add_res(*res, -required_amount);
                        };

                        ui.horizontal(|ui| {
                            let color = if *amount >= required_amount { egui::Color32::GREEN } else {  egui::Color32::RED };

                            ui.add_sized(
                                [40.0 * s.x, 7.5 * s.y],
                                egui::Label::new(egui::RichText::new(
                                    format!(
                                        "{} {}/{}",
                                        translate(res.to_string().as_str(), &settings.language),
                                        format_number(*amount),
                                        format_number(required_amount)
                                    )
                                ).color(color))
                            );

                            ui.add_sized([80.0 * s.x, 7.5 * s.y], egui::ProgressBar::new(target_cost as f32));
                        });
                    };
                    if *prestige {
                        *pr_room += 1;

                        *prestige = false;
                    };
                });

            });
        });
}
