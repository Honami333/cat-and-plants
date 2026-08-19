use crate::schema::{world_components::*, common::*, global_inventory::*, economy_inventory::*, prestige::*, upgrade_storege::*};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const RADIUS: f32 = 16.0;


// Начало перетаскивая предмета
pub fn start_drag_item(
    _trigger: On<Pointer<DragStart>>,
    mut dragged: ResMut<DragItem>,
    query_item: Query<(Entity, &SlotItem)>,
    window: Single<&Window, With<PrimaryWindow>>,
    scale: Res<WorldScale>,
) {
    if dragged.mouse_stage != MouseStage::Dragg { return; };

    let s = scale.0;

    let Some(mouse_pos) = window.cursor_position() else { return; };

    let pos = vec2(
        (mouse_pos.x - window.width() / 2.0) / s.x,
        (window.height() / 2.0 - mouse_pos.y) / s.y,
    );

    let Some((entity, _)) = query_item.iter()
        .find(|(_, item)|
            pos.distance(item.base_pos) < RADIUS * s.x.min(s.y)
    ) else { return; };

    dragged.entity = Some(entity);   
}

// Окончания перетаскивая предмета
pub fn end_drag_item(
    _trigger: On<Pointer<DragDrop>>,
    mut gl_inventory: ResMut<GlobalInventory>,
    mut dragget: ResMut<DragItem>,
    query_item: Query<&SlotItem>,
    query_slots: Query<(&Transform, &Slot)>,
    current_world: Res<State<CurrentWorld>>,
    window: Single<&Window, With<PrimaryWindow>>,
    scale: Res<WorldScale>,
) {
    let s = scale.0;

    let Some(mouse_pos) = window.cursor_position() else { return; };

    let pos = vec2(
        (mouse_pos.x - window.width() / 2.0) / s.x,
        (window.height() / 2.0 - mouse_pos.y) / s.y,
    );

    let item = query_item.iter()
        .find(|item|
            pos.distance(item.base_pos) < RADIUS * s.x.min(s.y)
    );

    if let Some(item) = item {
        let mut targer_slot: Option<usize> = None;

        for (slot_trans, slot_data) in query_slots.iter() {
            if item
                .base_pos
                .distance(slot_trans.translation.truncate() / (s.x).min(s.y))
                < 24.0 * (s.x).min(s.y)
            {
                targer_slot = Some(slot_data.id);
                break;
            }
        }

        if let Some(new_id) = targer_slot
            && let Some((_, _)) = query_slots.iter().find(|(_, slot)| slot.id == new_id) {

            let Some(inventory) = gl_inventory.get_for_world(&current_world) else { return; };

            let Some(slot_state) = inventory.get(&new_id) else { return; };

            if matches!(
                slot_state,
                SlotState::Occupied(_) | SlotState::Empty
            ) {
                let old_id = item.slot_id;
                gl_inventory.move_plant(&current_world, old_id, new_id);
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
    scale: Res<WorldScale>,
) {
    let s = scale.0;

    let Some(mouse_pos) = window.cursor_position() else { return; };

    for (entity, mut item) in query_item.iter_mut() {
        if Some(entity) == dragged.entity {
            item.base_pos = vec2(
                (mouse_pos.x - window.width() / 2.0) / s.x,
                (window.height() / 2.0 - mouse_pos.y) / s.y,
            )
        } else {
            if let Some(slot_info) = query_slot.iter().find(|slot| slot.id == item.slot_id) {
                item.base_pos = vec2(slot_info.base_pos.x, slot_info.base_pos.y + 40.0);
            }
        }
    }
}

pub fn harvest(
    _trigger: On<Pointer<Click>>,
    query_item: Query<&SlotItem>,
    mut global_inventory: ResMut<GlobalInventory>,
    mut resources_inv: ResMut<Economy>,
    dragged: Res<DragItem>,
    current_world: Res<State<CurrentWorld>>,
    upgrade_storege: Res<UpgradeStorege>,
    prestige_inv: Res<PrestigeRoom>,
    window: Single<&Window, With<PrimaryWindow>>,
    scale: Res<WorldScale>,
) {
    if dragged.mouse_stage != MouseStage::Click { return; };

    let s = scale.0;

    let Some(mouse_pos) = window.cursor_position() else { return; };

    let pos = vec2(
        (mouse_pos.x - window.width() / 2.0) / s.x,
        (window.height() / 2.0 - mouse_pos.y) / s.y,
    );

    let item = query_item.iter()
        .find(|item| pos.distance(item.base_pos) < RADIUS * s.x.min(s.y)
    );

    let mut up_value_1 =  0.0;

    let mut up_value_2 =  1.0;

    let mut up_value_3 =  0.0;

    let mut modifier_unlocked_1 = false;

    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::OverBlooming) {up_value_1 = value};
    
    if let (Some(value), _) = upgrade_storege.get_global_modifier(UpgradeUID::FertileSoil) {up_value_2 = value};

    if let (Some(value), is_unlocked) = upgrade_storege.get_global_modifier(UpgradeUID::ConcentratedNectar) {
        up_value_3 = value;
        modifier_unlocked_1 = is_unlocked;
    };

    let tomato_combo = if let Some(data) = global_inventory.find_ability_global(PlantAbilityType::TomatoClickCombo, &current_world) 
        && let PlantAbilityData::TomatoClickCombo { combo } = data { Some(combo) } else { None };

    let Some(global_inv) = global_inventory.get_for_world_mut(&current_world) else {return; };

    let prestige_buff = 1.0 + ((prestige_inv.get_all_prestige() as f64).powf(1.25) * up_value_1);

    if let Some(slot_item) = item {
        let Some(inv_slot) = global_inv.get_mut(&slot_item.slot_id) else { return; };

        let SlotState::Occupied(plant) = inv_slot else { return; };

        if plant.state != PlantStateGrowth::Mature { return; };

        let mut plant_bounty =  0.0;

        let mut modifier_unlocked_2 = false;

        if let (Some(value), is_unlocked ) = upgrade_storege.get_plant_global_modifier(&plant.species_id, PlantGGM::Bounty) {
            plant_bounty = value;
            modifier_unlocked_2 = is_unlocked;
        };

        let mut amount = if modifier_unlocked_2 {
            match plant.species_id {
                TypePlant::Pumpkin => ((plant.gather_amount + plant_bounty) * up_value_2 * prestige_buff).floor(),
                _ => (plant.gather_amount * up_value_2 * prestige_buff * plant_bounty).floor()
            }
        } else {
            (plant.gather_amount * up_value_2 * prestige_buff).floor()
        };

        let species_id = plant.species_id;
        let tomato_bonus = if species_id == TypePlant::Tomato { 2.0 } else { 1.0 };
        
        if modifier_unlocked_1 && let Some(combo) = tomato_combo {
            amount += (up_value_3 * tomato_bonus * amount * combo as f64).floor();
        }

        resources_inv.add_res(plant.species_id.into(), amount);

        plant.state = PlantStateGrowth::Seed;
        plant.growth_score = 0.0;

        let mut tomato_vec = Vec::new();

        if species_id == TypePlant::Tomato && modifier_unlocked_1 {
            global_inventory.find_ability_global_mut_all(PlantAbilityType::TomatoClickCombo, &current_world, &mut tomato_vec);

            for tomato_data in tomato_vec.iter_mut() {
                if let PlantAbilityData::TomatoClickCombo { combo } = tomato_data {
                    *combo += 1_usize;
                };
            };
            GlobalInventory::ability_global_to_max(&mut tomato_vec);
        };
    }
}
