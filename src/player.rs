use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::prelude::*;
use crate::{WorldCamera, LevelState, BassUI, BassNotes, BassPick, NoteState};

#[derive(Resource, Default)]
pub struct CurrentJumpCoord(pub usize);

#[derive(Resource)]
pub struct JumpCoords(pub Vec<GridCoords>);

#[derive(Component, Default)]
pub struct Jump;

#[derive(Bundle, LdtkEntity, Default)]
pub struct JumpBundle {
    jump: Jump,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

// States for player movement actions
#[derive(Default)]
pub enum MoveState {
    PlatformMove,
    #[default]
    JumpMove
}

// PLayer struct
#[derive(Component, Default)]
pub struct Bassist {
    movement: MoveState,
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
        transform: Transform::from_xyz(-1000.0, 336.0 / 2.0, 100.0),
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
    mut camera_query: Query<&mut Transform, (Without<BassNotes>, Without<BassPick>, With<WorldCamera>, Without<Bassist>)>,
    mut string_query: Query<&mut Transform, (Without<BassNotes>, Without<BassPick>, Without<WorldCamera>, Without<Bassist>, With<BassUI>)>,
    input: Res<Input<KeyCode>>,
    level_state: Res<State<LevelState>>,
    mut grid_coord_index: ResMut<CurrentJumpCoord>,
    grid_coords: Res<JumpCoords>,
    note_state: Res<State<NoteState>>,
    mut time: ResMut<Time>,
) {
    // start game timer if it's not already unpaused
    if time.is_paused() {
        time.unpause();
    }

    // move camera and player + increment the picks tanslation
    for (mut character_transform, mut bassist) in character_query.iter_mut() {
        // extra query response handles
        let mut camera_transform = camera_query.single_mut();
        let mut string_transform = string_query.single_mut();

        let mut direction = Vec3::ZERO;
        // logic for character translations
        match note_state.get() {
            &NoteState::Rest => {
                // translate during rests
                let position_x: f32 = PLAYER_SPEED * time.delta_seconds();
                character_transform.translation.x += position_x;

                // translate anything other than the player (i.e. camera, pick)
                if level_state.get() == &LevelState::Playing {
                    camera_transform.translation.x += position_x;
                    string_transform.translation.x += position_x;
                }
            },
            &NoteState::Playing => {
                // translate to jump spots
                let mut position_x: f32 = 0.;
                if character_transform.translation.x >= grid_coords.0[grid_coord_index.0].x as f32 {
                    // position_x is for translating the dependents (i.e. camera and bass_ui)
                    position_x = (grid_coords.0[grid_coord_index.0].x as f32 * 16.) - character_transform.translation.x + 8.;
                    character_transform.translation.x += position_x;
                    character_transform.translation.y = grid_coords.0[grid_coord_index.0].y as f32 * 16. + 8.;

                    grid_coord_index.0 += 1;
                } 

                if level_state.get() == &LevelState::Playing {
                    camera_transform.translation.x += position_x;
                    string_transform.translation.x += position_x;
                }
            },
            _ => panic!("!! Enum MoveState not set for Bassist in player_move query !!")
        } 


        println!("{}", time.elapsed_seconds());
    }
}

pub fn insert_beat_coords(
    mut commands: Commands,
    jump_query: Query<&GridCoords, With<Jump>>,
) {
    let mut jump_coords = JumpCoords(Vec::new());

    for grid_coord in jump_query.iter() {
        jump_coords.0.push(*grid_coord);
        println!("{:?}", grid_coord);
    }

    commands.insert_resource(jump_coords);
    commands.insert_resource(CurrentJumpCoord(1));
}

/* pub fn bassist_state_handle(

) */
