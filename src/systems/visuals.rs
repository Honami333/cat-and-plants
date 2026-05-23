use crate::content::world::sunlit_nursery::*;
use crate::content::world::warm_paws_porch::*;
use crate::schema::{config::*, resources::*, types_and_states::*, world_components::*, save_file::*};
use crate::systems::lifecycle::item_spawn;
use crate::systems::locales::*;
use bevy::prelude::*;
use bevy::ui::UiScale;
use bevy::ecs::relationship::Relationship;

// Обновление визуала в инвенторе
pub fn sync_inventory_visuals(
    mut query_items: Query<(Entity, &mut SlotItem)>,
    mut commands: Commands,
    mut inv: ResMut<GlobalInventory>,
    assets: Res<AtlasAssets>,
    query_slots: Query<&Slot>,
    current_world: Res<State<CurrentWorld>>,
) {
    let Some(inventory) = inv.get_inv_mut(&current_world) else { return; };

    // Удаление призраков
    for (entity, item) in query_items.iter() {
        let is_still_in_inventory = inventory.iter().any(|slot| {
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

    // Спаун преметов в слотах
    for (idx, slot_state) in inventory.iter_mut().enumerate() {
        let SlotState::Occupied(plant) = slot_state else { continue; };

        let Some(target_slot) = query_slots.iter().find(|s| s.id == idx)  else { continue; };
                
        let existing_item = query_items
            .iter_mut()
            .find(|(_, item)| item.uid == plant.slot_uid);

        if let Some((entity,  item)) = existing_item {
            if item.slot_id != idx {
                plant.slot_uid = idx;

                commands.entity(entity).despawn();

                item_spawn(
                    &mut commands,
                    &assets,
                    &query_slots,
                    idx,
                    plant.species_id,
                );
            }
        } else {
            item_spawn(
                &mut commands,
                &assets,
                &query_slots,
                target_slot.id,
                plant.species_id,
            );
        };
    }
}

// Обновление визула роста ростения
pub fn update_plant_appearance(
    mut query_item: Query<(&mut Sprite, &mut SlotItem)>,
    mut inv: ResMut<GlobalInventory>,
    current_world: Res<State<CurrentWorld>>,
) {
    let Some(inventory) =  inv.get_inv_mut(&current_world) else {
        return;
    };

    for (mut sprite, slot_info) in query_item.iter_mut() {
        let Some(slot_state) = inventory.get_mut(slot_info.uid as usize) else {
            continue;
        };

        let SlotState::Occupied(plant) = slot_state else {
            continue;
        };

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        atlas.index = plant.state.atlas_texture_id() as usize;
    }
}

// Z сортировка, анимация перестаскивание и маштабировние предметов
pub fn grad_item_anim_and_zsort(
    worlds: Res<WorldScale>,
    mut item_slot_query: Query<(&mut Transform, &SlotItem)>,
) {
    for (mut item_trans, item_info) in item_slot_query.iter_mut() {
        let z_index = 2.5 - (item_info.base_pos.y / 360.0);

        item_trans.scale = Vec3::splat(worlds.scale);
        item_trans.translation = (item_info.base_pos * worlds.scale).extend(z_index);
    }
}

// Обновление маштаба всей сцены
pub fn update_scene_scale(
    mut set: ParamSet<(
        Query<(&mut Transform, &ScaleBackground)>,
        Query<(&mut Transform, &Slot)>,
        Query<(&mut Transform, &MyButton)>,
        Query<(&mut Transform, &MeshMaterial2d<ShaderMaterial>), With<ShaderMesh>>,
    )>,
    mut materials: ResMut<Assets<ShaderMaterial>>,
    mut ui_scale: ResMut<UiScale>,
    world_scale: ResMut<WorldScale>,
) {
    let s = world_scale.scale;

    if let Ok((mut bg_trans, _)) = set.p0().single_mut() {
        bg_trans.scale = Vec3::splat(s);
    } else { return; }

    // Слоты
    for (mut slot_trans, slot_info) in set.p1().iter_mut() {
        slot_trans.translation = (slot_info.base_pos * s).extend(1.0);
        slot_trans.scale = Vec3::splat(s);
    }

    // Кнопки
    for (mut button_trans, button_info) in set.p2().iter_mut() {
        button_trans.translation = (button_info.base_pos * s).extend(5.0);
        button_trans.scale = Vec3::splat(s);
    }

    // Шейдеры
    for (mut transform, material_handle) in set.p3().iter_mut() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.scale = material.original_scale / s;

            transform.scale = Vec3::splat(material.mesh_scale * s);
        };
    };

    ui_scale.0 = s;
}

pub fn shader_animation(
    mut materials: ResMut<Assets<ShaderMaterial>>,
    shader_query: Query<&MeshMaterial2d<ShaderMaterial>>,
    time: Res<Time>,
) {
    for material_handle in shader_query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            match material.shader_type {
                0 => {
                    let sin_time = (time.elapsed_secs() * 2.0).sin() * 0.001;
                    material.color.set_alpha(material.color.alpha + sin_time);
                }
                1 => {
                    let sin_time = (time.elapsed_secs() * 2.0).sin() * 0.001;
                    material.color.set_alpha(material.color.alpha + sin_time);
                }
                _ => return,
            };
        }
    }
}

pub fn update_resourse_text(
    mut text_query: Query<(&mut VisualCounter, &mut Text2d, &EconomyText)>,
    current_world: Res<State<CurrentWorld>>,
    economy: Res<Economy>,
    settings: Res<GlobalSettings>,
) {
    let plant_res = match current_world.get() {
        CurrentWorld::SunlitNursery => SN_PLANT_RES,
        CurrentWorld::WarmPawsPorch => WPP_PLANT_RES,
    };


    for (mut counter, mut text, marker) in text_query.iter_mut() {
        let i = marker.0;

        let (icon, resource_type) = match i {
            0 => (translate("res-cat-happiness", &settings.language), ResourceType::CatHappiness),
            1 => (plant_res.plant_icon0.to_string(), plant_res.plant0),
            2 => (plant_res.plant_icon1.to_string(), plant_res.plant1),
            3 => (plant_res.plant_icon2.to_string(), plant_res.plant2),
            4 => (plant_res.plant_icon3.to_string(), plant_res.plant3),
            _ => continue,
        };

        counter.target_value = economy.get_item(resource_type as usize, false);

        if counter.display_value > 0.0 || resource_type == ResourceType::CatHappiness {
            let formatted_val = format_number(counter.display_value);

            text.0 = format!("{} {}", icon, formatted_val);
        } else {
            text.0 = format!("")
        };
    }
}


pub fn animate_counters(
    time: Res<Time>,
    mut text_query: Query<(&mut VisualCounter, &EconomyText)>,
) {
    for (mut counter, _) in text_query.iter_mut() {
        if (counter.display_value - counter.target_value).abs() < 0.1 {
            counter.display_value = counter.target_value;
        } else {
            let step =
                (counter.target_value - counter.display_value) * time.delta_secs() as f64 * 5.0;
            counter.display_value += step;
        };
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

pub fn price_button_text(
    mut query_button_text: Query<(&ChildOf, &mut Text2d, &ButtonText)>,
    query_button: Query<&MyButton>,
    current_world: Res<State<CurrentWorld>>,
    inventory: Res<GlobalInventory>,
    count_item_type: Res<ItemTypeInfo>,
    prestige_inv: Res<PrestigeRoom>,
    settings: Res<GlobalSettings>,
) {
    for (parent, mut text, info) in query_button_text.iter_mut() {
        let ButtonText(b_type) = info;

        let my_parent;

        if let Ok(parent_data) = query_button.get(parent.get()) { my_parent = parent_data.text } else { my_parent = ""; };

        let Some(count_item) = count_item_type.get_inv(&current_world) else { continue; };

        let new_text = match b_type {
            TypeButton::SlotsUnLocking => button_slots_unlocking(&inventory, &current_world,  &my_parent, &prestige_inv, &settings),
            _ => button_buy_text(count_item, &current_world, &b_type, &my_parent, &prestige_inv, &settings),
        };

        text.0 = new_text;
    }
}
fn button_buy_text(
    count_item: &[usize],
    current_world: &State<CurrentWorld>,
    b_type: &TypeButton,
    b_text: &str,
    prestige_inv: &PrestigeRoom,
    settings: &GlobalSettings,
) -> String {
    let Some(plant) = b_type.get_plant_cfg() else {
        return "".into();
    };

    let Some(prestige_room) = prestige_inv.get_room(current_world.get()) else { return "".into(); };

    let final_count = count_item[plant.species_id as usize];

    if final_count < plant.max_count {
        let final_price = plant.price[final_count] * (1.0 + (prestige_room as f64).powf(1.6) * 3.0);
        
        format!(
            "{}\n😸 {}\n{}\n{} / {}",
            translate(b_text, &settings.language),
            format_number(final_price),
            translate("ui-count", &settings.language),
            final_count,
            plant.max_count
        )
    } else {
        translate("ui-max", &settings.language)
    }
}

fn button_slots_unlocking(
    inventory: &GlobalInventory,
    current_world: &State<CurrentWorld>,
    b_text: &str,
    prestige_inv: &PrestigeRoom,
    settings: &GlobalSettings,
) -> String {
    let price = match current_world.get() {
        CurrentWorld::SunlitNursery => SLOT_PRICES,
        CurrentWorld::WarmPawsPorch => return "".into(),
    };

    let mut new_price = Vec::new();

    let Some(prestige_room) = prestige_inv.get_room(current_world.get()) else { return "".into(); };

    for cost in price.prices.iter() {
        new_price.push(cost + (prestige_room as f64).powf(1.6) * 5500.0);
    };

    if let Some(index_slot) = inventory.get_slots_unlocking(&current_world) {
        format!(
            "{}\n😸 {}\n{}\n{} / 16",
            translate(b_text, &settings.language),
            format_number(new_price[index_slot]),
            translate("ui-count", &settings.language),
            index_slot + 4
        )
    } else {
        translate("ui-max", &settings.language)
    }
}
