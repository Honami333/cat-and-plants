use crate::content::world::sunlit_nursery::*;
use crate::content::world::warm_paws_porch::*;
use crate::schema::{config::*, resources::*, types_and_states::*, world_components::*};
use crate::systems::lifecycle::item_spawn;
use bevy::prelude::*;
use bevy::ui::UiScale;
use bevy::window::PrimaryWindow;
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
    let Some(inventory) = current_world.get_inv_mut(&mut inv) else {
        return;
    };

    // Спаун преметов в слотах
    for (idx, slot_state) in inventory.iter_mut().enumerate() {
        if let SlotState::Occupied(plant) = slot_state {
            if let Some(target_slot) = query_slots.iter().find(|s| s.id == idx) {
                let existing_item = query_items
                    .iter_mut()
                    .find(|(_, item)| item.uid == plant.slot_uid);

                if let Some((_, mut item)) = existing_item {
                    if item.slot_id != idx {
                        item.slot_id = idx;
                        plant.slot_uid = idx;
                    }
                } else {
                    item_spawn(
                        &mut commands,
                        &assets,
                        &query_slots,
                        target_slot.id,
                        plant.species_id,
                    );
                }
            }
        }
    }

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
}

// Обновление визула роста ростения
pub fn update_plant_appearance(
    mut query_item: Query<(&mut Sprite, &mut SlotItem)>,
    mut inv: ResMut<GlobalInventory>,
    current_world: Res<State<CurrentWorld>>,
) {
    let Some(inventory) = current_world.get_inv_mut(&mut inv) else {
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
        Query<&mut Transform, With<ShaderMesh>>,
    )>,
    mut materials: ResMut<Assets<ShaderMaterial>>,
    mut ui_scale: ResMut<UiScale>,
    mut worlds: ResMut<WorldScale>,
    window: Single<&Window, With<PrimaryWindow>>,
    shader_query: Query<&MeshMaterial2d<ShaderMaterial>>,
) {
    // Задний фон и общий скейл
    let scale = if let Ok((mut bg_trans, bg_info)) = set.p0().single_mut() {
        let s = (window.width() / bg_info.wh.x).min(window.height() / bg_info.wh.y);
        bg_trans.scale = Vec3::splat(s);
        s
    } else {
        return;
    };

    worlds.scale = scale;

    // Слоты
    for (mut slot_trans, slot_info) in set.p1().iter_mut() {
        slot_trans.translation = (slot_info.base_pos * scale).extend(1.0);
        slot_trans.scale = Vec3::splat(scale);
    }

    // Кнопки
    for (mut button_trans, button_info) in set.p2().iter_mut() {
        button_trans.translation = (button_info.base_pos * scale).extend(5.0);
        button_trans.scale = Vec3::splat(scale);
    }

    // Шейдеры
    for material_handle in shader_query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.scale = material.original_scale / scale;

            for mut transform in set.p3().iter_mut() {
                transform.scale = Vec3::splat(material.mesh_scale * scale);
            }
        }
    }

    ui_scale.0 = scale;
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
) {
    let plant_res = match current_world.get() {
        CurrentWorld::SunlitNursery => SN_PLANT_RES,
        CurrentWorld::WarmPawsPorch => WPP_PLANT_RES,
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

        counter.target_value = economy.get_item(resource_type as usize);

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
    count_item_type: Res<CountItemType>,
) {
    for (parent, mut text, info) in query_button_text.iter_mut() {
        let ButtonText(b_type) = info;

        let my_parent;

        if let Ok(parent_data) = query_button.get(parent.get()) { my_parent = parent_data.text } else { my_parent = ""; };

        let Some(count_item) = count_item_type.get_inv(&current_world) else { continue; };

        let new_text = match b_type {
            TypeButton::SlotsUnLocking => button_slots_unlocking(&inventory, &current_world,  &my_parent),
            _ => button_buy_text(count_item, &b_type, &my_parent),
        };

        text.0 = new_text;
    }
}

fn button_buy_text(
    count_item: &[usize],
    b_type: &TypeButton,
    b_text: &str
) -> String {
    let Some(plant) = b_type.get_plant_cfg() else {
        return "".into();
    };

    let final_count = count_item[plant.species_id as usize];

    if final_count < plant.max_count {
        format!(
            "{}\n😸 {}\ncount:\n{} / {}",
            b_text,
            format_number(
            plant.price[final_count]),
            final_count,
            plant.max_count
        )
    } else {
        "   MAX".to_string()
    }
}

fn button_slots_unlocking(
    inventory: &GlobalInventory,
    current_world: &State<CurrentWorld>,
    b_text: &str
) -> String {
    {
        if let Some(index_slot) = inventory.get_slots_unlocking(&current_world) {
            format!(
                "{}\n😸 {}\ncount:\n{} / 16",
                b_text,
                format_number(SLOT_PRICES.prices[index_slot]),
                index_slot + 4
            )
        } else {
            "   MAX".to_string()
        }
    }
}
