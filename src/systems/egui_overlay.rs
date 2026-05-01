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
    .fixed_size([600.0, 400.0])
    .resizable(false)

    .show(ctx, |ui| {
        ui.columns(3, |columns| {
            columns[0].label("Choice");
            columns[0].separator();
            columns[0].horizontal(|ui| {
                let count_percent: [u8; 5] = [1, 10, 25, 50, 100];
                
                for percent in count_percent {
                    let is_select = trade_state.selected_percent == percent;

                    if ui.selectable_label(is_select, percent.to_string()).clicked() {
                        trade_state.selected_percent = percent;
                    };
                };
            });
            columns[0].add_space(10.0);
            columns[0].add( 
                egui::Slider::new(&mut trade_state.selected_percent, 1..=100).text("%")
            );
            columns[0].add_space(10.0);
            columns[0].horizontal(|ui| {
                let current_economy = (economy.egui_get(trade_state.selected_item) * trade_state.selected_percent as f64 / 100.0).floor();
                trade_state.selected_economy = current_economy;

                ui.horizontal_centered(|ui| {
                    ui.set_height(40.0);
                    ui.set_width(130.0);

                    ui.group(|ui| {
                        ui.set_height(50.0);
                        ui.set_width(50.0);
                        
                        ui.vertical_centered(|ui| {
                            ui.add_sized([40.0, 40.0], egui::Label::new(trade_state.selected_item.to_string()));
                            ui.add(egui::Label::new(format_number(current_economy)));
                        });
                    });

                    ui.centered_and_justified(|ui| {
                        ui.set_height(10.0);
                        ui.set_width(50.0);

                        ui.add(egui::Label::new("===>"));
                    });

                    ui.group(|ui| {
                        let trade = trade_well(&trade_state, &economy);

                        ui.set_height(50.0);
                        ui.set_width(50.0);

                        ui.vertical_centered(|ui| {
                            ui.add_sized([40.0, 40.0], egui::Label::new("😸"));
                            ui.add(egui::Label::new(format_number(trade)));
                        });
                    });
                });
            });
            columns[0].add_space(10.0);
            columns[0].group(|ui| {
                let trade = trade_well(&trade_state, &economy);

                if ui.add_sized([100.0, 40.0], egui::Button::new("Feed")).clicked() {
                    economy.add(0,  trade);
                    if trade_state.selected_item != EGUIResourceType::All {
                        economy.add(trade_state.selected_item as usize, -trade_state.selected_economy);
                    } else {
                        economy.add_all(trade_state.selected_percent as f64);
                    };
                };
            });

            columns[1].label("Inventory");
            columns[1].separator();
            columns[1].vertical(|ui| {
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
                };
            });
        })
    });
}


fn trade_well(
    trade_state: &TradeState,
    economy: &Economy,
) -> f64 {
    let mut cur_well = 0.0;

    let item_idx = trade_state.selected_item as usize;
    if item_idx > 0 {
        if let Some(well) = TRADEWELL.well.get(item_idx - 1) {
             cur_well = *well
        };
    };

    let trade = match trade_state.selected_item {
        EGUIResourceType::All => economy.get_egui_all(TRADEWELL, trade_state.selected_percent as f64),
        EGUIResourceType::None => 0.0,
        _ => cur_well * trade_state.selected_economy
    };

    trade
}

pub fn map_ui_system(
    mut contexts: EguiContexts,
    mut new_current_world: ResMut<NextState<CurrentWorld>>,
    current_world: Res<State<CurrentWorld>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 150),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new("")
    .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
    .fixed_size([320.0, 240.0])
    .collapsible(false)
    .title_bar(false)
    .resizable(false)
    .frame(my_frame)

    .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            let (response, painter) = ui.allocate_painter(
                [310.0, 230.0].into(),
                egui::Sense::click()
            );

            let min = response.rect.min;

            let room_point_sn = vec![
                min + egui::vec2(20.0, 20.0),
                min + egui::vec2(20.0, 120.0),
                min + egui::vec2(100.0, 120.0),
                min + egui::vec2(100.0, 80.0),
                min + egui::vec2(95.0, 80.0),
                min + egui::vec2(95.0, 50.0),
                min + egui::vec2(100.0, 50.0),
                min + egui::vec2(100.0, 20.0),
            ];

            let room_point_wpp = vec![
                min + egui::vec2(130.0, 20.0),
                min + egui::vec2(260.0, 20.0),
                min + egui::vec2(260.0, 120.0),
                min + egui::vec2(130.0, 120.0),
                min + egui::vec2(130.0, 100.0),
                min + egui::vec2(135.0, 100.0),
                min + egui::vec2(135.0, 70.0),
                min + egui::vec2(130.0, 70.0),
            ];

            let critical_point_sn = [20.0, 100.0, 20.0, 120.0];
            let critical_point_wpp= [130.0, 260.0, 20.0, 120.0];

            room_map_spawn(ui, &mut new_current_world, &current_world, room_point_sn, min.clone(), &painter, critical_point_sn, CurrentWorld::SunlitNursery);
            room_map_spawn(ui, &mut new_current_world, &current_world, room_point_wpp, min.clone(), &painter, critical_point_wpp, CurrentWorld::WarmPawsPorch);
        });
    });
}

fn room_map_spawn(
    ui: &mut egui::Ui,
    new_current_world: &mut NextState<CurrentWorld>,
    current_world: &CurrentWorld,
    room_point: Vec<egui::Pos2>,
    min: egui::Pos2,
    painter: &egui::Painter,
    critical_point: [f32; 4],
    location: CurrentWorld,
) {
    let mouse_pos = ui.input(|i| i.pointer.hover_pos());
    let is_click = ui.input(|i| i.pointer.any_click());

    let mut fill_color = if *current_world == location {
        egui::Color32::from_gray(60)
    } else {
        egui::Color32::from_gray(20)
    };
    
    if let Some(m_pos) = mouse_pos {
        if m_pos.x > min.x + critical_point[0] && m_pos.x < min.x + critical_point[1]
            && m_pos.y > min.y + critical_point[2] && m_pos.y < min.y + critical_point[3] {
            fill_color = egui::Color32::from_rgba_unmultiplied(140, 150, 75, 200);
                    
            if is_click && *current_world != location{
                new_current_world.set(location);
            };
        };
    };

    let center_x = min.x + (critical_point[0] + critical_point[1]) / 2.0;
    let center_y = min.y + (critical_point[2] + critical_point[3]) / 2.0;
    
    painter.add(egui::Shape::convex_polygon(
        room_point,
        fill_color,
           egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(100, 190, 50, 180))
        ));

    painter.text(
        egui::pos2(center_x, center_y),
        egui::Align2::CENTER_CENTER,
        location.to_string(),
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE
    );
}