use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::prelude::*;
use crate::{WorldCamera, LevelState, BassUI, BassNotes, BassPick, NoteState, NoteCollision, LevelClock, IntroTime, GameState};

#[derive(Resource, Default)]
pub struct CurrentJumpCoord(pub usize);

#[derive(Resource)]
pub struct JumpCoords(pub Vec<GridCoords>);

#[derive(Resource)]
pub struct FirstJumpCoord(pub GridCoords);

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

#[derive(Component)]
pub struct Particle {
    color: Color,
    lifetime: f32,
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
        commands.entity(player).despawn_recursive();
    }
}

pub fn set_player_bounds(
    mut character_query: Query<(&mut Transform), With<Bassist>>
) {
    let mut character_transform = character_query.single_mut();
    character_transform.translation.x = -100.0;
}

/* fn spawn_particles(
    mut commands: Commands,
) {
    
} */

const PLAYER_SPEED: f32 = 250.0;

pub fn player_movement(
    mut character_query: Query<(&mut Transform, &mut Bassist), Without<WorldCamera>>,
    mut camera_query: Query<&mut Transform, (Without<BassNotes>, Without<BassPick>, With<WorldCamera>, Without<Bassist>)>,
    mut string_query: Query<&mut Transform, (Without<BassNotes>, Without<BassPick>, Without<WorldCamera>, Without<Bassist>, With<BassUI>)>,
    mut note_collision_event: EventReader<NoteCollision>,
    input: Res<Input<KeyCode>>,
    level_state: Res<State<LevelState>>,
    mut grid_coord_index: ResMut<CurrentJumpCoord>,
    grid_coords: Res<JumpCoords>,
    note_state: Res<State<NoteState>>,
    mut time: ResMut<LevelClock>,
) {
    time.0.unpause();
    // move camera and player + increment the picks tanslation
    for (mut character_transform, mut bassist) in character_query.iter_mut() {
        // extra query response handles

        let mut camera_transform = camera_query.single_mut();
        let mut string_transform = string_query.single_mut();

        let mut direction = Vec3::ZERO;
        let mut position_x: f32 = PLAYER_SPEED * time.0.delta_seconds();
        let mut position_y: f32 = character_transform.translation.y;

        let jump_grid_xy = ((grid_coords.0[grid_coord_index.0].x as f32 * 16.), (grid_coords.0[grid_coord_index.0].y as f32 * 16.));
        for collision_event in note_collision_event.iter() {
            if character_transform.translation.x >= grid_coords.0[grid_coord_index.0].x as f32 {
                // position_x is for translating the dependents (i.e. camera and bass_ui)
                position_x = (jump_grid_xy.0 - character_transform.translation.x + 8.);
                position_y = (jump_grid_xy.1 + 8.);
                if grid_coord_index.0 < grid_coords.0.len() - 1 {
                    grid_coord_index.0 += 1;
                }
            } 
        }

        character_transform.translation.x += position_x;
        character_transform.translation.y = position_y;
        if level_state.get() == &LevelState::Playing {
            camera_transform.translation.x += position_x;
            string_transform.translation.x += position_x;
        }
/*         println!("{}", time.0.elapsed_seconds()); */
    }
}

pub fn insert_beat_coords(
    mut commands: Commands,
    bassist_query: Query<&Transform, With<Bassist>>,
    jump_query: Query<&GridCoords, With<Jump>>,
) {
    let mut jump_coords = JumpCoords(Vec::new());

    for grid_coord in jump_query.iter() {
        jump_coords.0.push(*grid_coord);
        println!("{:?}", grid_coord);
    }

    // wish this was a separate function but the systems scheduling is acting funny
    // 
    let bassist_transform = bassist_query.single();
    let intro_time: f32 = ((jump_coords.0[0].x as f32 * 16.) - bassist_transform.translation.x) / 250.;
    commands.insert_resource(IntroTime(intro_time));
    commands.insert_resource(jump_coords);
    commands.insert_resource(CurrentJumpCoord(1));
}

pub fn game_state_end(
    mut camera_query: Query<&Transform, With<WorldCamera>>,
    bassist_query: Query<&Transform, With<Bassist>>,
    window_query: Query<&Window>,
    mut change_game_state: ResMut<NextState<GameState>>,
) {
    let camera_transform = camera_query.single();
    let bassist_transform = bassist_query.single();
    let window = window_query.single();

    if bassist_transform.translation.x >= camera_transform.translation.x + window.width() / 2 as f32 {
        change_game_state.set(GameState::Ending);
    }
}
