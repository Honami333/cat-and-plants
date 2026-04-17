use bevy::prelude::*;


// Общие компоненты миров
#[derive(Component)]
pub struct ShaderMash; // Шейдер

#[derive(Component)]
pub struct Room; // Мир

#[derive(Component)]
pub struct MyButton { // Кнопка
    pub base_pos: Vec2,
}

#[derive(Component)]
pub struct Slot { // Слот инвенторя
    pub id: usize,
    pub base_pos: Vec2,
}

#[derive(Component)]
pub struct SlotItem { // Предмет
    pub uid: usize,
    pub base_pos: Vec2,
    pub slot_id: usize,
}

#[derive(Component)]
pub struct MyText(pub usize);

#[derive(Component)]
pub struct VisualCounter {
    pub display_value: f64,
    pub target_value: f64,
}
