use crate::schema::{types_and_states::*, save_file::*};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::systems::locales::*;

pub fn map_ui_system(
    mut contexts: EguiContexts,
    mut new_current_world: ResMut<NextState<CurrentWorld>>,
    current_world: Res<State<CurrentWorld>>,
    world_scale: Res<WorldScale>,
    settings: Res<GlobalSettings>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let s = world_scale.scale / 2.0;

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 150),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new(translate("ui-map", &settings.language))
        .anchor(egui::Align2::RIGHT_TOP, [-10.0 * s, 10.0 * s])
        .fixed_size([320.0 * s, 240.0 * s])
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .constrain(true)
        .frame(my_frame)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let (response, painter) =
                    ui.allocate_painter(egui::vec2(310.0 * s, 230.0 * s), egui::Sense::click());

                let min = response.rect.min;

                let room_point_sn = vec![
                    min + egui::vec2(20.0, 20.0) * s,
                    min + egui::vec2(20.0, 120.0) * s,
                    min + egui::vec2(100.0, 120.0) * s,
                    min + egui::vec2(100.0, 80.0) * s,
                    min + egui::vec2(95.0, 80.0) * s,
                    min + egui::vec2(95.0, 50.0) * s,
                    min + egui::vec2(100.0, 50.0) * s,
                    min + egui::vec2(100.0, 20.0) * s,
                ];

                let room_point_wpp = vec![
                    min + egui::vec2(130.0, 20.0) * s,
                    min + egui::vec2(260.0, 20.0) * s,
                    min + egui::vec2(260.0, 120.0) * s,
                    min + egui::vec2(130.0, 120.0) * s,
                    min + egui::vec2(130.0, 100.0) * s,
                    min + egui::vec2(135.0, 100.0) * s,
                    min + egui::vec2(135.0, 70.0) * s,
                    min + egui::vec2(130.0, 70.0) * s,
                ];

                let critical_point_sn = [20.0 * s, 100.0 * s, 20.0 * s, 120.0 * s];
                let critical_point_wpp = [130.0 * s, 260.0 * s, 20.0 * s, 120.0 * s];

                room_map_spawn(
                    ui,
                    &mut new_current_world,
                    &current_world,
                    room_point_sn,
                    min.clone(),
                    &painter,
                    critical_point_sn,
                    CurrentWorld::SunlitNursery,
                    s,
                    &settings
                );
                room_map_spawn(
                    ui,
                    &mut new_current_world,
                    &current_world,
                    room_point_wpp,
                    min.clone(),
                    &painter,
                    critical_point_wpp,
                    CurrentWorld::WarmPawsPorch,
                    s,
                    &settings
                );
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
    s: f32,
    settings: &GlobalSettings
) {
    let mouse_pos = ui.input(|i| i.pointer.hover_pos());
    let is_click = ui.input(|i| i.pointer.any_click());

    let mut fill_color = if *current_world == location {
        egui::Color32::from_gray(60)
    } else {
        egui::Color32::from_gray(20)
    };

    if let Some(m_pos) = mouse_pos {
        if m_pos.x > min.x + critical_point[0]
            && m_pos.x < min.x + critical_point[1]
            && m_pos.y > min.y + critical_point[2]
            && m_pos.y < min.y + critical_point[3]
        {
            fill_color = egui::Color32::from_rgba_unmultiplied(140, 150, 75, 200);

            if is_click && *current_world != location {
                new_current_world.set(location);
            };
        };
    };

    let center_x = min.x + (critical_point[0] + critical_point[1]) / 2.0;
    let center_y = min.y + (critical_point[2] + critical_point[3]) / 2.0;

    painter.add(egui::Shape::convex_polygon(
        room_point,
        fill_color,
        egui::Stroke::new(
            2.0,
            egui::Color32::from_rgba_unmultiplied(100, 190, 50, 180),
        ),
    ));

    painter.text(
        egui::pos2(center_x, center_y),
        egui::Align2::CENTER_CENTER,
        translate(location.to_string().as_str(), &settings.language),
        egui::FontId::proportional(12.0 * s),
        egui::Color32::WHITE,
    );
}
