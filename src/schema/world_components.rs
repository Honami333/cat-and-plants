use bevy::prelude::*;

use crate::schema::types_and_states::TypeButton;

// Общие компоненты миров
#[derive(Component)]
pub struct ShaderMesh; // Шейдер

#[derive(Component)]
pub struct Room; // Мир

#[derive(Component)]
pub struct MyButton {
    // Кнопка
    pub base_pos: Vec2,
}

#[derive(Component)]
pub struct Slot {
    // Слот инвенторя
    pub id: usize,
    pub base_pos: Vec2,
}

#[derive(Component, Debug)]
pub struct SlotItem {
    // Предмет
    pub uid: usize,
    pub base_pos: Vec2,
    pub slot_id: usize,
}

#[derive(Component)]
pub struct EconomyText(pub usize);

#[derive(Component)]
pub struct ButtonText(pub TypeButton);

#[derive(Component)]
pub struct VisualCounter {
    pub display_value: f64,
    pub target_value: f64,
}

#[derive(Component)]
pub struct CleanupMarker;
