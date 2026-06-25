use bevy::{prelude::*, window::PrimaryWindow};
use std::fs::{metadata, remove_file};
use std::time::SystemTime;
use bevy_egui::{EguiContexts, egui, egui::Response};
use crate::schema::{common::*, economy_inventory::*, hud::*, global_settings::*, save_file::*, resources::*};
use crate::systems::{ui::*, save::*};
use chrono::{DateTime};
use crate::systems::locales::*;

const RESOLUTIONS: [([f32; 2], &str); 13] = [
    ([1024.0, 768.0], "1024 x 768"),
    ([1280.0, 800.0], "1280 x 800"),
    ([1280.0, 1024.0], "1280 x 1024"),
    ([1366.0, 768.0], "1366 x 768"),
    ([1440.0, 900.0], "1440 x 900"),
    ([1600.0, 900.0], "1600 x 900"),
    ([1600.0, 1200.0], "1600 x 1200"),
    ([1680.0, 1050.0], "1680 x 1050"),
    ([1920.0, 1080.0], "1920 x 1080"),
    ([1920.0, 1200.0], "1920 x 1200"),
    ([2560.0, 1440.0], "2560 x 1440"),
    ([2560.0, 1600.0], "2560 x 1600"),
    ([3840.0, 2160.0], "3840 x 2160"),
];

fn add_start(economy: &mut Economy) {
    // economy.add_res(ResourceType::CatHappiness, 1000000.0);
    // economy.add_res(ResourceType::Tomatoes, 1000000.0);
    // economy.add_res(ResourceType::Cucumbers, 1000000.0);
    // economy.add_res(ResourceType::Corn, 1000000.0);
    // economy.add_res(ResourceType::Pumpkin, 1000000.0);
    economy.add_res(ResourceType::Tomatoes, 10.0);
}

pub fn main_menu(
    (mut window_query, mut contexts, mut menu_page): (
        Query<&mut Window, With<PrimaryWindow>>,
        EguiContexts,
        ResMut<MenuCurPage>),
    mut assets_loaded: Local<bool>,
    mut handle_texture_id: Local<egui::TextureId>,
    mut save_slot_inv: ResMut<SaveSlotInv>,
    mut game_state: ResMut<NextState<GameState>>,
    mut economy: ResMut<Economy>,
    mut exit_event: MessageWriter<AppExit>,
    mut settings: ResMut<GlobalSettings>,
    mut fonts_loaded: Local<bool>,
    all_fonts: Res<Assets<Font>>,
    font: Res<FontAssets>,
    scale: Res<WorldScale>,
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

    let s = scale.0;

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    *fonts_loaded = func_fonts_loaded(ctx, *fonts_loaded, &all_fonts, &font);

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new("Main Menu")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size([640.0 * s.x, 360.0 * s.y])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(my_frame)
        .show(ctx, |ui| {
            let page = &mut menu_page.page;

            if let Ok(mut window) = window_query.single_mut() {
                back_button(ctx, s, page, &mut save_slot_inv, &mut game_state, &mut window, &mut settings);
            };

            match page {
                MenuPage::Main => create_main_menu(
                    ui,
                    page,
                    &mut save_slot_inv,
                    &mut game_state,
                    &mut economy,
                    &mut exit_event,
                    s,
                    &settings
                ),
                MenuPage::SaveSlot => create_save_menu(
                    ui,
                    &mut save_slot_inv,
                    &mut game_state, 
                    &mut economy,
                    atlas_layout,
                    &handle_texture_id,
                    s,
                    &settings
                ),
                MenuPage::Settings => setting_menu(
                    ui,
                    &mut settings,
                    s
                ),
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
    s: Vec2,
    settings: &GlobalSettings,
) {
    ui.allocate_ui(egui::vec2(160.0 * s.x, 185.0 * s.y), |ui| {
        ui.vertical_centered_justified(|ui| {
            let (i, is_continue) = continue_enabled();

            ui.add_enabled_ui(is_continue, |ui| {
                let button = egui::Button::new(translate("menu-continue", &settings.language));
            
                let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], button);

                continue_clicked(&response, save_slot_inv, game_state, i);
            });

            let (i, is_continue) = new_game_enabled();

            ui.add_enabled_ui(is_continue, |ui| {
                let button = egui::Button::new(translate("menu-new-game", &settings.language));
            
                let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], button);

                new_game_clicked(&response, save_slot_inv, game_state, economy, i);
            });

            let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], egui::Button::new(translate("menu-save", &settings.language)));

            clicked_page(&response, menu_page, MenuPage::SaveSlot);
            clicked_sss(&response, save_slot_inv);

            let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], egui::Button::new(translate("menu-setting", &settings.language)));

            clicked_page(&response, menu_page, MenuPage::Settings);

            let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], egui::Button::new(translate("menu-exit", &settings.language)));

            exit_clicked(&response, exit_event);
        });
    });
}

pub fn exit_clicked(response: &Response, exit_event: &mut MessageWriter<AppExit>,) {
    if response.clicked() {
        exit_event.write(AppExit::Success);
    };
}

fn continue_enabled() -> (usize, bool) {
    let mut newest_slot: Option<usize> = None;

    let mut latest_time: Option<SystemTime> = None;

    for i in 0..3 {
        let Ok(metadata) = metadata(get_save_path(i)) else { continue; };

        let Ok(modified_time) = metadata.modified() else { continue; };
        
        if latest_time.is_none() || modified_time > latest_time.unwrap() {
            latest_time = Some(modified_time);

            newest_slot = Some(i);
        };
    };

    if let Some(i) = newest_slot { (i, true)} else { (0, false)}
}


fn new_game_enabled() -> (usize, bool) {
    for i in 0..3 {
        if !get_save_path(i).exists() {
            return (i, true);
        };
    };
    
    (0, false)
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
    s: Vec2,
    settings: &GlobalSettings
) {
    ui.horizontal_centered(|ui| {
        for i in 0..3 {
            let Some(save_slot) = save_slot_inv.slot.get_mut(i) else { continue; };

            let stage = save_slot.stage;

            let Some(image) = create_image(
                *handle_texture_id,
                atlas_layout,
                stage as usize,
                 (100.0, 240.0),
                 s
            ) else { continue; };

            ui.allocate_ui(egui::vec2(100.0 * s.x, 300.0 * s.y), |ui| {
                ui.vertical_centered_justified(|ui| {
                    double_slot_click(ui, game_state, save_slot_inv, economy, image, i, s, settings);

                    match stage {
                        SlotTextureState::Empty => slot_save_button(ui, game_state, save_slot_inv, economy,  s, i, SvSlBT::Start, settings),
                        SlotTextureState::Occupied => {
                            slot_save_button(ui, game_state, save_slot_inv, economy,  s, i, SvSlBT::Continue, settings);
                            if let Some(del) = save_slot_inv.deleting_slot && del == i {
                                slot_save_delete(ui, save_slot_inv, i, s, settings);
                            } else {
                                slot_save_button(ui, game_state, save_slot_inv, economy,  s, i, SvSlBT::Delete, settings);
                            };
                        },
                    };
                });
            }) ;

            if i < 2 {
                ui.add_space(5.0 * (s.x).min(s.y));
            };
        };
    });
}
fn back_button(
    ctx: &egui::Context,
    s: Vec2,
    menu_page: &mut MenuPage,
    save_slot_inv: &mut SaveSlotInv,
    game_state: &mut NextState<GameState>,
    window: &mut Window,
    settings: &mut GlobalSettings
) {
    if *menu_page != MenuPage::Main{
        egui::Area::new(egui::Id::new("back_button_area"))
            .anchor(egui::Align2::RIGHT_TOP, [-10.0 * s.x, 10.0 * s.y])
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let text = if save_slot_inv.active_slot.is_some() { translate("ui-back-game", &settings.language) } else { translate("ui-back-menu", &settings.language) };

                let response = ui.add_sized([100.0 * s.x, 40.0 * s.y], egui::Button::new(text));

                if response.clicked() {
                    if save_slot_inv.active_slot.is_some() {
                        let Some(i) = save_slot_inv.active_slot else { return; };
                        continue_clicked(&response, save_slot_inv, game_state, i);
                    } else {
                        *menu_page = MenuPage::Main;
                    }
                };
                let path = get_setting_path();

                
                let _ = setting_save(window, settings, path);
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
    s: Vec2,
    settings: &GlobalSettings,
) {
    let Some(save_slot) = save_slot_inv.slot.get_mut(i) else { return; };
    
    ui.allocate_ui(egui::vec2(100.0 * s.x, 2.5 * s.y), |ui| {
        ui.group(|ui| {
            let color = if save_slot.last_data_text == "A game needs to be created" { egui::Color32::GRAY } else {egui::Color32::GREEN};
            
            let display_text = if save_slot.last_data_text == "A game needs to be created" { 
                format!("{}\n", translate("ui-need-create", &settings.language)) 
            } else { 
                save_slot.last_data_text.to_string().clone() 
            };
            
            ui.colored_label(color, display_text);
        });
    });

    let response = ui.add_sized([100.0 * s.x, 240.0 * s.y], egui::Button::image(image));

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
    s: Vec2,
    settings: &GlobalSettings,
) {
    ui.horizontal_centered(|ui| {
        let response = ui.add_sized([50.0 * s.x, 20.0 * s.y], egui::Button::new(translate("ui-save-delete-yes", &settings.language)));

        if response.clicked() {
            let _ = remove_file(get_save_path(i));
            save_slot_inv.deleting_slot = None;
            save_slot_inv.slot[i].stage = SlotTextureState::Empty;
            save_slot_inv.active_slot = None;
            scan_save_slots(save_slot_inv);
        };

        let response = ui.add_sized([50.0 * s.x, 20.0 * s.y], egui::Button::new(translate("ui-save-delete-no", &settings.language)));
            
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
    s: Vec2,
    i: usize,
    b_type: SvSlBT,
    settings: &GlobalSettings,
) {
    let response = ui.add_sized([100.0 * s.x, 20.0 * s.y], egui::Button::new(translate(b_type.to_string().as_str(), &settings.language)));

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
            slot.last_data_text = "A game needs to be created";
        };
    };
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
pub fn setting_menu(
    ui: &mut egui::Ui,
    settings: &mut GlobalSettings,
    s: Vec2,
) {
    egui::ScrollArea::vertical()
        .show(ui, |ui| {
            ui.allocate_ui(egui::vec2(600.0 * s.x, 360.0 * s.y), |ui| {
                    
                ui.heading(translate("ui-settings-title", &settings.language));

                ui.group(|ui| {
                    let color_heading = egui::Color32::GREEN;

                    ui.add_sized([100.0* s.x, 10.0 * s.y],
                        egui::Label::new(egui::RichText::new(translate("ui-settings-language", &settings.language)).color(color_heading).heading()));

                    separate(ui, s);

                    language_combo_box(ui, settings, s);

                    separate(ui, s);

                    ui.add_sized([100.0* s.x, 10.0 * s.y], 
                        egui::Label::new(egui::RichText::new(translate("ui-settings-global", &settings.language)).color(color_heading).heading()));

                    separate(ui, s);

                    select_setting_text(ui, &mut settings.fps.limit, s, &translate("ui-settings-fps-limit", &settings.language));

                    ui.add_enabled_ui(settings.fps.limit, |ui| {
                        fps_combo_box(ui, settings, s);
                    });

                    ui.add_space(5.0 * (s.x).min(s.y));

                    ui.group(|ui| {
                        ui.add_sized([110.0* s.x, 3.0 * s.y], egui::Label::new(translate("ui-settings-screen-mode", &settings.language)));
                    });
                    
                    ui.add_enabled_ui(!cfg!(target_os = "android"), |ui| {
                        screen_mode_combo_box(ui, settings, s);
                    });

                    ui.add_space(5.0 * (s.x).min(s.y));
                    
                    ui.group(|ui| {
                        ui.add_sized([110.0* s.x, 3.0 * s.y], egui::Label::new(translate("ui-settings-select-res", &settings.language)));
                    });

                    ui.add_enabled_ui(settings.display.screen_mode != ScreenMode::Fullscreen, |ui| {
                        resolution_combo_box(ui, settings, s);
                    });

                    separate(ui, s);

                    ui.add_sized([100.0* s.x, 10.0 * s.y], 
                        egui::Label::new(egui::RichText::new(translate("ui-settings-shaders", &settings.language)).color(color_heading).heading()));

                    separate(ui, s);

                    select_setting_text(ui, &mut settings.shader.light_shaders, s, &translate("ui-settings-light-shader", &settings.language));

                    ui.add_enabled_ui(settings.shader.light_shaders, |ui| {
                        select_setting_text(ui, &mut settings.shader.dust_particles, s, &translate("ui-settings-dust-particles", &settings.language));
                    });

                    ui.add_enabled_ui(settings.shader.dust_particles && settings.shader.light_shaders, |ui| {
                        ui.add_sized([120.0* s.x, 20.0 * s.y],               
                            egui::Slider::new(&mut settings.shader.dust_amount, 0.2..=1.0)
                            .step_by(0.05)
                            .text(translate("ui-settings-count", &settings.language))
                            .trailing_fill(true)
                            .custom_formatter(|val, _| format!("{val:.2}"))
                        );
                    });

                    select_setting_text(ui, &mut settings.shader.breeze_shaders, s, &translate("ui-settings-breeze-shader", &settings.language));

                    separate(ui, s);

                    ui.add_sized([100.0* s.x, 10.0 * s.y], 
                        egui::Label::new(egui::RichText::new(translate("ui-settings-save", &settings.language)).color(color_heading).heading()));
                    
                    separate(ui, s);

                    ui.group(|ui| {
                        ui.add_sized([110.0* s.x, 3.0 * s.y], egui::Label::new(translate("ui-settings-autosave-period", &settings.language)));
                    });

                    save_combo_box(ui, settings, s);
                });
            });
        });
}

fn language_combo_box(
    ui: &mut egui::Ui,
    settings: &mut GlobalSettings,
    s: Vec2,
) {
    let current_key = settings.language.to_string();

    let current_lang = settings.language;
    let combo = egui::ComboBox::from_id_salt("language_select")
        .selected_text(translate(&current_key, &settings.language))
        .width(120.0 * s.x);

    combo.show_ui(ui, |ui| {
        ui.selectable_value(
            &mut settings.language,
            Language::En,
            translate(&Language::En.to_string(), &current_lang)
        );
        ui.selectable_value(
            &mut settings.language,
            Language::Ru,
            translate(&Language::Ru.to_string(), &current_lang)
        );
    });
}

fn screen_mode_combo_box(
    ui: &mut egui::Ui,
    settings: &mut GlobalSettings,
    s: Vec2,
) {
    let current_key = settings.display.screen_mode.to_string();

    let combo = egui::ComboBox::from_id_salt("screen_mode_select")
        .selected_text(translate(&current_key, &settings.language))
        .width(120.0 * s.x);

    combo.show_ui(ui, |ui| {
        ui.selectable_value(
            &mut settings.display.screen_mode,
            ScreenMode::Windowed,
            translate(&ScreenMode::Windowed.to_string(), &settings.language)
        );
        ui.selectable_value(
            &mut settings.display.screen_mode,
            ScreenMode::Fullscreen,
            translate(&ScreenMode::Fullscreen.to_string(), &settings.language)
        );
    });

    if settings.display.screen_mode == ScreenMode::Fullscreen {
        settings.display.max_display = settings.display.resolution;
    };
}

fn fps_combo_box(
    ui: &mut egui::Ui,
    settings: &mut GlobalSettings,
    s: Vec2,
) {
    let combo = egui::ComboBox::from_id_salt("fps_select")
        .selected_text(format!("{} {:.0} fps", translate("ui-settings-select-fps", &settings.language), settings.fps.foces_fps))
        .width(120.0 * s.x);

    combo.show_ui(ui, |ui| {
        ui.selectable_value(&mut settings.fps.foces_fps, 30.0, "30 FPS");
        ui.selectable_value(&mut settings.fps.foces_fps, 60.0, "60 FPS");
        ui.selectable_value(&mut settings.fps.foces_fps, 120.0, "120 FPS");
        ui.selectable_value(&mut settings.fps.foces_fps, 144.0, "144 FPS");
    });
}

fn resolution_combo_box(
    ui: &mut egui::Ui,
    settings: &mut GlobalSettings,
    s: Vec2,
) {
    let combo = egui::ComboBox::from_id_salt("resolution_select")
        .selected_text(format!("{} {:.0}x{:.0}", translate("ui-settings-resolution-label", &settings.language), settings.display.resolution[0], settings.display.resolution[1]))
        .width(120.0 * s.x);

    combo.show_ui(ui, |ui| {
        for &(res, text) in &RESOLUTIONS {
            if res[0] * res[1] > settings.display.max_display[0] * settings.display.max_display[1] { continue; };

            ui.selectable_value(&mut settings.display.resolution, res, text);
        };
    });
}


fn save_combo_box(    
    ui: &mut egui::Ui,
    settings: &mut GlobalSettings,
    s: Vec2,
) {
    let ui_minut = translate("ui-settings-minute", &settings.language);

    let combo = egui::ComboBox::from_id_salt("save-time_select")
        .selected_text(format!("{} {} {}", translate("ui-settings-time-label", &settings.language), settings.autosave_interval / 60.0, ui_minut))
        .width(120.0 * s.x);
    
    combo.show_ui(ui, |ui| {
        ui.selectable_value(&mut settings.autosave_interval, 60.0, format!("1 {}", ui_minut));
        ui.selectable_value(&mut settings.autosave_interval, 120.0, format!("2 {}", ui_minut));
        ui.selectable_value(&mut settings.autosave_interval, 180.0, format!("3 {}", ui_minut));
        ui.selectable_value(&mut settings.autosave_interval, 240.0, format!("4 {}", ui_minut));
        ui.selectable_value(&mut settings.autosave_interval, 300.0, format!("5 {}", ui_minut));
        ui.selectable_value(&mut settings.autosave_interval, 600.0, format!("10 {}", ui_minut));
    });
}

fn select_setting_text(ui: &mut egui::Ui, setting_type: &mut bool, s: Vec2, text: &str) {
    ui.horizontal(|ui| {
        let response = ui.add_sized([100.0* s.x, 10.0 * s.y], egui::Button::new(text));

        if response.clicked() {
            *setting_type = !*setting_type;
        };

        ui.group(|ui| {
            let text = if *setting_type {
                "✓".to_string()
            } else {
                "✕".to_string()
            };
                                
            ui.add_sized([10.0 * s.x, 3.0 * s.y], egui::Label::new(text))
        });
    });
}


fn separate(ui: &mut egui::Ui, s: Vec2) {
    ui.add_space(2.5 * (s.x).min(s.y));
    ui.separator();
    ui.add_space(2.5 * (s.x).min(s.y));
}
