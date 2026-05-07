use crate::content::world::sunlit_nursery::*;
use crate::content::world::warm_paws_porch::*;
use crate::schema::{config::*, resources::*, types_and_states::*, world_components::*};
use bevy::prelude::*;
use bevy::sprite::Anchor;


pub fn add_start(mut economy: ResMut<Economy>) {
    economy.add(ResourceType::CatHappiness as usize, 100000.0);
    economy.add(ResourceType::Tomatoes as usize, 0.0);
}


pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

// Общий спаун мира
pub fn spawn_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    shaders: Res<ShaderAssets>,
    current_world: Res<State<CurrentWorld>>,
    font: Res<FontAssets>,
) {
    match current_world.get() {
        CurrentWorld::SunlitNursery => {
            bg_spawn(&mut commands, assets.sunlit_nursery.clone(), SN_DATA, &font);
            shader_spawn(&mut commands, &mut meshes, shaders.sn_window_light.clone());
            spawn_slots(&mut commands, &SN_SLOT_CFG, assets.pot_stands.clone());
            spawn_button(
                &mut commands,
                &BUT_TOMATO_CFG,
                assets.button_buy_tomato.clone(),
                &font,
            );
            spawn_button(
                &mut commands,
                &BUT_SLOTSUNLOKING_CFG,
                assets.button_buy_tomato.clone(),
                &font,
            );
        }
        CurrentWorld::WarmPawsPorch => {
            bg_spawn(&mut commands, assets.warm_paws_porch.clone(), WPP_DATA, &font);
            shader_spawn(&mut commands, &mut meshes, shaders.wpp_window_light.clone());
        }
    };
}

pub fn bg_spawn(commands: &mut Commands, bg_image: Handle<Image>, bg_data: ScaleBackground, font: &FontAssets) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(bg_image),
        bg_data.clone(),
        Room,
        CleanupMarker,
    )).with_children(|parent| {
        spawn_resourse_text(parent, &font, bg_data);
    });
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
        ShaderMesh,
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
            let pos =
                config.slot_start_pos + (config.step_x * row as f32) + (config.step_y * col as f32);

            commands.spawn((
                Transform::from_translation(pos.extend(1.0)),
                Sprite::from_image(slot_image_handle.clone()),
                Slot {
                    id: (row * config.slot_grid_scale + col) as usize,
                    base_pos: pos,
                },
                CleanupMarker,
            ));
        }
    }
}

// Спаун кнопок
pub fn spawn_button(
    commands: &mut Commands,
    config: &ButtonCFG,
    button_image_handle: Handle<Image>,
    font: &FontAssets,
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
    )).with_children(|parent| {
        parent.spawn((
            Text2d::new(" "),
            TextColor(Color::WHITE),
            TextFont {
                font: font.emoji_font.clone(),
                font_size: 8.0,
                ..default()
            },
            Anchor::CENTER_LEFT,
            Transform::from_xyz(65.0, 0.0, 0.1),
            TextLayout::new(Justify::Left, LineBreak::NoWrap),
            ButtonText(config.b_type),
            CleanupMarker,
        ));
    });
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
        TypePlant::Tomato => (
            assets.tomato_pot_atlas.clone(),
            assets.common_layout.clone(),
        ),
    };

    if let Some(slot) = query_slots.iter().find(|slot| slot.id == target_id) {
        commands.spawn((
            Sprite::from_atlas_image(
                image_handle,
                TextureAtlas {
                    layout: layout_type,
                    index: 0,
                },
            ),
            Transform::from_xyz(10000.0, 10000.0, 2.5),
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
            },
            SlotItem {
                uid: target_id,
                base_pos: Vec2::new(10000.0, 10000.0),
                slot_id: slot.id,
            },
            CleanupMarker,
        ));
    }
}

pub fn spawn_resourse_text(main_parent: &mut ChildSpawnerCommands, font: &FontAssets, bg_data: ScaleBackground) {
    main_parent
        .spawn((
        Transform::from_xyz(-bg_data.wh.x / 2.0, bg_data.wh.y / 2.0 - 20.0, 0.1),
        Visibility::default(),
        InheritedVisibility::default(),
        CleanupMarker,
    ))
        .with_children(|parent| {
            for i in 0..5 {
                parent.spawn((
                    Text2d::new(" "),
                    TextFont {
                        font: font.emoji_font.clone(),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, -(i as f32 * 30.0), 0.0),
                    Anchor::CENTER_LEFT,
                    TextLayout::new(Justify::Right, LineBreak::NoWrap),
                    EconomyText(i),
                    VisualCounter {
                        display_value: 0.0,
                        target_value: 0.0,
                    },
                    CleanupMarker,
                ));
            }
        });
}

pub fn cleanup_system(mut commands: Commands, clean_query: Query<Entity, With<CleanupMarker>>) {
    for entity in clean_query.iter() {
        commands.entity(entity).despawn();
    }
}
