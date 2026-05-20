use crate::content::world::sunlit_nursery::*;
use crate::schema::{resources::*, types_and_states::*, save_file::*};
use crate::systems::visuals::format_number;
use bevy::prelude::*;
use crate::systems::ui::*;
use bevy_egui::{EguiContexts, egui};
use strum::IntoEnumIterator;

pub fn trading_ui_system(
    mut contexts: EguiContexts,
    mut trade_state: ResMut<TradeState>,
    mut economy: ResMut<Economy>,
    mut fonts_loaded: Local<bool>,
    prestige_inv: Res<PrestigeRoom>,
    current_world: Res<State<CurrentWorld>>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
    upgrade_storege: Res<UpgradeStorege>,
) {
    if *current_world.get() != CurrentWorld::WarmPawsPorch { return; };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let mut up_value =  0.0;

    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::PurrProfit) {up_value = value};

    *fonts_loaded = func_fonts_loaded(ctx, *fonts_loaded, &all_fonts, &font);

    egui::Window::new("Cat feed")
        .default_open(true)
        .fixed_size([600.0, 300.0])
        .resizable(false)
        .constrain(true)
        .show(ctx, |ui| {
            ui.columns(3, |columns| {
                let prestige_buff = 1.0 + ((prestige_inv.get_all_prestige() as f64).powf(1.25) * up_value);

                columns[0].label("Choice");
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
                columns[0].add_space(10.0);
                columns[0]
                    .add(egui::Slider::new(&mut trade_state.selected_percent, 1..=100).text("%"));
                columns[0].add_space(10.0);
                columns[0].horizontal(|ui| {
                    let current_economy = (economy.egui_get_item(trade_state.selected_item)
                        * trade_state.selected_percent as f64
                        / 100.0)
                        .floor();
                    trade_state.selected_economy = current_economy;

                    ui.horizontal_centered(|ui| {
                        ui.set_height(40.0);
                        ui.set_width(130.0);

                        ui.group(|ui| {
                            ui.set_height(50.0);
                            ui.set_width(50.0);

                            ui.vertical_centered(|ui| {
                                ui.add_sized(
                                    [40.0, 40.0],
                                    egui::Label::new(trade_state.selected_item.to_string()),
                                );
                                ui.add(egui::Label::new(format_number(current_economy)));
                            });
                        });

                        ui.centered_and_justified(|ui| {
                            ui.set_height(10.0);
                            ui.set_width(50.0);

                            ui.add(egui::Label::new("===>"));
                        });

                        ui.group(|ui| {
                            let trade = trade_well(&trade_state, &economy, &upgrade_storege);

                            ui.set_height(50.0);
                            ui.set_width(50.0);

                            ui.vertical_centered(|ui| {
                                ui.add_sized([40.0, 40.0], egui::Label::new("😸"));
                                ui.add(egui::Label::new(format_number((trade * prestige_buff).floor())));
                            });
                        });
                    });
                });

                let trade = trade_well(&trade_state, &economy, &upgrade_storege);

                columns[0].add_space(10.0);
                if trade > 0.0 {
                    columns[0].group(|ui| {
                        if ui
                            .add_sized([100.0, 40.0], egui::Button::new("Feed"))
                            .clicked()
                        {
                            economy.add(ResourceType::CatHappiness as usize, (trade * prestige_buff).floor(), false);

                            if trade_state.selected_item != EGUIResourceType::All {
                                economy.add(
                                    trade_state.selected_item as usize,
                                    -trade_state.selected_economy, 
                                    false
                                );
                            } else {
                                economy.add_all(trade_state.selected_percent as f64);
                            };
                        };
                    });
                }

                columns[1].label("Inventory");
                columns[1].separator();
                columns[1].vertical(|ui| {
                    egui::Grid::new("trade_inventory_grid")
                        .spacing([8.0, 8.0])
                        .show(ui, |ui| {
                            let all_items: Vec<_> = EGUIResourceType::iter()
                                .filter(|t| {
                                    *t != EGUIResourceType::All && *t != EGUIResourceType::None
                                })
                                .collect();

                            let range = if trade_state.selected_world == EGUICurrntWorld::All {
                                0..all_items.len()
                            } else {
                                let world_idx = trade_state.selected_world as usize;

                                let start = (world_idx - 1) * 4;

                                let end = start + 4;

                                start..end
                            };

                            for (i, item_idx) in range.enumerate() {
                                if let Some(item) = all_items.get(item_idx) {
                                    let is_select = trade_state.selected_item == *item
                                        || trade_state.selected_item == EGUIResourceType::All;

                                    if ui
                                        .add_sized(
                                            [40.0, 40.0],
                                            egui::Button::new(item.to_string()).selected(is_select),
                                        )
                                        .clicked()
                                    {
                                        trade_state.selected_item = *item;
                                    };

                                    if (i + 1) % 4 == 0 {
                                        ui.end_row();
                                    }
                                }
                            }
                        });
                });

                columns[2].label("Location");
                columns[2].separator();
                columns[2].vertical(|ui| {
                    for world in EGUICurrntWorld::iter() {
                        let is_select = trade_state.selected_world == world;

                        if ui.selectable_label(is_select, world.to_string()).clicked() {
                            trade_state.selected_world = world;

                            if world == EGUICurrntWorld::All {
                                trade_state.selected_item = EGUIResourceType::All;
                            };
                        }
                    }
                });
            });

            ui.allocate_space(ui.available_size());
        });
}

fn trade_well(
    trade_state: &TradeState,
    economy: &Economy,
    upgrade_storege: &UpgradeStorege,
) -> f64 {
    let mut up_value_1 =  1.0;
    
    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::WholesaleSupply) {up_value_1 = value};

    let mut cur_well = 0.0;

    let item_idx = trade_state.selected_item as usize;
    if item_idx > 0 {
        if let Some(well) = TRADEWELL.well.get(item_idx - 1) {
            cur_well = *well
        };
    };

    let trade = match trade_state.selected_item {
        EGUIResourceType::All => {
            economy.egui_get_item_all(TRADEWELL, trade_state.selected_percent as f64,&upgrade_storege)
        }
        EGUIResourceType::None => 0.0,
        _ => {
            let mut up_value_2 =  1.0;

            if let Some(plant) = trade_state.selected_item.into_plant().get(0) {
                if let (Some(value), _) = upgrade_storege.get_plant_global_modifier(&plant, PlantGGM::Joy) {up_value_2 = value};
            };

            cur_well * trade_state.selected_economy * up_value_2
        },
    };

    (trade * up_value_1).floor()
}
