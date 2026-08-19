use crate::content::world::sunlit_nursery::*;
use crate::schema::{config::*, resources::*, world_components::*, common::*, global_inventory::*};

use bevy::prelude::*;


pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn menu_current_world(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    shaders: Res<ShaderAssets>,
) {
    bg_spawn(&mut commands, assets.dark_storage.clone(), 0.0);
    shader_spawn(&mut commands, &mut meshes, shaders.ds_light.clone());
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
            bg_spawn(&mut commands, assets.sunlit_nursery.clone(), 0.0);
            shader_spawn(&mut commands, &mut meshes, shaders.sn_window_light.clone());
            spawn_slots_grid(&mut commands, &SN_SLOT_CFG, assets.pot_stands.clone());
        },
        CurrentWorld::WarmPawsPorch => {
            bg_spawn(&mut commands,  assets.warm_paws_porch.clone(), 0.0);
            shader_spawn(&mut commands, &mut meshes, shaders.wpp_window_light.clone());
        },
        CurrentWorld::ShadowGreenhouse => {
            bg_spawn(&mut commands,  assets.shadow_greenhouse_base.clone(), 0.0);
            bg_spawn(&mut commands,  assets.shadow_greenhouse_shelf.clone(), 2.5);
            bg_spawn(&mut commands,  assets.shadow_greenhouse_photos.clone(), 18.0);
        }
    };
}

fn bg_spawn(
    commands: &mut Commands,
    bg_image: Handle<Image>,
    z_layer: f32
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, z_layer),
        Sprite::from_image(bg_image),
        Background,
        Room,
        CleanupMarker,
    ));
}

fn shader_spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    shaders: Handle<LightShaderMaterial>,
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
fn spawn_slots_grid(
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
    atlas_assets: &AtlasAssets,
    query_slots: &Query<&Slot>,
    target_id: usize,
    plant_type: TypePlant,
    shader_set: &mut ParamSet<(
        ResMut<Assets<BreezeShaderMaterial>>,
        ResMut<Assets<Mesh>>,
        Res<Assets<TextureAtlasLayout>>,
    )>,
) {
    let (image_handle, layout_type) = plant_type.get_plant_image(atlas_assets);

    if let Some(slot) = query_slots.iter().find(|slot| slot.id == target_id) {
        let mut sprite_rect = Vec4::ZERO;

        if let Some(layout) = shader_set.p2().get(&layout_type) {
            let index = 0;

            let mut rectangle = Rectangle::default();

            if let Some(rect) = layout.textures.get(index) {
                let rect_size = rect.size();
                rectangle = Rectangle::new(rect_size.x as f32, rect_size.y as f32);

                let atlas_size = layout.size.as_vec2();

                sprite_rect = Vec4::new(
                    rect.min.x as f32 / atlas_size.x,
                    rect.min.y as f32 / atlas_size.y,
                    rect.max.x as f32 / atlas_size.x,
                    rect.max.y as f32 / atlas_size.y
                );
            };

            let breeze_material = BreezeShaderMaterial {
                wind_speed: plant_type.get_wind_speed(),
                wind_strength: plant_type.get_wind_strength(),
                soil_line: plant_type.get_soil_line(),
                sprite_rect,
                texture: image_handle.clone(),
                breeze_shaders: 1,
            };

            let mesh_handle = shader_set.p1().add(rectangle);

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(shader_set.p0().add(breeze_material)),

                Transform::from_xyz(10000.0, 10000.0, 2.5),
                SlotItem {
                    uid: target_id,
                    base_pos: Vec2::new(10000.0, 10000.0),
                    slot_id: slot.id,
                },
                Visibility::Visible,
                CleanupMarker,
            ));
        };
    }
}

pub fn cleanup_system(mut commands: Commands, clean_query: Query<Entity, With<CleanupMarker>>) {
    for entity in clean_query.iter() {
        commands.entity(entity).try_despawn();
    }
}
