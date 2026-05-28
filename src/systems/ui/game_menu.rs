use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui, egui::Response};
use crate::schema::{common::*, global_inventory::*, item_type_info::*, economy_inventory::*, prestige::*, upgrade_storege::*, hud::*, global_settings::*, save_file::*};
use crate::systems::{ui::main_menu::exit_clicked, save::*};
use crate::systems::locales::*;


pub fn game_menu(
    mut contexts: EguiContexts,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut exit_event: MessageWriter<AppExit>,
    mut menu_page: ResMut<MenuCurPage>,
    mut save_slot_inv: ResMut<SaveSlotInv>,
    mut up_storege: ResMut<UpgradeStorege>,
    mut global_storege: ResMut<GlobalInventory>,
    mut eco_storege: ResMut<Economy>,
    mut cit_storege: ResMut<ItemTypeInfo>,
    mut current_world: ResMut<NextState<CurrentWorld>>,
    mut pristige_room: ResMut<PrestigeRoom>,
    world: Res<State<CurrentWorld>>,
    game_state: Res<State<GameState>>,
    scale: Res<WorldScale>,
    settings: Res<GlobalSettings>,
) {
    if *game_state != GameState::Playing { return; };

    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let s = scale.0;

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new("Game Menu")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size([160.0 * s.x, 185.0 * s.y])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(my_frame)
        .show(ctx, |ui| {
            game_menu_button(ctx, &mut menu_page, s);

            if menu_page.game_menu {
                ui.allocate_ui(egui::vec2(150.0 * s.x, 175.0 * s.y), |ui| {
                    let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], egui::Button::new(translate("menu-continue", &settings.language)));

                    continue_clicked(&response, &mut menu_page);

                    let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], egui::Button::new(translate("menu-setting", &settings.language)));

                    exit_to_menu(
                        &response,
                        &mut current_world,
                        &mut game_next_state,
                        &mut menu_page,
                        &mut save_slot_inv,
                        &mut up_storege,
                        &mut global_storege,
                        &mut eco_storege,
                        &mut cit_storege,
                        &mut pristige_room,
                        &world,
                        false,
                    );

                    setting_clicked(&response, &mut menu_page);

                    let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], egui::Button::new(translate("menu-exit-menu", &settings.language)));

                    exit_to_menu(
                        &response,
                        &mut current_world,
                        &mut game_next_state,
                        &mut menu_page,
                        &mut save_slot_inv,
                        &mut up_storege,
                        &mut global_storege,
                        &mut eco_storege,
                        &mut cit_storege,
                        &mut pristige_room,
                        &world,
                        true,
                    );

                    let response = ui.add_sized([150.0 * s.x, 35.0 * s.y], egui::Button::new(translate("menu-exit-desktop", &settings.language)));

                    exit_clicked(&response, &mut exit_event);
                });
            };
        });
}


fn game_menu_button(
    ctx: & egui::Context,
    menu_page: &mut MenuCurPage,
    s: Vec2,
) {
    egui::Area::new(egui::Id::new("game_menu_area"))
    .fixed_pos([10.0 * s.x, 10.0 * s.y])
    .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let response = ui.add_sized([40.0 * s.x, 40.0 * s.y], egui::Button::new("||"));

            if response.clicked() {
                menu_page.game_menu = !menu_page.game_menu
            };
        });
}

fn continue_clicked(response: &Response, menu_page: &mut MenuCurPage) {
    if response.clicked() {
        menu_page.game_menu = !menu_page.game_menu
    };
}

fn exit_to_menu(
    response: &Response,
    current_world: &mut NextState<CurrentWorld>,
    game_next_state: &mut NextState<GameState>,
    menu_page: &mut MenuCurPage,
    save_slot_inv: &mut SaveSlotInv,
    up_storege: &mut UpgradeStorege,
    global_storege: &mut GlobalInventory,
    eco_storege: &mut Economy,
    cit_storege: &mut ItemTypeInfo,
    pristige_room: &mut PrestigeRoom,
    world: &State<CurrentWorld>,
    slot_reset: bool,
) {
    if response.clicked() {
        auto_save_system(&up_storege, &global_storege, &eco_storege, &cit_storege, &save_slot_inv, world, pristige_room);

        *up_storege = UpgradeStorege::default();
        *global_storege = GlobalInventory::default();
        *eco_storege = Economy::default();
        *cit_storege = ItemTypeInfo::default();
        *pristige_room = PrestigeRoom::default();
        current_world.set(CurrentWorld::WarmPawsPorch);

        if slot_reset {
            save_slot_inv.active_slot = None;
        };

        game_next_state.set(GameState::Menu);
        menu_page.game_menu = !menu_page.game_menu;
        menu_page.page = MenuPage::Main;
    };
}

fn setting_clicked(response: &Response, menu_page: &mut MenuCurPage,) {
    if response.clicked() {
        menu_page.page = MenuPage::Settings;
    };
}