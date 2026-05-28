use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use strum::IntoEnumIterator;
use crate::schema::{economy_inventory::*, common::*, global_settings::*, hud::*};
use crate::systems::{locales::translate, visuals::format_number};

const ITEM_ADD_ANIM: f64 = 1.5;


pub fn economy_inventory(
    mut contexts: EguiContexts,
    mut visual_counter: ResMut<VisualCounter>,
    settings: Res<GlobalSettings>,
    game_state: Res<State<GameState>>,
    economy: Res<Economy>,
    scale: Res<WorldScale>,
) {
    if *game_state != GameState::Playing { return; };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 200),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    let s = scale.0;

    let window_response  = egui::Window::new(translate("ui-cat-paws-inventory", &settings.language))
        .frame(my_frame)
        .default_open(false) 
        .fixed_size([150.0 * s.x, 100.0 * s.y])
        .anchor(egui::Align2::LEFT_BOTTOM, [5.0 * s.x, -5.0 * s.y])
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    for res in ResourceType::iter() {
                        let Some(count) = economy.vault.get(&res) else { continue; };

                        visual_counter.target_value.entry(res).or_insert(*count);

                        if let Some(vsc_target) = visual_counter.target_value.get_mut(&res) {
                            *vsc_target = *count;
                        };

                        let vsc_display = visual_counter.display_value.entry(res).or_insert([*count, 0.0]);

                        let color = if vsc_display[0] > 0.0 { egui::Color32::GREEN } else { egui::Color32::RED };

                        let text = egui::RichText::new(
                            format!("{}   {}",
                                translate(&res.to_string(), &settings.language),
                                format_number(vsc_display[0])))
                                .heading()
                                .color(color);

                        ui.add_enabled_ui(false, |ui| {
                            ui.add_sized(
                                [150.0 * s.x, 15.0 * s.y], 
                                egui::Button::new(text)
                            );
                        });
                    }
                });

            ui.allocate_space(ui.available_size());
        });

    if let Some(res) = window_response {
        if res.inner.is_some() { return; };
    };
    egui::Area::new(egui::Id::new("economy_anim_area"))
        .pivot(egui::Align2::LEFT_BOTTOM) 
        .constrain(true) 
        .anchor(egui::Align2::LEFT_BOTTOM, [5.0 * s.x, -30.0 * s.y])
        .show(ctx, |ui| {
            ui.set_width(150.0 * s.x);

            ui.vertical(|ui| {
                for res in ResourceType::iter() {
                    let Some(count) = economy.vault.get(&res) else { continue; };
                    
                    visual_counter.target_value.entry(res).or_insert(*count);

                    let vsc_display = visual_counter.display_value
                        .entry(res)
                        .or_insert([0.0, ITEM_ADD_ANIM]);

                    if vsc_display[0] != *count {
                        let anim_count = *count - vsc_display[0];

                        let color = if anim_count > 0.0 { egui::Color32::GREEN } else { egui::Color32::RED };

                        let sing = if anim_count > 0.0 { "+" } else { "" };

                        if vsc_display[1] == 0.0 {
                            let v = vsc_display[0];

                            if let Some(vsc_target) = visual_counter.target_value.get(&res) {
                                if vsc_target != count {
                                    visual_counter.display_value.insert(res, [v, ITEM_ADD_ANIM]);
                                };
                            };

                            if let Some(vsc_target) = visual_counter.target_value.get_mut(&res) {
                                *vsc_target = *count; 
                            };
                        };

                        let text = egui::RichText::new(
                            format!("{}   {}{}",
                                translate(&res.to_string(), &settings.language),
                                sing,
                                format_number(anim_count)
                            )
                        )
                        .heading()
                        .color(color);

                        ui.add_sized([50.0 * s.x, 15.0 * s.y], egui::Label::new(text));
                    };
                };
            });
        });
}


pub fn animate_counters(
    time: Res<Time>,
    mut visual_counter: ResMut<VisualCounter>,
) {
    let mut new_vsc_display = HashMap::new();

    for (res, vsc_target) in visual_counter.target_value.iter() {
        let Some(vsc_display) = visual_counter.display_value.get(res) else { continue; };

        if vsc_display[1] > 0.0 { new_vsc_display.insert(*res, [vsc_display[0], vsc_display[1] - time.delta_secs() as f64]); continue; };

        let vsc_display = vsc_display[0];

        if (vsc_display - *vsc_target).abs() < 0.1 {
            new_vsc_display.insert(*res, [*vsc_target, 0.0]);
        } else {
            let step = (*vsc_target - vsc_display) * time.delta_secs() as f64 * 5.0;

            new_vsc_display.insert(*res, [vsc_display + step, 0.0]);
        };
    }

    visual_counter.display_value = new_vsc_display;
}