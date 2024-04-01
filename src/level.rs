use bevy::prelude::*;
use bevy::utils::Instant;

use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::path::{Path, PathBuf};
use crate::{WorldCamera, Bassist, GameState, WorldEvent, IntroTimer, WindowScaleFactor};

// Marker Component
#[derive(Component)]
pub struct CurrentLevel();

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    #[default]
    OutOfLevel,
    Introduction,
    Playing,
    Paused,
    Ending
}

#[derive(Resource)]
pub struct LevelResource(pub PathBuf);

#[derive(Debug, Resource)]
pub struct LevelClock(pub Time);

pub fn insert_world_dir(
    mut commands: Commands,
    mut level_dir_events: EventReader<WorldEvent>,
    mut change_game_state: ResMut<NextState<GameState>>,
) {
    for level_dir_event in level_dir_events.iter() {
        let path = Path::new(&level_dir_event.0);
        // coerce to PathBuf
        let path_buf = path.to_path_buf();
        commands.insert_resource(LevelResource(path_buf));
        // switch state
        change_game_state.set(GameState::AssetLoading);
    }
}

pub fn load_world(
    mut commands: Commands,
    level_path: Res<LevelResource>,
    asset_server: Res<AssetServer>,

) {
    // set the path extension and coerce to str
    // resource heavy clone as we don't want to be mutating the resource (note. I mean technically
    // we could as .set_extension() does, in fact, replace the extension type)
    let mut path = level_path.0.clone();
    path.set_extension("ldtk");
    let path_str = path.to_str().unwrap();

    commands.spawn((LdtkWorldBundle {
        ldtk_handle: asset_server.load(path_str),
        ..default()
    },
        CurrentLevel {}
    ));
    commands.insert_resource(LevelClock(Time::new(Instant::now())));
}

pub fn manage_level_states(
    camera_query: Query<&Transform, With<WorldCamera>>,
    character_query: Query<&Transform, (With<Bassist>, Without<WorldCamera>)>,
    level_query: Query<&Handle<LdtkLevel>, (Without<Bassist>, Without<WorldCamera>)>,
    window_query: Query<&Window>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    mut change_level_state: ResMut<NextState<LevelState>>,
    level_state: Res<State<LevelState>>,
    scale_factor: Res<WindowScaleFactor>,
) {
    for level_handle in level_query.iter(){
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let window = window_query.single();
            let level = &ldtk_level.level;
            let camera_transform = camera_query.single();
            let character_transform = character_query.single();

            // If world will be out of bounds in either direction before and after the bassist
            // is off or on screen we set the x translation accordingly 
            match level_state.get() {
                // if character reaches start of level
                // TODO: set timer after start for when the song should start
                &LevelState::Introduction if character_transform.translation.x >= (camera_transform.translation.x) => 
                    change_level_state.set(LevelState::Playing),
                // if camera is out of bounds of leve
                &LevelState::Playing if (camera_transform.translation.x + (window.width() / 2.0) * scale_factor.0) >= level.px_wid as f32 =>  
                    change_level_state.set(LevelState::Ending),
                // if nuthin
                &_ => (),
            }
        }    
    }
}

pub fn despawn_world(
    mut commands: Commands,
    level_query: Query<Entity, With<Handle<LdtkAsset>>>,
) {
    level_query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

pub fn level_start(
    level_state: Res<State<LevelState>>,
    mut change_level_state: ResMut<NextState<LevelState>>,
) {
    if level_state.get() == &LevelState::OutOfLevel {
        change_level_state.set(LevelState::Introduction);
    }
}

pub fn level_exit(
    mut change_level_state: ResMut<NextState<LevelState>>,
) {
    change_level_state.set(LevelState::OutOfLevel);
}

pub fn despawn_clock_time(
    mut commands: Commands,
) {
    commands.remove_resource::<LevelClock>();
    commands.remove_resource::<IntroTimer>();
}

// clock systems
pub fn update_level_clock(
    mut clock: ResMut<LevelClock>,
) {
    clock.0.update();
}

pub fn unpause_level_clock(
    mut clock: ResMut<LevelClock>,
) {
    clock.0.unpause();
}

pub fn pause_level_clock(
    mut clock: ResMut<LevelClock>,
) {
    clock.0.pause();
}

