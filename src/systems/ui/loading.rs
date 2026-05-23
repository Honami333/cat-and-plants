use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{EguiContexts, egui};
use crate::schema::save_file::GlobalSettings;
use crate::schema::{types_and_states::*, resources::*};
use crate::content::loading_text::*;
use std::time;
use rand::seq::SliceRandom;
use crate::GAME_VERSION;
use crate::systems::locales::*;

const ERROR_TIME: f64 = 20.0;
const LOAD_TIME: f64 = 0.0;
const DOT_TIME: f64 = 0.1;

pub fn assets_load_screen(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,

    mut error_timer: Local<Timer>,
    mut load_timer: Local<Timer>,
    mut dot_ani_timer: Local<Timer>,
    mut current_text: Local<String>,
    mut current_state: Local<usize>,
    mut current_dots: Local<usize>,

    asset_server: Res<AssetServer>,
    settings: Res<GlobalSettings>,
    world: Res<WorldScale>,
    time: Res<Time>,

    game_assets: Option<Res<GameAssets>>,
    shaders_assets: Option<Res<ShaderAssets>>,
    atlas_assets: Option<Res<AtlasAssets>>,
    font_assets: Option<Res<FontAssets>>,
) {
    if *current_state == 0 {
        error_timer.set_duration(time::Duration::from_secs_f64(ERROR_TIME));
        load_timer.set_duration(time::Duration::from_secs_f64(LOAD_TIME));
        dot_ani_timer.set_duration(time::Duration::from_secs_f64(DOT_TIME));

        error_timer.set_mode(TimerMode::Once);
        load_timer.set_mode(TimerMode::Once);
        dot_ani_timer.set_mode(TimerMode::Repeating);

        *current_state = 1;

        if let Some(random_phrase) = STAGE_1_TEXTS.choose(&mut rand::thread_rng()) {
            *current_text = translate(random_phrase, &settings.language);
        };

        return;
    };

    error_timer.tick(time.delta());
    load_timer.tick(time.delta());
    dot_ani_timer.tick(time.delta());

    if dot_ani_timer.just_finished() {
        *current_dots = (*current_dots + 1) % 4;
    };

    let s = world.scale;

    render_ui_loading(&mut contexts, *current_state, current_text.clone(), *current_dots, s, &settings);

    if error_timer.is_finished() {
        let text = translate("err-init-resources", &settings.language);
        *current_text = text.clone();
        error!(text);
        return;
    };


    if *current_state == 1 {
        let Some(assets) = game_assets else { return; };

        let game_assets_array = &[
            &assets.pot_stands,

            &assets.sunlit_nursery,
            &assets.warm_paws_porch,

            &assets.button_buy_tomato,
            &assets.button_buy_cucumber,
            &assets.button_buy_corn,
            &assets.button_buy_pumpkin,
            &assets.button_slots_unlocking
        ];

        assets_load_meneger(
            game_assets_array,
            &asset_server,
            &mut current_text,
            &mut error_timer,
            &mut load_timer,
            &mut current_state,
            STAGE_2_TEXTS,
            2
        );

        return;
    };

    if *current_state == 2 {
        let Some(_assets) = shaders_assets else { return; };

        if !load_timer.is_finished() { return; };

        load_timer.reset();
        error_timer.reset();

        if let Some(random_phrase) = STAGE_3_TEXTS.choose(&mut rand::thread_rng()) {
            *current_text = translate(random_phrase, &settings.language);
        };

        *current_state = 3;
        return;
    };

    if *current_state == 3 {
        let Some(assets) = atlas_assets else { return; };

        let atlas_assets_array = &[
            &assets.pockets_of_improvements,
            &assets.save_slots_atlas,

            &assets.tomato_pot_atlas,
            &assets.cucumber_pot_atlas,
            &assets.corn_pot_atlas,
            &assets.pumpkin_pot_atlas,
        ];

        assets_load_meneger(
            atlas_assets_array,
            &asset_server,
            &mut current_text,
            &mut error_timer,
            &mut load_timer,
            &mut current_state,
            STAGE_4_TEXTS,
            4
        );

        return;
    };

    if *current_state == 4 {
        let Some(assets) = font_assets else { return; };

        let font_assets_array = &[
            &assets.emoji_font,
        ];

        assets_load_meneger(
            font_assets_array,
            &asset_server,
            &mut current_text,
            &mut error_timer,
            &mut load_timer,
            &mut current_state,
            STAGE_5_TEXTS,
            5
        );

        return;
    };

    if *current_state == 5 {
        if load_timer.is_finished() {
            load_timer.reset();
            error_timer.reset();

            *current_state = 6;
            return;
        }
    };

    if *current_state != 6 || !load_timer.is_finished() { return; };

    next_state.set(GameState::Menu);
}

fn assets_load_meneger<T: Asset>(
    assets_array: &[&Handle<T>],
    asset_server: &AssetServer,
    current_text: &mut String,
    error_timer: &mut Timer,
    load_timer: &mut Timer,
    current_state: &mut usize,
    stage_text: &[&str],
    new_state: usize,
) {
    let all_loaded = assets_array.iter().all(|h| {
        matches!(asset_server.get_load_state(*h), Some(LoadState::Loaded))
    });

    if !all_loaded || !load_timer.is_finished() { return; };

    load_timer.reset();
    error_timer.reset();
    *current_state = new_state;

    if let Some(random_phrase) = stage_text.choose(&mut rand::thread_rng()) {
        *current_text = random_phrase.to_string();
    };
}

fn render_ui_loading(
    contexts: &mut EguiContexts,
    current_state: usize,
    current_text: String,
    current_dots: usize,
    s: f32,
    settings: &GlobalSettings,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
        .show(ctx, |ui| {
            let dot = ".".repeat(current_dots);

            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.45);

                let loading_text = if current_state != 6 { 
                    if current_text.is_empty() {
                        dot.to_string()
                    } else {
                        format!("{}{} ", translate(&current_text, &settings.language), dot) 
                    }
                } else { 
                    translate("ui-load-lets-go", &settings.language) 
                };

                ui.label(egui::RichText::new(loading_text)
                    .size(24.0)
                    .color(egui::Color32::WHITE)
                    .strong()
                );
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(20.0 * s);

                ui.horizontal(|ui| {
                    ui.add_space(10.0 * s);

                    let load_stage_text = match current_state {
                        5 => format!("Cat and plant v{}\n{}{}", GAME_VERSION, translate("ui-load-final", &settings.language), dot),
                        6 => format!("Cat and plant v{}\n{}", GAME_VERSION, translate("ui-load-complete", &settings.language)),
                        _ => format!("Cat and plant v{}\n{} {}/4{}", GAME_VERSION, translate("ui-load-stage", &settings.language), current_state, dot),
                    };

                    ui.label(egui::RichText::new(load_stage_text)
                        .size(18.0)
                        .color(egui::Color32::WHITE)
                        .strong()
                    );
                });
            });
        });
}
