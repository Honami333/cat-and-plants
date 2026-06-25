use std::time::Duration;
use strum::IntoEnumIterator;
use bevy::{asset::uuid::Uuid, camera::NormalizedRenderTarget, picking::pointer::{Location, PointerId}, platform::collections::HashMap, prelude::*, window::{NormalizedWindowRef, PrimaryWindow}};
use bevy_egui::{EguiContexts, egui, egui::Response};
use crate::schema::{world_components::*, common::*, global_inventory::*, global_settings::*, hud::*, resources::*, prestige::*, item_type_info::*, economy_inventory::*, upgrade_storege::*};
use crate::systems::{locales::*, ui::*, visuals::format_number};
use crate::content::world::sunlit_nursery::*;

const FRAME_ANIM_TIME: f64 = 0.8;


pub fn cat_happiness_market(
    mut contexts: EguiContexts,
    mut trade_state: ResMut<TradeState>,
    (mut assets_loaded, mut map_texture_id, mut map_layuot_id, mut map_market_sprite):
        (Local<bool>, Local<HashMap<usize, egui::TextureId>>, Local<HashMap<usize, TextureAtlasLayout>>, Local<HashMap<usize, (Handle<Image>, Handle<TextureAtlasLayout>)>>),
    (mut commands, mut gl_inventory, mut economy): 
        (Commands, ResMut<GlobalInventory>, ResMut<Economy>),
    (mut market_sprite_timer, mut market_sprite_frame, time):
        (Local<Timer>, Local<usize>, Res<Time>),
   ( mut iti_inventory, upgrade_storege, prestige_inv): 
    (ResMut<ItemTypeInfo>, Res<UpgradeStorege>, Res<PrestigeRoom>), 
    (mut button_sprite_map, game_assets):
        (Local<HashMap<TypePage, egui::TextureId>>, Res<GameAssets>),
    current_world: Res<State<CurrentWorld>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    query_page_item: Query<Entity, With<PageItem>>,
    assets: Res<AtlasAssets>,
    query_window: Query<Entity, With<PrimaryWindow>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    game_state: Res<State<GameState>>,
    scale: Res<WorldScale>,
    setting: Res<GlobalSettings>,
) {
    if *game_state.get() != GameState::Playing { return; };

    let Ok(window_entity) = query_window.single() else { return; };

    let Some(normalized_window) = bevy::window::WindowRef::Primary.normalize(Some(window_entity)) else { return; };

    let s = scale.0;

    if !*assets_loaded {
        market_sprite_timer.set_mode(TimerMode::Once);
        market_sprite_timer.set_duration(Duration::from_secs_f64(FRAME_ANIM_TIME));

        for type_page in TypePage::iter() {
            let image = type_page.get_button_sprite(&game_assets);

            let texture_id = contexts.add_image(EguiTextureHandle::Strong(image));

            button_sprite_map.insert(type_page, texture_id);
        };

        let assets_market_sprite = HashMap::from([
            (0_usize, (assets.tomato_pot_atlas.clone(), assets.common_layout_x128.clone())),
            (1_usize, (assets.cucumber_pot_atlas.clone(), assets.common_layout_x128.clone())),
            (2_usize, (assets.corn_pot_atlas.clone(), assets.common_layout_x128.clone())),
            (3_usize, (assets.pumpkin_pot_atlas.clone(), assets.common_layout_x128.clone())),
            (4_usize, (assets.pumpkin_pot_atlas.clone(), assets.common_layout_x128.clone())),
        ]);

        for i in 0..assets_market_sprite.len() {
            let Some(sprite_and_layout) = assets_market_sprite.get(&i) else { continue; };

            let current= *map_texture_id.get(&i).unwrap_or(&egui::TextureId::default());

            let (new_bool, Some(atlas_layout), text_id) = 
            func_assets_loaded(
                *assets_loaded,
                current,
                &mut contexts,
                &layouts,
                sprite_and_layout.0.clone(),
                &sprite_and_layout.1,
            ) else { continue; };

            
            map_texture_id.insert(i, text_id);
            map_layuot_id.insert(i, atlas_layout.clone());

            if i == assets_market_sprite.len() - 1 {
                *assets_loaded = new_bool;
            };
        };

        *map_market_sprite = assets_market_sprite;
    };

    market_sprite_timer.tick(time.delta());

    if market_sprite_timer.is_finished() {
        *market_sprite_frame =( *market_sprite_frame + 1)%4;
        market_sprite_timer.reset();
    };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 200),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    let is_scroll = query_page_item.is_empty();

    egui::Window::new(translate("ui-cat-market", &setting.language))
        .enabled(is_scroll)
        .fixed_size([160.0 * s.x, 200.0 * s.y])
        .default_open(false) 
        .frame(my_frame)
        .show(ctx, |ui| {
            egui::ScrollArea::horizontal()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for tabs_ind in 0..trade_state.tabs.len() {
                            let Some((tab_world, _)) = trade_state.tabs.get(&tabs_ind) else { continue; };

                            if tab_world != current_world.get() { continue; };

                            let Some(tab_id_image) = map_texture_id.get(&tabs_ind) else { continue; };

                            let Some(tab_layout_image) = map_layuot_id.get(&tabs_ind) else { continue; };

                            let Some(sprite_atlas) = map_market_sprite.get(&tabs_ind) else { continue; };

                            let Some(image) = create_image(
                                *tab_id_image,
                                tab_layout_image,
                                *market_sprite_frame,
                                (64.0 * s.y, 64.0 * s.y),
                                s
                            ) else { continue; };

                            ui_build_from_market(
                                &mut trade_state,
                                &mut commands,
                                ui,
                                &mut gl_inventory,
                                &mut economy,
                                &mut iti_inventory,
                                &upgrade_storege,
                                &current_world,
                                &prestige_inv,
                                sprite_atlas,
                                query_page_item,
                                tabs_ind,
                                image,
                                &mouse_input,
                                window_entity,
                                normalized_window,
                                &setting,
                                &button_sprite_map,
                                s,
                            );
                        };
                    });
                });

            ui.allocate_space(ui.available_size());
        });
}

fn ui_build_from_market(
    trade_state: &mut TradeState,
    commands: &mut Commands,
    ui: &mut egui::Ui,
    gl_inventory: &mut GlobalInventory,
    economy: &mut Economy,
    iti_inventory: &mut ItemTypeInfo,
    upgrade_storege: &UpgradeStorege,
    current_world: &State<CurrentWorld>,
    prestige_inv: &PrestigeRoom,
    sprite_atlas: &(Handle<Image>, Handle<TextureAtlasLayout>),
    query_page_item: Query<Entity, With<PageItem>>,
    tabs_ind: usize,
    image: egui::Image,
    mouse_input: &ButtonInput<MouseButton>,
    window_entity: Entity,
    normalized_window: NormalizedWindowRef,
    setting: &GlobalSettings,
    button_sprite_map: &HashMap<TypePage, egui::TextureId>,
    s: Vec2,
) {
    let Some((tab_world, tab_info)) = trade_state.tabs.get(&tabs_ind) else { return; };

    let cit = &iti_inventory.item_count_inv;

    let rect_size = egui::vec2(160.0 * s.x, 200.0 * s.y);

    let rect = egui::Rect::from_min_size(ui.cursor().min, rect_size);

    let ui_builder = egui::UiBuilder::new().max_rect(rect);

    let mut page_plant_count = None;

    let mut max_page_plant_count = None;

    if let Ok(tp) = TypePlant::try_from(tab_info.type_page)
        && let Some(count) = cit.get(&tp) { page_plant_count = Some(*count)};

    if let Some(plant) = tab_info.type_page.get_plant_cfg() { max_page_plant_count = Some(plant.max_count); };

    let slot_count = gl_inventory.get_for_world(current_world).map(|inv| inv.len());
            
    let slot_count_unlock = if let Some(count) = gl_inventory.get_slots_unlocking(current_world) {
        Some(count) } else { slot_count };

    let slot_price = slot_price(gl_inventory, current_world, prestige_inv);

    let plant_price = plant_price(iti_inventory, current_world, &tab_info.type_page, prestige_inv);
        
    let price = match tab_info.type_page {
        TypePage::SlotsUnLocking => slot_price,
        TypePage::TomatoBuy => plant_price,
        TypePage::CucumberBuy => plant_price,
        TypePage::CornBuy => plant_price,
        TypePage::PumpkinBuy => plant_price,
    };

    let is_enabled_1 = page_plant_count != max_page_plant_count || page_plant_count.is_none();

    let is_enabled_2 = slot_count_unlock != slot_count  || slot_count_unlock.is_none() || tab_info.type_page != TypePage::SlotsUnLocking;


    ui.scope_builder(ui_builder, |page_ui| {
        page_ui.vertical_centered_justified(|page_ui| {
            let text = egui::RichText::new(translate(tab_info.title_key, &setting.language))
                .color(egui::Color32::GOLD)
                .heading();

            page_ui.label(text);

            page_ui.add_enabled_ui(is_enabled_1 && is_enabled_2, |page_ui| {
                let widget = egui::Button::image(image).fill(egui::Color32::TRANSPARENT);

                let response = page_ui.add_sized([64.0 * s.x, 64.0 * s.y], widget);

                if response.is_pointer_button_down_on() && query_page_item.is_empty() {
                    page_click_maneger(commands,
                        gl_inventory,
                        economy,
                        iti_inventory,
                        upgrade_storege,
                        current_world,
                        prestige_inv,
                        sprite_atlas,
                        query_page_item,
                        mouse_input,
                        window_entity,
                        normalized_window,
                        tab_info,
                        true,
                    );
                };

                hover_info(setting, response, tab_world, tab_info, true, price);
            });

            let mut color: egui::Color32;

            let mut text = egui::RichText::new("");


            if let (Some(count), Some(max_count)) = (page_plant_count, max_page_plant_count) {
                color = if count == max_count { egui::Color32::GOLD } else { egui::Color32::GREEN };

                text = egui::RichText::new(format!("{} / {}", count, max_count))
                    .color(color).size(18.0);
            };


            if tab_info.type_page == TypePage::SlotsUnLocking
                && let (Some(count), Some(max_count)) = (slot_count_unlock, slot_count) {
                    color = if count == max_count { egui::Color32::GOLD } else { egui::Color32::GREEN };

                    text = egui::RichText::new(format!("{} / {}", count, max_count))
                        .color(color).size(18.0);
                };

            page_ui.label(text);

            let Some(image_id) = button_sprite_map.get(&tab_info.type_page) else { return; };

            let widget = egui::Button::new("").min_size(egui::vec2(160.0 * s.x, 48.0 * s.y));

            let response = page_ui.add_enabled(is_enabled_1 && is_enabled_2, widget);

            let rect = response.rect;

            let tint = if is_enabled_1 && is_enabled_2 { egui::Color32::WHITE } else { egui::Color32::from_rgba_unmultiplied(255, 255, 255, 120)};

            let paiter = page_ui.painter();

            paiter.image(*image_id, rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), tint);

            paiter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                translate("fast_buy_one", &setting.language),
                egui::FontId::proportional(14.0),
                tint
            );

            if response.clicked() {
                    page_click_maneger(commands,
                        gl_inventory,
                        economy,
                        iti_inventory,
                        upgrade_storege,
                        current_world,
                        prestige_inv,
                        sprite_atlas,
                        query_page_item,
                        mouse_input,
                        window_entity,
                        normalized_window,
                        tab_info,
                        false,
                    );
                };

                hover_info(setting, response, tab_world, tab_info, false, price);
            });
            

        if trade_state.scroll_to_tab == Some(tabs_ind) {
            let rect = page_ui.max_rect();

            page_ui.scroll_to_rect(rect, Some(egui::Align::Center));                              
        };

        page_ui.allocate_space(page_ui.available_size());
    });
}

fn hover_info(
    setting: &GlobalSettings,
    response: Response,
    tab_world: &CurrentWorld,
    tab_info: &TradeTab,
    is_move_or_click: bool,
    price: Option<f64>,
) {
    response.on_hover_ui(|page_ui| {
        let lang = &setting.language;

        let text = egui::RichText::new(
            format!("{} {}", translate(tab_info.description, lang), translate(&tab_world.to_string(), lang)))
            .color(egui::Color32::GREEN)
            .heading();

        page_ui.label(text);

        if is_move_or_click {
            let text = egui::RichText::new(
                translate(tab_info.dragg_info, lang).to_string())
                .color(egui::Color32::RED)
                .size(16.0);

            page_ui.label(text);
        };

        page_ui.add_space(10.0);

        if let Some(price) = price {
            let text = egui::RichText::new(
                format!("{}: {}",
                    translate(ResourceType::CatHappiness.as_ref(), lang),
                    format_number(price)
                ))
                .color(egui::Color32::GOLD)
                .size(16.0);

            page_ui.label(text);
        };
    });
}

fn page_click_maneger(
    commands: &mut Commands,
    gl_inventory: &mut GlobalInventory,
    economy: &mut Economy,
    iti_inventory: &mut ItemTypeInfo,
    upgrade_storege: &UpgradeStorege,
    current_world: &State<CurrentWorld>,
    prestige_inv: &PrestigeRoom,
    sprite_atlas: &(Handle<Image>, Handle<TextureAtlasLayout>),
    query_page_item: Query<Entity, With<PageItem>>,
    mouse_input: &ButtonInput<MouseButton>,
    window_entity: Entity,
    normalized_window: NormalizedWindowRef,
    tab_info: &TradeTab,
    is_move_or_click: bool,
) {
    match tab_info.type_page {
        TypePage::TomatoBuy | TypePage::CucumberBuy | TypePage::CornBuy | TypePage::PumpkinBuy => {
            if is_move_or_click {
                buy_plant_button_down_drag(
                    commands,
                    sprite_atlas,
                    query_page_item,
                    mouse_input,
                    window_entity,
                    normalized_window,
                    tab_info.type_page
                );
             } else {
                add_plant_and_lock(
                    gl_inventory,
                    economy,
                    iti_inventory,
                    upgrade_storege,
                    current_world,
                   &tab_info.type_page,
                    prestige_inv,
                    None
                );
            };
        },
        TypePage::SlotsUnLocking => try_slots_unlocking(
            gl_inventory,
            economy,
            current_world,
            prestige_inv
        ),
    };
}

fn buy_plant_button_down_drag(
    commands: &mut Commands,
    sprite_atlas: &(Handle<Image>, Handle<TextureAtlasLayout>),
    query_page_item: Query<Entity, With<PageItem>>,
    mouse_input: &ButtonInput<MouseButton>,
    window_entity: Entity,
    normalized_window: NormalizedWindowRef,
    type_page_item: TypePage,
) {
    let spawned_entity  = commands.spawn((
        Sprite::from_atlas_image(
            sprite_atlas.0.clone(),
            TextureAtlas {
                    layout: sprite_atlas.1.clone(),
                    index: 3,
                },
        ),
        Transform::from_xyz(10000.0, 10000.0, 2.5),
        PageItem { type_page: type_page_item },
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
        )).id();

    let event = Pointer::<DragStart>::new(
        PointerId::Custom(Uuid::new_v4()),
        Location {
            target: NormalizedRenderTarget::Window(normalized_window),
            position: Vec2::ZERO,
        },
        DragStart{
                button: PointerButton::Primary,
                hit: bevy::picking::backend::HitData {
                    camera: window_entity,
                    depth: 0.0,
                        position: None,
                    normal: None
                },
            },
        spawned_entity,
    );

    commands.trigger(event);

    if !query_page_item.is_empty() && mouse_input.just_released(MouseButton::Left) {
        for entity in query_page_item.iter() {
            commands.entity(entity).try_despawn();
        };
    };
}


pub fn page_item_dragg_start(    
    trigger: On<Pointer<DragStart>>,
    query_page_item: Query<Entity, With<PageItem>>,
    mut dragged: ResMut<DragItem>,

) {
    let Ok(entity) = query_page_item.get(trigger.entity) else { return; };

    dragged.entity = Some(entity);
}

pub fn page_item_dragg(
    mut query_page_item: Query<(Entity, &mut Transform), With<PageItem>>,
    dragged: Res<DragItem>,
    window: Single<&Window, With<PrimaryWindow>>,
    scale: Res<WorldScale>,
) {
    if let Some(entity) = dragged.entity {
        let Ok((_, mut pi_trans)) = query_page_item.get_mut(entity) else { return; };

        let Some(mouse_pos) = window.cursor_position() else { return; };
        
        let s = scale.0;

        let base_pos =vec2(
            (mouse_pos.x - window.width() / 2.0) / s.x,
            (window.height() / 2.0 - mouse_pos.y) / s.y,
        );

        let z_index = 2.5 - (base_pos.y / 360.0);
        
        pi_trans.scale.x = s.x;
        pi_trans.scale.y = s.y;

        pi_trans.translation = (base_pos * (s.x).min(s.y)).extend(z_index);
    };
}

pub fn page_item_dragg_end(    
    _trigger: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut dragged: ResMut<DragItem>,
    mut gl_inventory: ResMut<GlobalInventory>,
    mut economy: ResMut<Economy>,
    mut iti_inventory: ResMut<ItemTypeInfo>,
    mut buy_plant: Local<bool>,
    upgrade_storege: Res<UpgradeStorege>,
    current_world: Res<State<CurrentWorld>>,
    prestige_inv: Res<PrestigeRoom>,
    query_page_item: Query<(Entity, &Transform, &PageItem)>,
    query_slots: Query<(&Transform, &Slot)>,
    scale: Res<WorldScale>,
) {
    let s = scale.0;

    if let Some(entity) = dragged.entity {
        for (slot_transform, slot_info) in query_slots {
            let Ok((_, pi_transform, pi_info)) = query_page_item.get(entity) else { continue; };

            if (pi_transform.translation.truncate() / (s.x).min(s.y))
                    .distance(slot_transform.translation.truncate() / (s.x).min(s.y)) 
                    < 10.0 * (s.x).min(s.y) && !*buy_plant {

                *buy_plant = true;

                add_plant_and_lock(
                    &mut gl_inventory,
                    &mut economy,
                    &mut iti_inventory,
                    &upgrade_storege,
                    &current_world,
                    &pi_info.type_page,
                    &prestige_inv,
                    Some(slot_info.id)
                );
            };
        };

        if query_page_item.get(entity).is_err() { return; };

        commands.entity(entity).try_despawn();

        dragged.entity = None;
        *buy_plant = false;
    };
}

fn add_plant_and_lock(
    gl_inventory: &mut GlobalInventory,
    economy: &mut Economy,
    iti_inventory: &mut ItemTypeInfo,
    upgrade_storege: &UpgradeStorege,
    current_world: &State<CurrentWorld>,
    page_data: &TypePage,
    prestige_inv: &PrestigeRoom,
    idx: Option<usize>,
) {
    if !gl_inventory.has_empty_slot(current_world) { return; };

    let availability = if let Some(upgrade_id) = page_data.get_dependencies_upgrade() {
        let (_, available) = upgrade_storege.get_global_modifier(upgrade_id);
        available
    } else {
        true
    };

    if !availability { return; };

    let Some(plant) = page_data.get_plant_cfg() else { return; };

    let Some(cur_price) = plant_price(iti_inventory, current_world, page_data, prestige_inv) else { return; };

    if economy.get_res(ResourceType::CatHappiness) < cur_price { return; };

    economy.add_res(ResourceType::CatHappiness, -cur_price);

    iti_inventory.add(plant.species_id, current_world);

    gl_inventory.add_plant(current_world, plant, idx);
}

fn plant_price(
    iti_inventory: &mut ItemTypeInfo,
    current_world: &State<CurrentWorld>,
    page_data: &TypePage,
    prestige_inv: &PrestigeRoom,
) -> Option<f64>{
    let Some(count_inv) = iti_inventory.get_for_world_mut(current_world) else { return None; };

    let Some(plant) = page_data.get_plant_cfg() else { return None; };

    let Some(plant_count) = count_inv.get(&plant.species_id) else { return None; };

    if *plant_count >= plant.max_count { return None; };

    let Some(prestige_room) = prestige_inv.get_room(current_world.get()) else { return None; };

    let cur_price = plant.price[*plant_count] * (1.0 + (prestige_room as f64).powf(1.6) * 3.0);

    Some(cur_price)
}

fn try_slots_unlocking(
    gl_inventory: &mut GlobalInventory,
    economy: &mut Economy,
    current_world: &State<CurrentWorld>,
    prestige_inv: &PrestigeRoom,
) {
    let Some(price) = slot_price(gl_inventory, current_world, prestige_inv) else { return; };

    let cat_happiness = economy.get_res(ResourceType::CatHappiness);

    gl_inventory.try_unlock_slot(current_world, cat_happiness, price, economy);
}

fn slot_price(
    gl_inventory: &mut GlobalInventory,
    current_world: &State<CurrentWorld>,
    prestige_inv: &PrestigeRoom,
) -> Option<f64> {
    let price = match current_world.get() {
        CurrentWorld::SunlitNursery => SLOT_PRICES,
        CurrentWorld::WarmPawsPorch => return None,
    };

    let mut new_price = Vec::new();

    let prestige_room = prestige_inv.get_room(current_world.get())?;

    for cost in price.prices.iter() {
        new_price.push(cost + (prestige_room as f64).powf(1.6) * 5500.0);
    };
     
    let i = gl_inventory.get_slots_unlocking(current_world)?;

    let price = new_price.get(i)?;

    Some(*price)
}