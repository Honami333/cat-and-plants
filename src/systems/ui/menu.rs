use bevy::prelude::*;
use std::fs::{metadata, remove_file};
use std::time::SystemTime;
use bevy_egui::{EguiContexts, egui, egui::Response};
use crate::schema::{types_and_states::*, resources::*, save_file::*};
use crate::systems::{ui::*, save::*};
use chrono::{DateTime};


pub fn game_menu(
    mut contexts: EguiContexts,
    mut menu_page: ResMut<MenuCurPage>,
    mut assets_loaded: Local<bool>,
    mut handle_texture_id: Local<egui::TextureId>,
    mut save_slot_inv: ResMut<SaveSlotInv>,
    mut game_state: ResMut<NextState<GameState>>,
    mut economy: ResMut<Economy>,
    mut exit_event: MessageWriter<AppExit>,
    world_scale: Res<WorldScale>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    assets: Res<AtlasAssets>,
) {
    let (new_bool, Some(atlas_layout), text_id) = 
        func_assets_loaded(
            *assets_loaded,
            *handle_texture_id,
            &mut contexts,
            &layouts,
            assets.save_slots_atlas.clone(),
            &assets.common_layout_x100_240,
        ) else { return; };
    
    *assets_loaded = new_bool;
    *handle_texture_id = text_id;

    let s = world_scale.scale;

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new("")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size([640.0 * s, 360.0 * s])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(my_frame)
        .show(ctx, |ui| {
            let page = &mut menu_page.page;

            back_button(ctx, s, page);

            match page {
                MenuPage::Main => create_main_menu(
                    ui, page,
                    &mut save_slot_inv,
                    &mut game_state,
                    &mut economy,
                    &mut exit_event,
                    s
                ),
                MenuPage::SaveSlot => create_save_menu(
                    ui,
                    &mut save_slot_inv,
                    &mut game_state, 
                    &mut economy,
                    atlas_layout,
                    &handle_texture_id,
                    s
                ),
                MenuPage::Settings => (),
            };
        });
}

fn create_main_menu(
    ui: &mut egui::Ui,
    menu_page: &mut MenuPage,
    save_slot_inv: &mut SaveSlotInv,
    game_state: &mut NextState<GameState>,
    economy: &mut Economy,
    exit_event: &mut MessageWriter<AppExit>,
    s: f32
) {
    ui.allocate_ui(egui::vec2(160.0 * s, 185.0 * s), |ui| {
        ui.vertical_centered_justified(|ui| {
            let (i, is_continue) = continue_enabled();

            ui.add_enabled_ui(is_continue, |ui| {
                let button = egui::Button::new("Continue");
            
                let response = ui.add_sized([150.0 * s, 35.0 * s], button);

                continue_clicked(&response, save_slot_inv, game_state, i);
            });

            let (i, is_continue) = new_game_enabled();

            ui.add_enabled_ui(is_continue, |ui| {
                let button = egui::Button::new("New Game");
            
                let response = ui.add_sized([150.0 * s, 35.0 * s], button);

                new_game_clicked(&response, save_slot_inv, game_state, economy, i);
            });

            let response = ui.add_sized([150.0 * s, 35.0 * s], egui::Button::new("Save"));

            clicked_page(&response, menu_page, MenuPage::SaveSlot);
            clicked_sss(&response, save_slot_inv);

            let response = ui.add_sized([150.0 * s, 35.0 * s], egui::Button::new("Setting"));

            clicked_page(&response, menu_page, MenuPage::Settings);

            let response = ui.add_sized([150.0 * s, 35.0 * s], egui::Button::new("Exit"));

            exit_clicked(&response, exit_event);
        });
    });
}

fn exit_clicked(response: &Response, exit_event: &mut MessageWriter<AppExit>,) {
    if response.clicked() {
        exit_event.write(AppExit::Success);
    };
}

fn continue_enabled() -> (usize, bool) {
    let mut newest_slot: Option<usize> = None;

    let mut latest_time: Option<SystemTime> = None;

    for i in 0..2 {
        let Ok(metadata) = metadata(get_save_path(i)) else { continue; };

        let Ok(modified_time) = metadata.modified() else { continue; };
        
        if latest_time == None || modified_time > latest_time.unwrap() {
            latest_time = Some(modified_time);

            newest_slot = Some(i);
        };
    };

    if let Some(i) = newest_slot { return (i, true); } else { return (0, false);}
}

fn new_game_enabled() -> (usize, bool) {
    for i in 0..3 {
        if !get_save_path(i).exists() {
            return (i, true);
        };
    };
    
    return (0, false)
}

fn continue_clicked(
    response: &Response,
    save_slot_inv: &mut SaveSlotInv,
    game_state: &mut NextState<GameState>,
    i: usize
) {
    if response.clicked() {
        game_state.set(GameState::LoadGame);
        save_slot_inv.active_slot = Some(i);
    };
}

fn new_game_clicked(
    response: &Response,
    save_slot_inv: &mut SaveSlotInv,
    game_state: &mut NextState<GameState>,
    economy: &mut Economy,
    i: usize,
) {
    if response.clicked() {
        game_state.set(GameState::Playing);
        save_slot_inv.active_slot = Some(i);
        add_start(economy);
                
        scan_save_slots(save_slot_inv);
    };
}

fn clicked_page(response: &Response, menu_page: &mut MenuPage, new_menu_page: MenuPage) {
    if response.clicked() {
        *menu_page = new_menu_page;
    };
}

fn clicked_sss(response: &Response, save_slot_inv: &mut SaveSlotInv) {
    if response.clicked() {
        scan_save_slots(save_slot_inv);
    };
}

fn create_save_menu(
    ui: &mut egui::Ui,
    save_slot_inv: &mut SaveSlotInv,
    game_state: &mut NextState<GameState>,
    economy: &mut Economy,
    atlas_layout: &TextureAtlasLayout,
    handle_texture_id: &egui::TextureId,
    s: f32,
) {
    ui.horizontal_centered(|ui| {
        for i in 0..3 {
            let Some(save_slot) = save_slot_inv.slot.get_mut(i) else { continue; };

            let stage = save_slot.stage;

            let image = create_image(*handle_texture_id, &atlas_layout, stage as usize,  (100.0, 240.0), s);

            ui.allocate_ui(egui::vec2(100.0 * s, 300.0 * s), |ui| {
                ui.vertical_centered_justified(|ui| {
                    double_slot_click(ui, game_state, save_slot_inv, economy, image, i, s);

                    match stage {
                        SlotTextureState::Empty => slot_save_button(ui, game_state, save_slot_inv, economy,  s, i, SvSlBT::Start),
                        SlotTextureState::Occupied => {
                            slot_save_button(ui, game_state, save_slot_inv, economy,  s, i, SvSlBT::Continue);
                            if let Some(del) = save_slot_inv.deleting_slot && del == i {
                                slot_save_delete(ui, save_slot_inv, i, s);
                            } else {
                                slot_save_button(ui, game_state, save_slot_inv, economy,  s, i, SvSlBT::Delete);
                            };
                        },
                    };
                });
            }) ;

            if i < 2 {
                ui.add_space(5.0 * s);
            };
        };
    });
}

fn back_button(ctx: &egui::Context, s: f32, menu_page: &mut MenuPage,) {
    if *menu_page != MenuPage::Main{
        egui::Area::new(egui::Id::new("back_button_area"))
            .fixed_pos(egui::pos2(10.0 * s, 10.0 * s))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let response = ui.add_sized([100.0 * s, 40.0 * s], egui::Button::new("Back to Menu"));

                if response.clicked() {
                    *menu_page = MenuPage::Main;
                };
            });
    };
}

fn double_slot_click(
    ui: &mut egui::Ui,
    game_state: &mut NextState<GameState>,
    save_slot_inv: &mut SaveSlotInv,
    economy: &mut Economy,
    image: egui::Image<'_>,
    i: usize,
    s: f32,
) {
    let Some(save_slot) = save_slot_inv.slot.get_mut(i) else { return; };
    
    ui.allocate_ui(egui::vec2(100.0 * s, 2.5 * s), |ui| {
        ui.group(|ui| {
            let color = if save_slot.last_data_text == "A game needs to be created\n" { egui::Color32::GRAY } else {egui::Color32::GREEN};
            
            ui.colored_label(color, save_slot.last_data_text);
        });
    });

    let response = ui.add_sized([100.0 * s, 240.0 * s], egui::Button::image(image));

    if response.clicked() {
        match save_slot.click {
            _ if save_slot.click < 1 => save_slot.click += 1,
            _ => {
                save_slot.click = 0;
                match save_slot.stage {
                    SlotTextureState::Empty => {
                        game_state.set(GameState::Playing);
                        save_slot_inv.active_slot = Some(i);
                        add_start(economy);
                    },
                    SlotTextureState::Occupied => {
                        game_state.set(GameState::LoadGame);
                        save_slot_inv.active_slot = Some(i);
                    },
                };
            },
        };
    };
}           

fn slot_save_delete(
    ui: &mut egui::Ui,
    save_slot_inv: &mut SaveSlotInv,
    i: usize,
    s: f32,
) {
    ui.horizontal_centered(|ui| {
        let response = ui.add_sized([50.0 * s, 20.0 * s], egui::Button::new("Yes, delete"));

        if response.clicked() {
            let _ = remove_file(get_save_path(i));
            save_slot_inv.deleting_slot = None;
            save_slot_inv.slot[i].stage = SlotTextureState::Empty;
            save_slot_inv.active_slot = None;
            scan_save_slots(save_slot_inv);
        };

        let response = ui.add_sized([50.0 * s, 20.0 * s], egui::Button::new("No"));
            
        if response.clicked() {
            save_slot_inv.deleting_slot = None;
        };
    });
}

fn slot_save_button(
    ui: &mut egui::Ui,
    game_state: &mut NextState<GameState>,
    save_slot_inv: &mut SaveSlotInv,
    economy: &mut Economy,
    s: f32,
    i: usize,
    b_type: SvSlBT,

) {
    let response = ui.add_sized([100.0 * s, 20.0 * s], egui::Button::new(b_type.to_string()));

    if response.clicked() { 
        match b_type {
            SvSlBT::Start =>{
                game_state.set(GameState::Playing);
                save_slot_inv.active_slot = Some(i);
                add_start(economy);
            },
            SvSlBT::Continue => {
                game_state.set(GameState::LoadGame);
                save_slot_inv.active_slot = Some(i);
            },
            SvSlBT::Delete => {
                save_slot_inv.deleting_slot = Some(i);
            },
        };
    };
}

pub fn scan_save_slots(save_slot_inv: &mut SaveSlotInv) {
    for i in 0..3 {
        let slot = &mut save_slot_inv.slot[i];

        slot.stage = if get_save_path(i).exists() {
            SlotTextureState::Occupied
        } else {
            SlotTextureState::Empty
        };

        if slot.stage == SlotTextureState::Occupied {
            slot.last_data_text = fetch_file_date_string(i);
        } else {
            slot.last_data_text = "A game needs to be created\n";
        };
    };
}

fn add_start(economy: &mut Economy) {
    economy.add(ResourceType::Tomatoes as usize, 10.0);
}

fn fetch_file_date_string(i: usize) -> &'static str {
    let path = get_save_path(i);

    if !path.exists() { return "No save data"; };

    let Ok(metadata) = metadata(path) else { return "Read error"; };

    let Ok(modified_data) = metadata.modified() else { return "Unknown date"; };

    let datatime: DateTime<chrono::Local> = modified_data.into();

    let formatted = datatime.format("%d.%m.%Y\n%H:%M").to_string();

    Box::leak(formatted.into_boxed_str())
}