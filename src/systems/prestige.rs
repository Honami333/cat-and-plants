use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::schema::{save_file::*, types_and_states::*, resources::*};
use crate::systems::ui::func_fonts_loaded;
use crate::systems::visuals::format_number;

pub fn prestige_flag(
    mut contexts: EguiContexts,
    mut economy: ResMut<Economy>,
    mut global_inventory: ResMut<GlobalInventory>,
    mut count_item_type: ResMut<CountItemType>,
    mut prestige_inv: ResMut<PrestigeRoom>,
    mut fonts_loaded: Local<bool>,
    mut is_prestige: Local<bool>,
    mut confirmation: Local<bool>,
    mut prestige: Local<bool>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
    current_world: Res<State<CurrentWorld>>,
    world: Res<WorldScale>,
) {
    let Some(cit_inv) = count_item_type.get_inv_mut(&current_world, false) else { return; };

    let Some(gl_inv) = global_inventory.get_inv_mut(&current_world) else { return; };

    if cit_inv.iter().sum::<usize>() < gl_inv.len() { return; };

    let Some(sparks) = prestige_inv.get_sparks_res(&current_world) else { return; };

    let Some(pr_room) = prestige_inv.get_mut_room(&current_world) else { return; };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    *fonts_loaded = func_fonts_loaded(ctx, *fonts_loaded, &all_fonts, &font);

    let s = world.scale;

    egui::Window::new("Prestige Flag")
        .collapsible(false)
        .fixed_size([120.0* s, 50.0 * s])
        .title_bar(false)
        .anchor(egui::Align2::RIGHT_CENTER, [-10.0 * s, 30.0* s])
        .show(ctx, |ui| {
            ui.allocate_ui(egui::vec2(120.0 * s, 50.0 * s), |ui| {
                if !*confirmation {
                    ui.add_enabled_ui(*is_prestige, |ui| {
                        let response = ui.add_sized([120.0 * s, 25.0 * s], egui::Button::new(
                            egui::RichText::new(format!("PRESTIGE TO {}!", *pr_room + 1)).heading().color(egui::Color32::GOLD))
                        );
                        
                        if response.clicked() {
                            *confirmation = true;
                        };
                    });
                } else {
                    ui.horizontal(|ui| {
                        let sparcks = (1.0 + *pr_room as f64).powf(1.2).floor();

                        let response = ui.add_sized([60.0 * s, 25.0 * s], egui::Button::new(
                            egui::RichText::new("LET'S GO!").heading().color(egui::Color32::GOLD)
                        ));

                        if response.clicked() {
                            let default_gl_inv = GlobalInventory::default();
                            let default_cut_inv = CountItemType::default();

                            let Some(new_inv_gl) = default_gl_inv.get_inv(&current_world) else { return; };
                            let Some(new_inv_cit) = default_cut_inv.get_inv(&current_world, false) else { return; };

                            *gl_inv = *new_inv_gl;
                            *cit_inv = *new_inv_cit;

                            *prestige = true;
                            *confirmation = false;

                            economy.add_sparcks(sparks as usize, sparcks);
                        };

                        response.on_hover_ui(|ui| {
                            ui.allocate_ui(egui::vec2(120.0 * s, 40.0 * s), |ui| {
                                ui.label(egui::RichText::new("WARNING!").color(egui::Color32::RED).heading());

                                ui.separator();

                                ui.label(egui::RichText::new(format!("This action will completely erase your progress in the {} room.", current_world.get()))
                                    .color(egui::Color32::GREEN));

                                ui.label(egui::RichText::new(format!("Once confirmed, your plants will be sent to the kitty shelter"))
                                    .color(egui::Color32::GOLD));

                                ui.label(egui::RichText::new(format!("You will receive {} in the amount of: {}", sparks.to_string(), sparcks))
                                    .color(egui::Color32::GREEN));
                            }); 
                        });

                        let response = ui.add_sized([60.0 * s, 25.0 * s], egui::Button::new(
                            egui::RichText::new("Cancel").heading().color(egui::Color32::GRAY)
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
                            economy.add(*res as usize, -required_amount, false);
                        };

                        ui.horizontal(|ui| {
                            let color = if *amount >= required_amount { egui::Color32::GREEN } else {  egui::Color32::RED };

                            ui.add_sized(
                                [40.0 * s, 7.5 * s],
                                egui::Label::new(egui::RichText::new(
                                    format!(
                                        "{} {}/{}",
                                        res.to_string(),
                                        format_number(*amount),
                                        format_number(required_amount)
                                    )
                                ).color(color))
                            );

                            ui.add_sized([80.0 * s, 7.5 * s], egui::ProgressBar::new(target_cost as f32));
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