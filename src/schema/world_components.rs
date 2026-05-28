use bevy::prelude::*;

// Общие компоненты миров
#[derive(Component)]
pub struct ShaderMesh; // Шейдер

#[derive(Component)]
pub struct Room; // Мир

#[derive(Component)]
pub struct Slot { // Слот инвенторя
    pub id: usize,
    pub base_pos: Vec2,
}

#[derive(Component, Debug)]
pub struct SlotItem { // Предмет
    pub uid: usize,
    pub base_pos: Vec2,
    pub slot_id: usize,
}

#[derive(Component)]
pub struct CleanupMarker;
