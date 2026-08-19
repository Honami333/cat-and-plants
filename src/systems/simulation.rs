use crate::schema::config::BreezeShaderMaterial;
use crate::schema::{config::LightShaderMaterial, world_components::SlotItem};
use crate::schema::{global_settings::*, common::*, global_inventory::*, upgrade_storege::*};
use bevy::platform::collections::HashMap;
use bevy::window::NormalizedWindowRef;
use bevy::{asset::uuid::Uuid, camera::NormalizedRenderTarget, picking::pointer::{Location, PointerId}, platform::thread, prelude::*, window::{PrimaryWindow, WindowFocused}};
use std::time::{Duration, Instant};

// Механика роста
pub fn plant_growth(mut inv: ResMut<GlobalInventory>, upgrade_storege: Res<UpgradeStorege>) {
    let invetories = [&mut inv.sunlit_nursery_inv];

    let mut up_value_1 =  1.0;

    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::SelectiveBreeding) {up_value_1 = value};

    for inventory in invetories {
        for i in 0..16 {
             let Some(slot) = inventory.get_mut(&i) else { continue; };

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
    mut scale: ResMut<WorldScale>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let s  = Vec2::from([window.width() / 640.0, window.height() / 360.0]);

    scale.0 = s;
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
        settings.display.resolution[0] = window.resolution.width();
        settings.display.resolution[1] = window.resolution.height();
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
    mut light_materials: ResMut<Assets<LightShaderMaterial>>,
    mut breeze_materials: ResMut<Assets<BreezeShaderMaterial>>,
    settings: Res<GlobalSettings>,
    query_light_shader: Query<&MeshMaterial2d<LightShaderMaterial>>,
    query_breeze_shader: Query<&MeshMaterial2d<BreezeShaderMaterial>>,
) {
    for material_handle in query_light_shader.iter() {
       let Some(material) =  light_materials.get_mut(material_handle) else { continue; };

       material.light_shaders = settings.shader.light_shaders.into();
       material.dust_particles = settings.shader.dust_particles.into();
       material.dust_amount = settings.shader.dust_amount;
    };

    for material_handle in query_breeze_shader {
        let Some(material) =  breeze_materials.get_mut(material_handle) else { continue; };

        material.breeze_shaders = settings.shader.breeze_shaders.into();
    };
}

pub fn corn_boom_ability(
    mut commands: Commands,
    upgrade_storege: Res<UpgradeStorege>,
    current_world: Res<State<CurrentWorld>>,
    global_inventory: ResMut<GlobalInventory>,
    query_slot_item: Query<(Entity, &SlotItem)>,
    query_window: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(gl_inv) = global_inventory.get_for_world(&current_world) else { return; };

    let Ok(window_entity) = query_window.single() else { return; };

    let Some(normalized_window) = bevy::window::WindowRef::Primary.normalize(Some(window_entity)) else { return; };
}

fn boom_harvest(
    commands: &mut Commands,
    inventory: &HashMap<usize, SlotState>,
    query_slot_item: &Query<(Entity, &SlotItem)>,
    normalized_window: NormalizedWindowRef,
    window_entity: Entity,
) {
    for slot_state in inventory.values() {
        if let SlotState::Occupied(plant) = slot_state {
            let Some((entity, _)) = query_slot_item.iter().find(|(_, si)| si.slot_id == plant.slot_uid) else { continue; };

            let event = Pointer::<Click>::new(
                PointerId::Custom(Uuid::new_v4()),
                Location {
                    target: NormalizedRenderTarget::Window(normalized_window),
                    position: Vec2::ZERO,
                },
                Click { 
                    button: PointerButton::Primary,
                    hit: bevy::picking::backend::HitData {
                        camera: window_entity,
                        depth: 0.0,
                        position: None,
                        normal: None
                    },
                    duration: std::time::Duration::ZERO,
                },
                entity,
            );

            commands.trigger(event);
        };
    };
}