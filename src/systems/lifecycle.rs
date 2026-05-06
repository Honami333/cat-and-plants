use bevy::prelude::*;
use crate::schema::{config::*, resources::*, types_and_states::*, world_components::*};
use crate::content::world::sunlit_nursery::*;
use crate::content::world::warm_paws_porch::*;



pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}


// Общий спаун мира
pub fn spawm_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    shaders: Res<ShaderAssets>,
    current_world: Res<State<CurrentWorld>>,
    font: Res<FontAssets>,
) {
    match current_world.get() {
        CurrentWorld::SunlitNursery => {
            bg_spawn(&mut commands, assets.sunlit_nursery.clone(), SN_DATA);
            shader_spawn(&mut commands, &mut meshes, shaders.sn_window_light.clone());
            spawn_slots(&mut commands, &SN_SLOT_CFG, assets.pot_stands.clone());
            spawn_button(&mut commands, &BUT_TOMATO_CFG, assets.button_buy_tomato.clone());
            spawn_resourse_text(&mut commands, &font);
        },
        CurrentWorld::WarmPawsPorch => {
            bg_spawn(&mut commands, assets.warm_paws_porch.clone(), WPP_DATA);
            shader_spawn(&mut commands, &mut meshes, shaders.wpp_window_light.clone());
            spawn_resourse_text(&mut commands, &font);
        }
    };
}


pub fn bg_spawn(
    commands: &mut Commands,
    bg_image: Handle<Image>,
    data: ScaleBackground,

) {
    commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    Sprite::from_image(bg_image),
    data.clone(),
    Room,
    CleanupMarker,
    ));
}


pub fn shader_spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    shaders: Handle<ShaderMaterial>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(shaders),
        Transform::from_xyz(0.0, 0.0, 20.0),
    ShaderMash,
    CleanupMarker,
    ));
}


// Спаун слотов в зависимости от мира
pub fn spawn_slots(
    commands: &mut Commands,
    config: &WorldSettingsSlot,
    slot_image_handle: Handle<Image>,
) {
    for row in 0..config.slot_grid_scale {
    for col in 0..config.slot_grid_scale {
        let pos = config.slot_start_pos
        + (config.step_x * row as f32)
        + (config.step_y * col as f32);

        commands.spawn((
            Transform::from_translation(pos.extend(1.0)),
            Sprite::from_image(slot_image_handle.clone()),
            Slot {
                id: (row * config.slot_grid_scale + col) as usize,
                base_pos: pos,
            },
            CleanupMarker,
        ));
    }}
}


// Спаун кнопок
pub fn spawn_button(
    commands: &mut Commands,
    config: &ButtonCFG,
    button_image_handle: Handle<Image>,
) {
    commands.spawn((
        config.b_type.clone(),
        Sprite::from_image(button_image_handle),
        Transform::from_xyz(config.pos.x, config.pos.y, 5.0),
        Pickable::default(),
        MyButton {
            base_pos: Vec2::new(config.pos.x, config.pos.y),
        },
        CleanupMarker,
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
            CleanupMarker,
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
                CleanupMarker,
            ));
        }
    });
}


pub fn cleanup_system(
    mut commands: Commands,
    clean_query: Query<Entity, With<CleanupMarker>>,
) {
    for entity in clean_query.iter() {
        commands.entity(entity).despawn();
    };
}