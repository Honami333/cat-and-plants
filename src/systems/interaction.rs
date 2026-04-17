use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::schema::{config::*, types_and_states::*, world_components::*};
use crate::world::*;


// Проверка типа кнопки
pub fn button_check(
    trigger: On<Pointer<Click>>,
    button_type_query: Query<&TypeButton>,
    current_world: Res<CurrentWorld>,
    mut inventory: ResMut<GlobalInventory>,
) {        
    if let Ok(button_data) = button_type_query.get(trigger.entity) {
    match button_data {
        TypeButton::TomatoButton => {
            inventory.add_plant(*current_world, SUNLIT_NURSERY_TOMATO);
        },
    }};
}


// Начало перетаскивая предмета
pub fn start_drag_item(
    trigger: On<Pointer<DragStart>>,
    mut dragged: ResMut<DragItem>,
    query_item: Query<(Entity, &SlotItem)>,
    inv: Res<GlobalInventory>,
    current_world: Res<CurrentWorld>,
) {
    let target = trigger.event_target();

    if query_item.get(target).is_ok() {

        if let Ok((_, item)) = query_item.get(target) {
            let inv_world = current_world.get_inv(&inv);

            let Some(slot_item) = inv_world.get(item.slot_id) else { return; };

            if let SlotState::Occupied(plant) = slot_item {
                if plant.state == PlantStateGrowth::Mature(PlantStateUpdate::Idle) { return; };
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
    query_item: Query<&SlotItem>,
    query_slots: Query<(&Transform, &Slot)>,
    bg_query: Query<&ScaleBackground>,
    current_world: Res<CurrentWorld>
) {
    let entity = trigger.entity;

    if let Ok(item) = query_item.get(entity) {
        let mut targer_slot: Option<usize> = None;

        for bg_info in bg_query.iter() {
        for (slot_trans, slot_data) in query_slots.iter() {
        if item.base_pos.distance(slot_trans.translation.truncate() / bg_info.scale_bg) < 35.0 {
            targer_slot = Some(slot_data.id);
            break;
        }}}

        if let Some(new_id) = targer_slot && let Some((_, _)) = query_slots.iter().find(| (_, slot) | slot.id == new_id) {
        if matches!(inventory.sunlit_nursery_inv[new_id], SlotState::Occupied(_) | SlotState::Empty) {
            let old_id = item.slot_id;
            inventory.move_plant(*current_world, old_id, new_id);
        }}
        dragget.entity = None;
    }
}


// Управления стадиями перетаскивания предмета
pub fn state_dragg_item(
    window: Single<&Window, With<PrimaryWindow>>,
    dragged: ResMut<DragItem>,
    bg_query: Query<&ScaleBackground>,
    query_slot: Query<&Slot>,
    mut query_item: Query<(Entity, &mut SlotItem)>
) {
    let Some(mouse_pos) = window.cursor_position() else { return; };
   
    for bg_info in bg_query.iter() {
    for (entity, mut item) in query_item.iter_mut() {
        if Some(entity) == dragged.entity {
        item.base_pos = vec2((mouse_pos.x - window.width() / 2.0) / bg_info.scale_bg,
                                (window.height() / 2.0 - mouse_pos.y) / bg_info.scale_bg)
        } else {
        if let Some(slot_info) = query_slot.iter().find( | slot| slot.id == item.slot_id ) {
            item.base_pos = vec2(slot_info.base_pos.x,
                                slot_info.base_pos.y + 40.0);
        }}
    }}
}


pub fn harvest(
    trigger: On<Pointer<Click>>,
    query_item: Query<(Entity, &SlotItem)>,
    mut inv: ResMut<GlobalInventory>,
    mut resources_inv: ResMut<Economy>,
    current_world: Res<CurrentWorld>,
) {
    let inv_world = current_world.get_inv_mut(&mut inv);

    for (_, slot_item) in query_item.get(trigger.entity).iter_mut() {
        let Some(inv_slot) = inv_world.get_mut(slot_item.slot_id) else { continue; };

        let SlotState::Occupied(plant) = inv_slot else { continue; };

        if plant.state != PlantStateGrowth::Mature(PlantStateUpdate::Idle) { continue; };

        resources_inv.add(plant.species_id as usize + 1, plant.gather_amount);

        plant.state = PlantStateGrowth::Seed(PlantStateUpdate::Idle);
        plant.growth_score = 0;
        
    }
}