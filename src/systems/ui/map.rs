use crate::schema::{common::*, global_settings::*};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::systems::locales::*;

pub fn map_ui_system(
    mut contexts: EguiContexts,
    mut new_current_world: ResMut<NextState<CurrentWorld>>,
    current_world: Res<State<CurrentWorld>>,
    scale: Res<WorldScale>,
    settings: Res<GlobalSettings>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let s = scale.0;

    let my_frame = egui::Frame {
        fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 150),
        corner_radius: 10.0.into(),
        inner_margin: 5.0.into(),
        ..default()
    };

    egui::Window::new(translate("ui-map", &settings.language))
        .anchor(egui::Align2::RIGHT_TOP, [-5.0 * s.x, 5.0 * s.y])
        .fixed_size([160.0 * s.x, 120.0 * s.y])
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .constrain(true)
        .frame(my_frame)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let (response, painter) =
                    ui.allocate_painter(egui::vec2(155.0 * s.x, 115.0 * s.y), egui::Sense::click());

                let min = response.rect.min;

                let room_point_sn = vec![
                    min + egui::vec2(10.0 * s.x, 10.0 * s.y),
                    min + egui::vec2(10.0 * s.x, 60.0 * s.y),
                    min + egui::vec2(50.0 * s.x, 60.0 * s.y),
                    min + egui::vec2(50.0 * s.x, 40.0 * s.y),
                    min + egui::vec2(47.5 * s.x, 40.0 * s.y),
                    min + egui::vec2(47.5 * s.x, 25.0 * s.y),
                    min + egui::vec2(50.0 * s.x, 25.0 * s.y),
                    min + egui::vec2(50.0 * s.x, 10.0 * s.y),
                ];

                let room_point_wpp = vec![
                    min + egui::vec2(65.0 * s.x, 10.0 * s.y),
                    min + egui::vec2(130.0 * s.x, 10.0 * s.y),
                    min + egui::vec2(130.0 * s.x, 60.0 * s.y),
                    min + egui::vec2(65.0 * s.x, 60.0 * s.y),
                    min + egui::vec2(65.0 * s.x, 50.0 * s.y),
                    min + egui::vec2(67.5 * s.x, 50.0 * s.y),
                    min + egui::vec2(67.5 * s.x, 35.0 * s.y),
                    min + egui::vec2(65.0 * s.x, 35.0 * s.y),
                ];

                let critical_point_sn = [10.0 * s.x, 50.0 * s.x, 10.0 * s.y, 60.0 * s.y];
                let critical_point_wpp = [65.0 * s.x, 130.0 * s.x, 10.0 * s.y, 60.0 * s.y];

                room_map_spawn(
                    ui,
                    &mut new_current_world,
                    &current_world,
                    room_point_sn,
                    min,
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
                    min,
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
    s: Vec2,
    settings: &GlobalSettings
) {
    let mouse_pos = ui.input(|i| i.pointer.hover_pos());
    let is_click = ui.input(|i| i.pointer.any_click());

    let mut fill_color = if *current_world == location {
        egui::Color32::from_gray(60)
    } else {
        egui::Color32::from_gray(20)
    };

    if let Some(m_pos) = mouse_pos
        && m_pos.x > min.x + critical_point[0]
            && m_pos.x < min.x + critical_point[1]
            && m_pos.y > min.y + critical_point[2]
            && m_pos.y < min.y + critical_point[3]
        {
            fill_color = egui::Color32::from_rgba_unmultiplied(140, 150, 75, 200);

            if is_click && *current_world != location {
                new_current_world.set(location);
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
        egui::FontId::proportional(6.0 * (s.x).min(s.y)),
        egui::Color32::WHITE,
    );
}
