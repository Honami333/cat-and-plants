use crate::content::world::sunlit_nursery::*;
use crate::schema::{config::*, resources::*, world_components::*, common::*, global_inventory::*};
use bevy::prelude::*;


pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

// Общий спаун мира
pub fn spawn_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    shaders: Res<ShaderAssets>,
    current_world: Res<State<CurrentWorld>>,
) {
    match current_world.get() {
        CurrentWorld::SunlitNursery => {
            bg_spawn(&mut commands, assets.sunlit_nursery.clone());
            shader_spawn(&mut commands, &mut meshes, shaders.sn_window_light.clone());
            spawn_slots(&mut commands, &SN_SLOT_CFG, assets.pot_stands.clone());
        }
        CurrentWorld::WarmPawsPorch => {
            bg_spawn(
                &mut commands,
                assets.warm_paws_porch.clone(),
            );
            shader_spawn(&mut commands, &mut meshes, shaders.wpp_window_light.clone());
        }
    };
}

fn bg_spawn(
    commands: &mut Commands,
    bg_image: Handle<Image>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(bg_image),
        Background,
        Room,
        CleanupMarker,
    ));
}

fn shader_spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    shaders: Handle<ShaderMaterial>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(shaders),
        Transform::from_xyz(0.0, 0.0, 20.0),
        ShaderMesh,
        CleanupMarker,
    ));
}

// Спаун слотов в зависимости от мира
fn spawn_slots(
    commands: &mut Commands,
    config: &WorldSettingsSlot,
    slot_image_handle: Handle<Image>,
) {
    for row in 0..config.slot_grid_scale {
        for col in 0..config.slot_grid_scale {
            let pos =
                config.slot_start_pos + (config.step_x * row as f32) + (config.step_y * col as f32);

            commands.spawn((
                Transform::from_translation(pos.extend(1.0)),
                Sprite::from_image(slot_image_handle.clone()),
                Slot {
                    id: (row * config.slot_grid_scale + col) as usize,
                    base_pos: pos,
                },
                CleanupMarker,
            ));
        }
    }
}

// Спаун предметов
pub fn item_spawn(
    commands: &mut Commands,
    assets: &AtlasAssets,
    query_slots: &Query<&Slot>,
    target_id: usize,
    plant_type: TypePlant,
) {
    let (image_handle, layout_type) = plant_type.get_plant_image(assets);

    if let Some(slot) = query_slots.iter().find(|slot| slot.id == target_id) {
        commands.spawn((
            Sprite::from_atlas_image(
                image_handle,
                TextureAtlas {
                    layout: layout_type,
                    index: 0,
                },
            ),
            Transform::from_xyz(10000.0, 10000.0, 2.5),
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
            },
            SlotItem {
                uid: target_id,
                base_pos: Vec2::new(10000.0, 10000.0),
                slot_id: slot.id,
            },
            CleanupMarker,
        ));
    }
}

pub fn cleanup_system(mut commands: Commands, clean_query: Query<Entity, With<CleanupMarker>>) {
    for entity in clean_query.iter() {
        commands.entity(entity).try_despawn();
    }
}
