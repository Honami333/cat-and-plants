use crate::schema::{common::*, economy_inventory::*, prestige::*, upgrade_storege::*, global_settings::*, hud::*};
use crate::systems::visuals::format_number;
use crate::content::world::sunlit_nursery::*;
use bevy::prelude::*;
use crate::systems::ui::*;
use bevy_egui::{EguiContexts, egui};
use strum::IntoEnumIterator;
use crate::systems::locales::*;

pub fn trading_ui_system(
    mut contexts: EguiContexts,
    mut trade_state: ResMut<FeedState>,
    mut economy: ResMut<Economy>,
    mut fonts_loaded: Local<bool>,
    prestige_inv: Res<PrestigeRoom>,
    current_world: Res<State<CurrentWorld>>,
    scale: Res<WorldScale>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
    upgrade_storege: Res<UpgradeStorege>,
    settings: Res<GlobalSettings>,
) {
    if *current_world.get() != CurrentWorld::WarmPawsPorch { return; };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let mut up_value =  0.0;

    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::PurrProfit) {up_value = value};

    *fonts_loaded = func_fonts_loaded(ctx, *fonts_loaded, &all_fonts, &font);

    let s = scale.0;

    let trade = trade_well(&trade_state, &economy, &upgrade_storege);

    egui::Window::new(translate("ui-cat-feed", &settings.language))
        .default_open(false) 
        .default_open(true)
        .fixed_size([300.0 * s.x, 150.0 * s.y])
        .resizable(false)
        .constrain(true)
        .show(ctx, |ui| {
            ui.columns(3, |columns| {
                let prestige_buff = 1.0 + ((prestige_inv.get_all_prestige() as f64).powf(1.25) * up_value);

                columns[0].label(translate("ui-choice", &settings.language));
                columns[0].separator();
                columns[0].horizontal(|ui| {
                    let count_percent: [u8; 5] = [1, 10, 25, 50, 100];

                    for percent in count_percent {
                        let is_select = trade_state.selected_percent == percent;

                        if ui
                            .selectable_label(is_select, percent.to_string())
                            .clicked()
                        {
                            trade_state.selected_percent = percent;
                        };
                    }
                });
                columns[0].add_space(5.0 * (s.x).min(s.y));
                columns[0]
                    .add(egui::Slider::new(&mut trade_state.selected_percent, 1..=100).text("%"));
                columns[0].add_space(5.0 * (s.x).min(s.y));
                columns[0].horizontal(|ui| {
                    let current_economy = economy.egui_get_res_list(
                        TRADEWELL,
                        trade_state.selected_percent as f64,
                        &upgrade_storege,
                        &trade_state.selected_item,
                        false,
                        false
                    );
                    
                    trade_state.selected_economy = current_economy;

                    ui.horizontal_centered(|ui| {
                        ui.set_height(20.0 * s.y);
                        ui.set_width(65.0 * s.x);

                        ui.group(|ui| {
                            ui.set_height(25.0 * s.y);
                            ui.set_width(25.0 * s.x);

                            ui.vertical_centered(|ui| {
                                let text = match trade_state.selected_item.len() {
                                    0 => "ui-trade-empty".to_string(),
                                    1 => trade_state.selected_item[0].to_string(),
                                    _ => "ui-trade-selected-handful".to_string()
                                };
                                    
                                ui.add_sized(
                                    [20.0 * s.x, 20.0 * s.y],
                                    egui::Label::new(translate(&text, &settings.language)),
                                );
                                ui.add(egui::Label::new(format_number(current_economy)));
                            });
                        });

                        ui.centered_and_justified(|ui| {
                            ui.set_height(5.0 * s.y);
                            ui.set_width(25.0 * s.x);

                            ui.add(egui::Label::new("===>"));
                        });

                        ui.group(|ui| {
                            ui.set_height(25.0 * s.y);
                            ui.set_width(25.0 * s.x);

                            ui.vertical_centered(|ui| {
                                ui.add_sized([20.0 * s.x, 20.0 * s.y], egui::Label::new(translate("res-cat-happiness", &settings.language)));
                                ui.add(egui::Label::new(format_number((trade * prestige_buff).floor())));
                            });
                        });
                    });
                });

                let trade = trade_well(&trade_state, &economy, &upgrade_storege);

                columns[0].add_space(5.0 * (s.x).min(s.y));
                if trade > 0.0 {
                    columns[0].group(|ui| {
                        if ui
                            .add_sized([50.0 * s.x, 20.0 * s.y], egui::Button::new(translate("ui-feed", &settings.language)))
                            .clicked()
                        {
                            economy.add_res(ResourceType::CatHappiness, (trade * prestige_buff).floor());

                            economy.feed_res_list(
                                trade_state.selected_percent as f64,
                                &trade_state.selected_item,
                            );
                        };
                    });
                }
                
                columns[1].label(translate("ui-inventory", &settings.language));
                columns[1].separator();
                columns[1].vertical(|ui| {
                    egui::Grid::new("trade_inventory_grid")
                        .spacing([4.0 * s.x, 4.0 * s.y])
                        .show(ui, |ui| {
                            let all_items: Vec<_> = ResourceType::iter()
                                .filter(|t| {
                                    !matches!(*t, ResourceType::CatHappiness | ResourceType::SunSparks | ResourceType::PhotoSparks)})
                                .collect();

                            let range = if trade_state.selected_world == EGUICurrntWorld::All {
                                0..all_items.len()
                            } else {
                                let world_idx = trade_state.selected_world as usize;

                                let end_pos = match trade_state.selected_world {
                                    EGUICurrntWorld::All => 0,
                                    EGUICurrntWorld::SunlitNursery => 4,
                                    EGUICurrntWorld::ShadowGreenhouse => 5,
                                };

                                let start = (world_idx - 1) * 4;

                                let end = start + end_pos;

                                start..end
                            };

                            for (i, item_idx) in range.enumerate() {
                                if let Some(item) = all_items.get(item_idx) {
                                    let is_select = trade_state.selected_item.iter().find(|r| item == *r).is_some();

                                    if ui
                                        .add_sized(
                                            [20.0 * s.x, 20.0 * s.y],
                                            egui::Button::new(translate(item.as_ref(), &settings.language)).selected(is_select),
                                        )
                                        .clicked()
                                    {
                                        if is_select {
                                            trade_state.selected_item.retain(|r| *r != *item);
                                        } else {
                                            trade_state.selected_item.push(*item);
                                        }
                                        
                                    };

                                    if (i + 1) % 4 == 0 {
                                        ui.end_row();
                                    }
                                }
                            }
                        });
                });

                columns[2].label(translate("ui-location", &settings.language));
                columns[2].separator();
                columns[2].vertical(|ui| {
                    for world in EGUICurrntWorld::iter() {
                        let is_select = trade_state.selected_world == world;

                        if ui.selectable_label(is_select, translate(world.to_string().as_str(), &settings.language)).clicked() {
                            trade_state.selected_world = world;
                        };
                    };
                });
            });

            ui.allocate_space(ui.available_size());
        });
}


fn trade_well(
    trade_state: &FeedState,
    economy: &Economy,
    upgrade_storege: &UpgradeStorege,
) -> f64 {
    let mut up_value_1 =  1.0;
    
    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::WholesaleSupply) {up_value_1 = value};

    let trade = economy.egui_get_res_list(
        TRADEWELL,
        trade_state.selected_percent as f64,
        upgrade_storege,
        &trade_state.selected_item,
        true,
        true,
    );

    (trade * up_value_1).floor()
}
