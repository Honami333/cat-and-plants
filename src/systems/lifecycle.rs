use bevy::prelude::*;
use crate::schema::{resources::*, config::*, types_and_states::*, world_components::*};


// Общий спаун мира
pub fn spawm_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    shaders: Res<ShaderAssets>,
    current_world: Res<CurrentWorld>,
    font: Res<FontAssets>,
) {
    commands.spawn(Camera2d::default());

    // Получения данных о мире
    let (data, slot_cfg, button_cfg, bg_image, pot_image, shaders) = current_world.get_config(&assets, &shaders);

    // Спаун всего мира
    commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    Sprite::from_image(bg_image),
    data.clone(),
    Room,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(shaders),
        Transform::from_xyz(0.0, 0.0, 20.0),
    ShaderMash,
    ));

    spawn_slots(&mut commands, &current_world, &slot_cfg, pot_image);
    spawn_button(&mut commands, &assets, &button_cfg);
    spawn_resourse_text(&mut commands, &font);
}


// Спаун слотов в зависимости от мира
pub fn spawn_slots(
    commands: &mut Commands,
    current_world: &Res<CurrentWorld>,
    config: &WorldSettingsSlot,
    pot_image: Handle<Image>,
) {
    for row in 0..config.slot_grid_scale {
    for col in 0..config.slot_grid_scale {
        let pos = config.slot_start_pos
        + (config.step_x * row as f32)
        + (config.step_y * col as f32);

        commands.spawn((
            Transform::from_translation(pos.extend(1.0)),
            Sprite::from_image(pot_image.clone()),
            Slot {
                id: (row * config.slot_grid_scale + col) as usize,
                base_pos: pos,
            },
        ));
    }}
}


// Спаун кнопок
pub fn spawn_button(
    commands: &mut Commands,
    assets: &GameAssets,
    config: &ButtonCFG,
) {
    let image_handle = match config.b_type {
        TypeButton::TomatoButton => assets.button_buy_tomato.clone(),
    };

    commands.spawn((
        config.b_type.clone(),
        Sprite::from_image(image_handle),
        Transform::from_xyz(config.pos.x, config.pos.y, 5.0),
        Pickable::default(),
        MyButton {
            base_pos: Vec2::new(config.pos.x, config.pos.y),
        },
    ));
}


// Спаун предметов
pub fn item_spawn(
    commands: &mut Commands,
    assets: &AtlasAssets,
    query_slots: &Query<&Slot>,
    target_id: usize,
    plant_type: TypePlant,
) {
    let (image_handle, layout_type) = match plant_type {
        TypePlant::Tomato => (assets.tomato_pot_atlas.clone(), assets.common_layot.clone()),

    };

    if let Some(slot) = query_slots.iter().find(| slot | slot.id == target_id) {
        commands.spawn((
            Sprite::from_atlas_image(
                image_handle,
                TextureAtlas { layout: layout_type, index: 0 }
            ),
            Transform::from_xyz(10000.0, 10000.0, 2.5),
            Pickable::default(),
            SlotItem {
                uid: target_id,
                base_pos: Vec2::new(10000.0, 10000.0),
                slot_id: slot.id,
            },
        ));
    }
}


pub fn spawn_resourse_text(
    commands: &mut Commands,
    font: &FontAssets,
) {
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(10.0),
            width: Val::Vw(20.0),
            height: Val::Vh(10.0),
            ..default()
        },
    )).with_children(|parent| {
        for i in 0..5 {
            parent.spawn((
                Text::new(" "),
                TextFont {
                    font: font.emoji_font.clone(),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new(Justify::Left, LineBreak::NoWrap),
                MyText(i),
                VisualCounter {
                    display_value: 0.0,
                    target_value: 0.0,
                },
            ));
        }
    });
}