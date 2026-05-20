use crate::schema::{config::ShaderMaterial, save_file::*, types_and_states::*};
use bevy::{platform::thread, prelude::*, window::{PrimaryWindow, WindowFocused}};
use std::time::{Duration, Instant};

// Механика роста
pub fn plant_growth(mut inv: ResMut<GlobalInventory>, upgrade_storege: Res<UpgradeStorege>) {
    let invetories = [&mut inv.sunlit_nursery_inv];

    let mut up_value_1 =  1.0;

    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::SelectiveBreeding) {up_value_1 = value};

    for inventory in invetories {
        for slot in inventory.iter_mut() {
            if let SlotState::Occupied(plant) = slot {
                let mut up_value_2 = 1.0;

                let mut modifier_unlocked = false;

                if let (Some(value), is_unlocked) = upgrade_storege.get_plant_global_modifier(&plant.species_id, PlantGGM::Growth) {
                    up_value_2 = value;
                    modifier_unlocked = is_unlocked;

                    if plant.growth_score < (plant.growth_thereshold / (up_value_1 * value)) {
                        plant.growth_score += plant.growth_rate;
                    }
                } else {
                    if plant.growth_score < (plant.growth_thereshold / up_value_1) {
                        plant.growth_score += plant.growth_rate;
                    }
                };

                let p = if modifier_unlocked {
                    (plant.growth_score / (plant.growth_thereshold / (up_value_1 * up_value_2))).clamp(0.0, 1.0)
                } else {
                    (plant.growth_score / (plant.growth_thereshold / up_value_1)).clamp(0.0, 1.0)
                };

                plant.state = match p {
                    _ if p <= 0.25 => PlantStateGrowth::Seed,
                    _ if p <= 0.5 => PlantStateGrowth::Sprout,
                    _ if p < 1.0 => PlantStateGrowth::Sapling,
                    _  => PlantStateGrowth::Mature,
                }
            }
        }
    }
}


pub fn set_global_scale(
    mut world_scale: ResMut<WorldScale>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let s  = (window.width() / 640.0).min(window.height() / 360.0);

    world_scale.scale = s;
}

pub fn monitor_window_settings(
    window: &mut Window,
    settings: &mut GlobalSettings,
) {
    window.mode = match settings.display.screen_mode {
        ScreenMode::Fullscreen => bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        ScreenMode::Windowed => bevy::window::WindowMode::Windowed,
    };

    if settings.display.screen_mode == ScreenMode::Fullscreen { 
        settings.display.resolution[0] = window.resolution.width() as f32;
        settings.display.resolution[1] = window.resolution.height() as f32;
        return; 
    };

    window.resolution.set(settings.display.resolution[0], settings.display.resolution[1]);
}

pub fn max_fps_sync(
    mut focus_events: MessageReader<WindowFocused>,
    mut settings: ResMut<GlobalSettings>,
    mut focus: Local<bool>,
) {
    if !settings.fps.limit { return; };

    for event in focus_events.read() {
        if !event.focused {
            if settings.fps.max_fps != settings.fps.unfoces_fps {
                settings.fps.max_fps = settings.fps.unfoces_fps; 

                *focus = false;
            };

        } else { *focus = true };
    };

    if !*focus { return; };

    if settings.fps.max_fps != settings.fps.foces_fps {
        settings.fps.max_fps = settings.fps.foces_fps; 
    };
}

pub fn fps_limiter_system(
    mut last_frame_time: Local<Option<Instant>>,
    settings: Res<GlobalSettings>,
) {
    if !settings.fps.limit {
        *last_frame_time = None;
        return;
    };

    let now = Instant::now();

    let Some(last_time)  = *last_frame_time else { 
        *last_frame_time = Some(Instant::now());
        return; 
    };

    let elapsed = now.duration_since(last_time);

    let target_frame_duration = Duration::from_secs_f64(1.0 / settings.fps.max_fps);

    if elapsed < target_frame_duration {
        thread::sleep(target_frame_duration - elapsed);
    };

    *last_frame_time = Some(Instant::now());
}

pub fn update_shader_settings(
    mut materials: ResMut<Assets<ShaderMaterial>>,
    settings: Res<GlobalSettings>,
    query_shader_material: Query<&MeshMaterial2d<ShaderMaterial>>,
) {
    for material_handle in query_shader_material.iter() {
       let Some(material) =  materials.get_mut(material_handle) else { continue; };

       material.light_shaders = settings.shader.light_shaders.into();
       material.dust_particles = settings.shader.dust_particles.into();
       material.dust_amount = settings.shader.dust_amount;
    }
}