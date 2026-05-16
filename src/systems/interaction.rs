use crate::content::world::sunlit_nursery::*;
use crate::schema::{types_and_states::*, world_components::*, save_file::*};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Проверка типа кнопки
pub fn button_check(
    trigger: On<Pointer<Click>>,
    mut inventory: ResMut<GlobalInventory>,
    mut economy: ResMut<Economy>,
    mut count_item_type: ResMut<CountItemType>,
    upgrade_storege: Res<UpgradeStorege>,
    button_type_query: Query<&TypeButton>,
    current_world: Res<State<CurrentWorld>>,
) {
    if let Ok(button_data) = button_type_query.get(trigger.entity) {
        match button_data {
            TypeButton::SlotsUnLocking => {
                try_slots_unlocking(&mut inventory, &mut economy, &current_world)
            }
            _ => add_plant_and_lock(
                &mut inventory,
                &mut economy,
                &mut count_item_type,
                &upgrade_storege,
                &current_world,
                &button_data,
            ),
        }
    };
}

fn try_slots_unlocking(
    inventory: &mut GlobalInventory,
    economy: &mut Economy,
    current_world: &State<CurrentWorld>,
) {
    let price = match current_world.get() {
        CurrentWorld::SunlitNursery => SLOT_PRICES,
        CurrentWorld::WarmPawsPorch => return,
    };

    let (try_lock, Some(i)) = inventory.slots_unlocking(&economy, &current_world, &price.prices)
    else {
        return;
    };

    if try_lock {
        economy.add(ResourceType::CatHappiness as usize, -price.prices[i]);
    }
}

fn add_plant_and_lock(
    inventory: &mut GlobalInventory,
    economy: &mut Economy,
    count_item_type: &mut CountItemType,
    upgrade_storege: &UpgradeStorege,
    current_world: &State<CurrentWorld>,
    button_data: &TypeButton,
) {
    if inventory.get_slots_empty(&current_world) == false {
        return;
    };

    let availability = if let Some(upgrade_id) = button_data.get_dependencies_upgrade() {
        let (_, available) = upgrade_storege.get_global_modifier(upgrade_id);
        available
    } else {
        true
    };

    if !availability {
        return;
    };

    let Some(count_inv) = count_item_type.get_inv_mut(&current_world) else {
        return;
    };

    let Some(plant) = button_data.get_plant_cfg() else {
        return;
    };

    let plant_count = count_inv[plant.species_id as usize];

    if plant_count >= plant.max_count {
        return;
    };

    let cur_price = plant.price[plant_count];

    if economy.get_item(ResourceType::CatHappiness as usize) < cur_price {
        return;
    };

    if count_inv[plant.species_id as usize] >= plant.max_count {
        return;
    };

    economy.add(ResourceType::CatHappiness as usize, -cur_price);
    count_item_type.add(plant.species_id as usize, &current_world);

    inventory.add_plant(current_world, plant);
}

// Начало перетаскивая предмета
pub fn start_drag_item(
    trigger: On<Pointer<DragStart>>,
    mut dragged: ResMut<DragItem>,
    query_item: Query<(Entity, &SlotItem)>,
    inv: Res<GlobalInventory>,
    current_world: Res<State<CurrentWorld>>,
) {
    let target = trigger.event_target();

    if query_item.get(target).is_ok() {
        if let Ok((_, item)) = query_item.get(target) {
            let Some(inv_world) = inv.get_inv(&current_world) else {
                return;
            };

            let Some(slot_item) = inv_world.get(item.slot_id) else {
                return;
            };

            if let SlotState::Occupied(plant) = slot_item {
                if plant.state == PlantStateGrowth::Mature {
                    return;
                };
            }
        }

        dragged.entity = Some(target);
    }
}

// Окончания перетаскивая предмета
pub fn end_drag_item(
    trigger: On<Pointer<DragDrop>>,
    mut inventory: ResMut<GlobalInventory>,
    mut dragget: ResMut<DragItem>,
    world_scale: Res<WorldScale>,
    query_item: Query<&SlotItem>,
    query_slots: Query<(&Transform, &Slot)>,
    current_world: Res<State<CurrentWorld>>,
    worlds: Res<WorldScale>,
) {
    let entity = trigger.entity;

    if let Ok(item) = query_item.get(entity) {
        let mut targer_slot: Option<usize> = None;

        for (slot_trans, slot_data) in query_slots.iter() {
            if item
                .base_pos
                .distance(slot_trans.translation.truncate() / worlds.scale)
                < 17.5 * world_scale.scale
            {
                targer_slot = Some(slot_data.id);
                break;
            }
        }

        if let Some(new_id) = targer_slot
            && let Some((_, _)) = query_slots.iter().find(|(_, slot)| slot.id == new_id)
        {
            if matches!(
                inventory.sunlit_nursery_inv[new_id],
                SlotState::Occupied(_) | SlotState::Empty
            ) {
                let old_id = item.slot_id;
                inventory.move_plant(&current_world, old_id, new_id);
            }
        }
        dragget.entity = None;
    }
}

// Управления стадиями перетаскивания предмета
pub fn state_dragg_item(
    mut query_item: Query<(Entity, &mut SlotItem)>,
    window: Single<&Window, With<PrimaryWindow>>,
    dragged: ResMut<DragItem>,
    query_slot: Query<&Slot>,
    worlds: Res<WorldScale>,
) {
    let Some(mouse_pos) = window.cursor_position() else {
        return;
    };

    for (entity, mut item) in query_item.iter_mut() {
        if Some(entity) == dragged.entity {
            item.base_pos = vec2(
                (mouse_pos.x - window.width() / 2.0) / worlds.scale,
                (window.height() / 2.0 - mouse_pos.y) / worlds.scale,
            )
        } else {
            if let Some(slot_info) = query_slot.iter().find(|slot| slot.id == item.slot_id) {
                item.base_pos = vec2(slot_info.base_pos.x, slot_info.base_pos.y + 40.0);
            }
        }
    }
}

pub fn harvest(
    trigger: On<Pointer<Click>>,
    query_item: Query<(Entity, &SlotItem)>,
    mut inv: ResMut<GlobalInventory>,
    mut resources_inv: ResMut<Economy>,
    current_world: Res<State<CurrentWorld>>,
    upgrade_storege: Res<UpgradeStorege>,
) {
    let mut up_value =  1.0;
    
    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::FertileSoil) {up_value = value};

    let Some(inv_world) = inv.get_inv_mut(&current_world) else {return; };

    for (_, slot_item) in query_item.get(trigger.entity).iter_mut() {
        let Some(inv_slot) = inv_world.get_mut(slot_item.slot_id) else { continue; };

        let SlotState::Occupied(plant) = inv_slot else { continue; };

        if plant.state != PlantStateGrowth::Mature { continue; };

        resources_inv.add(plant.species_id as usize + 1, plant.gather_amount * up_value);

        plant.state = PlantStateGrowth::Seed;
        plant.growth_score = 0.0;
    }
}
