use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::ui::UiScale;
use crate::schema::{resources::*, config::*, types_and_states::*, world_components::*};
use crate::systems::lifecycle::item_spawn;
use crate::world::{SUNLIT_NURSERY_PLANT_RESOURCE};


// Обновление визуала в инвенторе
pub fn sync_inventory_visuals(
    mut query_items: Query<(Entity, &mut SlotItem)>,
    mut commands: Commands,
    mut inv: ResMut<GlobalInventory>,
    assets: Res<AtlasAssets>,
    query_slots: Query<&Slot>,
    current_world: Res<CurrentWorld>,
) {
    let inventory = current_world.get_inv_mut(&mut inv);
    // Спаун преметов в слотах
    for (idx, slot_state) in inventory.iter_mut().enumerate() {
        if let SlotState::Occupied(plant) = slot_state {
        if let Some(target_slot) = query_slots.iter().find( |s| s.id == idx) {

        let existing_item = query_items.iter_mut().find( | (_ , item) | item.uid == plant.slot_uid);

        if let Some((_, mut item)) = existing_item {
        if item.slot_id != idx {
            item.slot_id = idx;
            plant.slot_uid = idx;
        }} else {
            item_spawn(&mut commands, &assets, &query_slots, target_slot.id, plant.species_id);
        }}}
    }

    // Удаление призраков
    for (entity, item) in query_items.iter() {
        let is_still_in_inventory = inventory.iter().any( |slot| {
            if let SlotState::Occupied(plant) = slot {
                plant.slot_uid == item.uid
            } else {
                false
            }
        });

        if !is_still_in_inventory {
            commands.entity(entity).despawn();
        }
    }
}


// Обновление визула роста ростения
pub fn update_plant_appearance(
    mut query_item: Query<(&mut Sprite, &mut SlotItem)>,
    mut inv: ResMut<GlobalInventory>,
    _assets: Res<AtlasAssets>,
    current_world: Res<CurrentWorld>,
) {
    let inventory = current_world.get_inv_mut(&mut inv);

    for (mut sprite, slot_info) in query_item.iter_mut() {
        let Some(slot_state) = inventory.get_mut(slot_info.uid as usize) else { continue; };

        let SlotState::Occupied(plant) = slot_state else { continue; };

        if plant.state.check_state() == PlantStateUpdate::Growth {
            let Some(atlas) = &mut sprite.texture_atlas else { continue; };

            atlas.index = plant.state.atlas_texture_id() as usize + 1;

            plant.state = plant.state.next_state()
        } else if plant.state.check_state() == PlantStateUpdate::Idle {
            let Some(atlas) = &mut sprite.texture_atlas else { continue; };

            atlas.index = plant.state.atlas_texture_id() as usize;
        };
    }
}


// Обновление маштаба всей сцены
pub fn update_scene_scale(
    mut set: ParamSet<(
        Query<(&mut Transform, &mut ScaleBackground)>,
        Query<(&mut Transform, &Slot)>,
        Query<(&mut Transform, &SlotItem)>,
        Query<(&mut Transform, &MyButton)>,
        Query<&mut Transform, With<ShaderMash>>,
    )>,
    mut materials: ResMut<Assets<ShaderMaterial>>,
    mut ui_scale: ResMut<UiScale>,
    window: Single<&Window, With<PrimaryWindow>>,
    shader_query: Query<&MeshMaterial2d<ShaderMaterial>>,
    time: Res<Time>,
) {
    let mut scale: f32 = 1.0;

    // Задний фон
    for (mut bg_trans, mut bg_info) in set.p0().iter_mut() {
        scale = (window.width() / bg_info.wh.x).min(window.height() / bg_info.wh.y);
        bg_info.scale_bg = scale;
        bg_trans.scale = Vec3::splat(scale);
    }

    // Слоты
    for (mut slot_trans, slot_info) in set.p1().iter_mut() {
        slot_trans.translation = (slot_info.base_pos * scale).extend(1.0);
        slot_trans.scale = Vec3::splat(scale);
    }

    // Предметы
    for (mut item_trans, item_info) in set.p2().iter_mut() {
        let z_index = 2.5 - (item_info.base_pos.y / 360.0);
        
        item_trans.translation = (item_info.base_pos * scale).extend(z_index);
        item_trans.scale = Vec3::splat(scale);
    }

    // Кнопки
    for (mut button_trans, button_info) in set.p3().iter_mut() {
        button_trans.translation = (button_info.base_pos * scale).extend(5.0);
        button_trans.scale = Vec3::splat(scale);
    }

    // Шейдеры
    for material_handle in shader_query.iter() {
    if let Some(material) = materials.get_mut(&material_handle.0) {
        let sin_time = (time.elapsed_secs() * 2.0).sin() * 0.001;
        material.color.set_alpha(material.color.alpha + sin_time);

        material.scale = material.original_scale / scale;

        for mut transform in set.p4().iter_mut() {
            transform.scale = Vec3::splat(material.mash_scale * scale);
            }
        }
    }

    ui_scale.0 = scale;
}


pub fn update_resourse_text(
    mut text_query: Query<(&mut VisualCounter, &mut Text, &MyText)>,
    current_world: Res<CurrentWorld>,
    economy: Res<Economy>,
) {
    let plant_res = match *current_world {
        CurrentWorld::SunlitNursery => SUNLIT_NURSERY_PLANT_RESOURCE,
    };

    for (mut counter, mut text, marker) in text_query.iter_mut() {
        let i = marker.0;

        let (icon, resource_type) = match i {
            0 => ("😸", ResourceType::CatHappiness),
            1 => (plant_res.plant_icon0, plant_res.plant0),
            2 => (plant_res.plant_icon1, plant_res.plant1),
            3 => (plant_res.plant_icon2, plant_res.plant2),
            4 => (plant_res.plant_icon3, plant_res.plant3),
            _ => continue,
        };

        counter.target_value = economy.get(resource_type);
        
        if counter.display_value > 0.0 || resource_type == ResourceType::CatHappiness {
            let formatted_val = format_number(counter.display_value);
            

            text.0 = format!("{} {}", icon, formatted_val);
        } else {  text.0 = format!("") };
    }
}

pub fn animate_counters(
    time: Res<Time>,
    mut text_query: Query<(&mut VisualCounter, &MyText)>,
) {
    for (mut counter, _) in text_query.iter_mut() {
        if (counter.display_value - counter.target_value).abs() < 0.1 {
            counter.display_value = counter.target_value;
        } else {
            let step = (counter.target_value - counter.display_value) * time.delta_secs() as f64 * 5.0;
            counter.display_value += step;
        };
    }
}

fn format_number(n : f64) -> String {
    match n {
        x if x >= 1e15 => format!("{:.2}Q", n / 1e15),
        x if x >= 1e12 => format!("{:.2}T", n / 1e12),
        x if x >= 1e9 => format!("{:.2}B", n / 1e9),
        x if x >= 1e6 => format!("{:.2}M", n / 1e6),
        x if x >= 1e3 => format!("{:.2}K", n / 1e3),
        _ => format!("{:.0}", n)
    }
}