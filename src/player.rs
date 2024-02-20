use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy::sprite::Anchor;
use crate::{WorldCamera, LevelState, BassUI};

// States to control the direction the player moves when jumping
#[derive(Default)]
pub enum Inverse {
    #[default]
    Up,
    Down
}

// States for player movement actions
#[derive(Default)]
pub enum MoveState {
    #[default]
    PlatformMove,
    JumpMove
}

// PLayer struct
#[derive(Component, Default)]
pub struct Bassist {
    movement: MoveState,
    inverse: Inverse
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 32, rows = 32, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "textures/bass_master_sprite.png")]
    sprite: Handle<TextureAtlas>,
}

pub fn spawn_character(
        mut commands: Commands,
        character_assets: Res<CharacterAssets>,
) {
    // Spawn Bassist sprite at the center of the screen with a high Z-index
    commands.spawn((SpriteSheetBundle {
        texture_atlas: character_assets.sprite.clone(),
        // set out of bounds for level loading
        transform: Transform::from_xyz(-1000.0, 352.0 / 2.0, 100.0),
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::splat(16.0)),
            anchor: Anchor::Center,
            ..default()
        },
        ..default()
    },
    Bassist::default()
    ));
}

pub fn despawn_character(
    mut commands: Commands,
    player_query: Query<Entity, With<Bassist>> 
) {
    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn();
    }
}

pub fn set_player_bounds(
    mut character_query: Query<(&mut Transform), With<Bassist>>
) {
    let mut character_transform = character_query.single_mut();
    character_transform.translation.x = -100.0;
}

const PLAYER_SPEED: f32 = 250.0;

pub fn player_movement(
        mut character_query: Query<(&mut Transform, &mut Bassist), Without<WorldCamera>>,
        mut camera_query: Query<&mut Transform, (With<WorldCamera>, Without<Bassist>)>,
        mut string_query: Query<&mut Transform, (Without<WorldCamera>, Without<Bassist>, With<BassUI>)>,
        input: Res<Input<KeyCode>>,
        level_state: Res<State<LevelState>>,
        mut time: ResMut<Time>,
) {
    // start game timer if it's not already unpaused
    if time.is_paused() {
        time.unpause();
    }

    // move camera and player + increment the picks tanslation
    for (mut character_transform, mut bassist) in character_query.iter_mut() {
        // extra query response handles
        let mut string_transform = string_query.single_mut();

        let mut direction = Vec3::ZERO;
        // logic for character translations
        match &bassist.movement {
            MoveState::PlatformMove => direction += Vec3::new(1.0, 0.0, 0.0),
            MoveState::JumpMove => direction += Vec3::new(PLAYER_SPEED, 20.0, 0.0),
            _ => panic!("!! Enum MoveState not set for Bassist in player_move query !!")
        } 
        // translate character and camera
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let position_x: f32 = PLAYER_SPEED * time.delta_seconds();
            character_transform.translation.x += position_x;
            // translate anything other than the player (i.e. camera, pick)
            if level_state.get() == &LevelState::Playing {
                camera_transform.translation.x += position_x;
                string_transform.translation.x += position_x;
            }

            println!("{}", time.elapsed_seconds());
        }
    }
}
