use crate::schema::{config::*, world_components::*, common::*, global_inventory::*, resources::*};
use crate::systems::lifecycle::item_spawn;
use bevy::prelude::*;

// Обновление визуала в инвенторе
pub fn sync_inventory_visuals(
    mut shader_set: ParamSet<(
        ResMut<Assets<BreezeShaderMaterial>>,
        ResMut<Assets<Mesh>>,
        Res<Assets<TextureAtlasLayout>>,
    )>,
    mut query_items: Query<(Entity, &mut SlotItem)>,
    mut commands: Commands,
    mut inv: ResMut<GlobalInventory>,
    assets: Res<AtlasAssets>,
    query_slots: Query<&Slot>,
    current_world: Res<State<CurrentWorld>>,
) {
    let Some(inventory) = inv.get_for_world_mut(&current_world) else { return; };

    // Удаление призраков
    for (entity, item) in query_items.iter() {
        let is_still_in_inventory = inventory.iter().any(|(_, slot)| {
            if let SlotState::Occupied(plant) = slot {
                plant.slot_uid == item.uid
            } else {
                false
            }
        });

        if !is_still_in_inventory {
            commands.entity(entity).try_despawn();
        }
    }

    // Спаун преметов в слотах
    for i in 0..16 {
        let Some(slot_state) = inventory.get_mut(&i) else { continue; };
    
        let SlotState::Occupied(plant) = slot_state else { continue; };

        let Some(target_slot) = query_slots.iter().find(|s| s.id == i)  else { continue; };
                
        let existing_item = query_items
            .iter_mut()
            .find(|(_, item)| item.uid == plant.slot_uid);

        if let Some((entity,  item)) = existing_item {
            if item.slot_id != i {
                plant.slot_uid = i;

                commands.entity(entity).despawn();

                item_spawn(
                    &mut commands,
                    &assets,
                    &query_slots,
                    i,
                    plant.species_id,
                    &mut shader_set,
                );
            }
        } else {
            item_spawn(
                &mut commands,
                &assets,
                &query_slots,
                target_slot.id,
                plant.species_id,
                &mut shader_set,
            );
        };
    }
}

// Обновление визула роста ростения
pub fn update_plant_appearance(
    mut shader_set: ParamSet<(
        ResMut<Assets<BreezeShaderMaterial>>,
        ResMut<Assets<Mesh>>,
        Res<Assets<TextureAtlasLayout>>,
    )>,
    mut query_item: Query<(&MeshMaterial2d<BreezeShaderMaterial>, &SlotItem)>,
    mut inv: ResMut<GlobalInventory>,
    atlas_assets: Res<AtlasAssets>,
    current_world: Res<State<CurrentWorld>>,
) {
    let Some(inventory) =  inv.get_for_world_mut(&current_world) else { return; };

    for (material_handle, slot_info) in query_item.iter_mut() {
        let Some(slot_state) = inventory.get_mut(&slot_info.uid) else { continue; };

        let SlotState::Occupied(plant) = slot_state else { continue; };

        let layout_type = atlas_assets.common_layout_x128.clone();

        if let Some(layout) = shader_set.p2().get(&layout_type) {
            let index = plant.state.atlas_texture_id();

            if let Some(rect) = layout.textures.get(index) {
                let atlas_size = layout.size.as_vec2();

                let sprite_rect = Vec4::new(
                    rect.min.x as f32 / atlas_size.x,
                    rect.min.y as f32 / atlas_size.y,
                    rect.max.x as f32 / atlas_size.x,
                    rect.max.y as f32 / atlas_size.y
                );

                if let Some(material) = shader_set.p0().get_mut(&material_handle.0) {
                    material.sprite_rect = sprite_rect;
                };
            };
        };
    }
}

// Z сортировка, анимация перестаскивание и маштабировние предметов
pub fn grad_item_anim_and_zsort(
    scale: Res<WorldScale>,
    mut item_slot_query: Query<(&mut Transform, &SlotItem)>,
) {
    let s = scale.0;

    for (mut item_trans, item_info) in item_slot_query.iter_mut() {
        let z_index = 2.5 - (item_info.base_pos.y / 360.0);

        item_trans.scale.x = s.x;
        item_trans.scale.y = s.y;

        item_trans.translation = (item_info.base_pos * (s.x).min(s.y)).extend(z_index);
    }
}

// Обновление маштаба всей сцены
pub fn update_scene_scale(
    mut set: ParamSet<(
        Query<(&mut Transform, &Background)>,
        Query<(&mut Transform, &Slot)>,
        Query<(&mut Transform, &MeshMaterial2d<LightShaderMaterial>), With<ShaderMesh>>,
    )>,
    mut materials: ResMut<Assets<LightShaderMaterial>>,
    scale: ResMut<WorldScale>,
) {
    let s = scale.0;

    for (mut bg_trans, _) in set.p0().iter_mut() {
        bg_trans.scale.x = s.x;
        bg_trans.scale.y = s.y;
    };

    // Слоты
    for (mut slot_trans, slot_info) in set.p1().iter_mut() {
        slot_trans.translation = (slot_info.base_pos * s).extend(1.0);
        slot_trans.scale.x = s.x;
        slot_trans.scale.y = s.y;
    }

    // Шейдеры
    for (mut transform, material_handle) in set.p2().iter_mut() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.scale = (material.original_scale / s.x).min(material.original_scale / s.y);

            transform.scale = Vec3::splat((material.mesh_scale * s.x).min(material.mesh_scale * s.y));
        };
    };
}

pub fn shader_animation(
    mut materials: ResMut<Assets<LightShaderMaterial>>,
    shader_query: Query<&MeshMaterial2d<LightShaderMaterial>>,
    time: Res<Time>,
) {
    for material_handle in shader_query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let sin_time = (time.elapsed_secs() * 2.0).sin() * 0.001;
            
            material.color.set_alpha(material.color.alpha + sin_time);
        }
    }
}

pub fn format_number(n: f64) -> String {
    match n {
        x if x >= 1e15 => format!("{:.1}Q", n / 1e15),
        x if x >= 1e12 => format!("{:.1}T", n / 1e12),
        x if x >= 1e9 => format!("{:.1}B", n / 1e9),
        x if x >= 1e6 => format!("{:.1}M", n / 1e6),
        x if x >= 1e3 => format!("{:.1}K", n / 1e3),
        _ => format!("{:.0}", n),
    }
}