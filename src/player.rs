use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy::sprite::Anchor;
use crate::{WorldCamera, LevelState, MovementTimer};

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
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 8, rows = 1, padding_x = 0., padding_y = 0., offset_x = 6., offset_y = 6.))]
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
        transform: Transform::from_xyz(-1000.0, 256.0 / 2.0, 100.0),
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::splat(60.0)),
            anchor: Anchor::Center,
            color: Color::PURPLE,
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
    character_transform.translation.x = 0.0;
}

const PLAYER_SPEED: f32 = 1.0;

pub fn player_movement(
        mut character_query: Query<(&mut Transform, &mut Bassist), Without<WorldCamera>>,
        mut camera_query: Query<&mut Transform, (With<WorldCamera>, Without<Bassist>)>,
        input: Res<Input<KeyCode>>,
        level_state: Res<State<LevelState>>,
        time: Res<Time>,
        mut timer: ResMut<MovementTimer>
) {
    for (mut character_transform, mut bassist) in character_query.iter_mut() {
        
        // translate character and camera
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            if timer.0.just_finished() {
                character_transform.translation.x += 10.0;
            }
            timer.0.tick(time.delta());
        }
        // match &bassist.movement {
        //     MoveState::PlatformMove => transform.translation += Vec3::new(1.0, 0.0, 0.0),
        //     MoveState::JumpMove => transform.translation += Vec3::new(PLAYER_SPEED, 20.0, 0.0),
        //     _ => panic!("!! Enum MoveState not set for Bassist in player_move query !!")
        // } 
    }
}
