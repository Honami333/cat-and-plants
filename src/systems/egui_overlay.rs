use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use strum::IntoEnumIterator;
use crate::schema::{types_and_states::*, resources::*};
use crate::systems::visials::format_number;
use crate::world::TRADEWELL;


pub fn trading_ui_system(
    mut contexts: EguiContexts,
    mut trade_state: ResMut<TradeState>,
    mut economy: ResMut<Economy>,
    all_fonts: Res<Assets<Font>>, 
    font: Res<FontAssets>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

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
    };

    egui::Window::new("Cat trade")
    .default_open(true)
    .default_size([600.0, 400.0])
    .resizable(false)

    .show(ctx, |ui| {
        ui.columns(3, |columns| {
            columns[0].label("Обмен");
            columns[0].horizontal(|ui| {
                let count_percent: [u8; 5] = [1, 10, 25, 50, 100];
                
                for percent in count_percent {
                    let is_select = trade_state.selected_percent == percent;

                    if ui.selectable_label(is_select, percent.to_string()).clicked() {
                        trade_state.selected_percent = percent;
                    }
                };
            });
            columns[0].add(
                egui::Slider::new(&mut trade_state.selected_percent, 1..=100).text("%")
            );
            columns[0].add_space(20.0);
            columns[0].horizontal(|ui| {
                let current_economy = (economy.egui_get(trade_state.selected_item) * trade_state.selected_percent as f64 / 100.0).floor();
                trade_state.selected_economy = current_economy;

                ui.horizontal_centered(|ui| {
                    ui.set_height(40.0);
                    ui.set_width(160.0);

                    ui.group(|ui| {
                        ui.add_sized([40.0, 40.0], egui::Label::new(trade_state.selected_item.to_string()));
                    });

                    ui.vertical_centered(|ui| {
                        ui.set_height(20.0);
                        ui.set_width(80.0);

                        ui.add(egui::Label::new("========>"));
                        ui.add(egui::Label::new(format_number(current_economy)));
                    });

                    ui.group(|ui| {
                        ui.add_sized([40.0, 40.0], egui::Label::new("😸"));
                    });
                });
            });
            columns[0].add_space(20.0);
            columns[0].group(|ui| {
                let trade = match trade_state.selected_item {
                    EGUIResourceType::All => economy.get_egui_all(TRADEWELL, trade_state.selected_economy),
                    EGUIResourceType::Tomatoes => trade_state.selected_economy * TRADEWELL.well[0],
                    EGUIResourceType::Cucumbers => trade_state.selected_economy * TRADEWELL.well[1],
                    EGUIResourceType::Corn => trade_state.selected_economy * TRADEWELL.well[2],
                    EGUIResourceType::Pumpkin => trade_state.selected_economy * TRADEWELL.well[3],
                    EGUIResourceType::None => 0.0
                };

                if ui.add_sized([150.0, 40.0], egui::Button::new("Trade")).clicked() {
                    economy.add(0,  trade);
                    if trade_state.selected_item != EGUIResourceType::All {
                        economy.add(trade_state.selected_item as usize, -trade_state.selected_economy);
                    } else {
                        economy.add_all(trade_state.selected_economy);
                    };
                }
            });

            columns[1].label("Инвентарь");
            columns[1].vertical(|ui| {
                ui.separator();
                egui::Grid::new("trade_inventory_grid")
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                    let all_items: Vec<_> = EGUIResourceType::iter()
                        .filter(|t| *t != EGUIResourceType::All && *t != EGUIResourceType::None)
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
                            let is_select = trade_state.selected_item == *item || trade_state.selected_item == EGUIResourceType::All;
                        
                            if ui.add_sized([40.0, 40.0], egui::Button::new(item.to_string()).selected(is_select)).clicked() {
                                trade_state.selected_item = *item;
                            };

                            if (i + 1) % 4 == 0 {
                                ui.end_row();
                            }
                        }
                    }
                });
            });

            columns[2].label("Локация");
            columns[2].vertical(|ui| {
                for world in EGUICurrntWorld::iter() {
                    let is_select = trade_state.selected_world == world;

                    if ui.selectable_label(is_select, world.to_string()).clicked() {
                        trade_state.selected_world = world;

                        if world == EGUICurrntWorld::All {
                            trade_state.selected_item = EGUIResourceType::All;
                        };
                    }
                };
            });
        })
    });
}