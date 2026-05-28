use bevy::{prelude::*};
use bevy_egui::{EguiPrimaryContextPass, egui::{self, Context}, EguiContexts, EguiTextureHandle};
use crate::schema::{resources::FontAssets, common::GameState};

mod eco_inventory;
mod game_menu;
mod loading;
mod main_menu;
mod map;
mod market;
mod nurturing;
mod prestige;
mod upgrades;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, loading::assets_load_screen.run_if(in_state(GameState::Loading)));
        
        app.add_systems(
            EguiPrimaryContextPass,
            (
                eco_inventory::economy_inventory,
                nurturing::trading_ui_system,
                map::map_ui_system,
                market::cat_happiness_market,
                upgrades::show_upgrade_grid,
                prestige::prestige_flag,
                game_menu::game_menu,
            )
                .chain()
                .run_if(in_state(GameState::Playing))
        );

        app.add_systems(EguiPrimaryContextPass, (
                main_menu::main_menu
            ).run_if(in_state(GameState::Menu))
        );

        app.add_systems(Update, (
            eco_inventory::animate_counters.run_if(in_state(GameState::Playing)),
            market::page_item_dragg
        ));

        app.add_observer(market::page_item_dragg_start);
        app.add_observer(market::page_item_dragg_end);
    }
}

pub fn func_fonts_loaded(ctx: &mut Context, fonts_loaded: bool, all_fonts: &Assets<Font>, font: &FontAssets) -> bool {
     if !fonts_loaded {
        let Some(font_data) = all_fonts.get(&font.emoji_font) else { return false; };

        let mut fonts = egui::FontDefinitions::default();

        let bytes = (*font_data.data).clone();

        fonts.font_data.insert(
            "f".to_owned(),
            std::sync::Arc::new(egui::FontData::from_owned(bytes)),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "f".to_owned());

            ctx.set_fonts(fonts);

        true
    } else { false }
}

pub fn create_image<'a> (
    handle_texture_id: egui::TextureId,
    atlas_layout: &'a TextureAtlasLayout,
    i: usize,
    size: (f32, f32),
    s: Vec2
) -> Option<egui::Image<'a>> {
    let Some(rect) = atlas_layout.textures.get(i) else { return None;};

    let atlas_size = atlas_layout.size.as_vec2();

    let uv  = egui::Rect::from_min_max(
        egui::pos2(rect.min.x as f32 / atlas_size.x, rect.min.y as f32 / atlas_size.y), 
        egui::pos2(rect.max.x as f32 / atlas_size.x, rect.max.y as f32 / atlas_size.y)
    );

    let image = egui::Image::new(egui::load::SizedTexture::new(
        handle_texture_id,
        [size.0 * s.x, size.1 * s.y],
    )).uv(uv)
    .bg_fill(egui::Color32::TRANSPARENT);

    Some(image)
}

pub fn func_assets_loaded<'a> (
    mut assets_loaded: bool,
    mut handle_texture_id: egui::TextureId,
    contexts: &mut EguiContexts,
    layouts: &'a Assets<TextureAtlasLayout>,
    get_image: Handle<Image>,
    get_layouts: &Handle<TextureAtlasLayout>,
) -> (bool, Option<&'a TextureAtlasLayout>, egui::TextureId) {
    if !assets_loaded {
        assets_loaded = true;

        handle_texture_id = contexts.add_image(EguiTextureHandle::Strong(get_image.clone()));
    }

    let atlas_layout = layouts.get(get_layouts);

    return (assets_loaded, atlas_layout, handle_texture_id);
}