use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::{WorldCamera, Bassist, GameState};

// Marker Component
#[derive(Component)]
pub struct CurrentLevel();

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    #[default]
    OutOfLevel,
    Introduction,
    Playing,
    Ending
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "levels/built_in/Bass_Master_Level_Test.ldtk")]
    handle: Handle<LdtkAsset>
}

pub fn load_world(
    mut commands: Commands,
    level_collection: Res<LevelAssets>,
) {
    commands.spawn((LdtkWorldBundle {
        ldtk_handle: level_collection.handle.clone(),
        ..default()
    },
        CurrentLevel {}
    ));
}

pub fn despawn_world(
    mut commands: Commands,
    level_query: Query<Entity, (With<Handle<LdtkAsset>>)>,
) {
    level_query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

// Two identical functions coming right up
const VIEWPORT_X: f32 = 500.0;
// fit camera to level when the camera has whitespace
pub fn fit_camera_to_window (
    mut camera_query: Query<(&mut bevy::render::camera::OrthographicProjection, &mut Transform), With<WorldCamera>>,
    mut level_query: Query<(&Handle<LdtkLevel>), (Without<Bassist>, Without<WorldCamera>)>,
    window_query: Query<&Window>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (mut projection, mut camera_transform) in camera_query.iter_mut() {
        for (level_handle) in level_query.iter(){
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let window = window_query.single();
                let level = &ldtk_level.level;

                // A scale that brings the top and the bottom of the world into frame.
                // (px_hei corresponds to the height of the window {pixels increase as window
                // height increases} so it doesn't matter how
                // large the viewport of the camera is)
                projection.scale = level.px_hei as f32 / window.height() as f32;
                camera_transform.translation.y = level.px_hei as f32 / 2.0;
            }
        }       
    }
} 


pub fn handle_level_camera_translations(
    mut camera_query: Query<&mut Transform, With<WorldCamera>>,
    character_query: Query<&Transform, (With<Bassist>, Without<WorldCamera>)>,
    level_query: Query<(&Handle<LdtkLevel>), (Without<Bassist>, Without<WorldCamera>)>,
    window_query: Query<&Window>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    mut change_level_state: ResMut<NextState<LevelState>>,
    level_state: Res<State<LevelState>>,
) {
    for level_handle in level_query.iter(){
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let window = window_query.single();
            let level = &ldtk_level.level;
            let mut camera_transform = camera_query.single_mut();
            let character_transform = character_query.single();

            // if camera is out of bounds left of the level
            if (camera_transform.translation.x - window.width() as f32 / 2.0) <= 0.0 { 
                camera_transform.translation.x = window.width() as f32 / 2.0
            }
            // TODO: turn the start resource into a more viable state
            match level_state.get() {
                // if character reaches start of level
                &LevelState::Introduction if character_transform.translation.x >= (camera_transform.translation.x) => 
                    change_level_state.set(LevelState::Playing),
                // if camera is out of bounds of level
                &LevelState::Playing if (camera_transform.translation.x + window.width() as f32 / 2.0) >= level.px_wid as f32 =>  
                    change_level_state.set(LevelState::Ending),
                &LevelState::Ending => 
                    camera_transform.translation.x = level.px_wid as f32 - window.width() as f32 / 2.0,
                // if nuthin
                &_ => (),
            }
            // If world will be out of bounds in either direction before and after the bassist
            // is off or on screen we set the x translation accordingly 
        }
    }       
    
} 

pub fn level_start(
    game_state: Res<State<GameState>>,
    mut change_level_state: ResMut<NextState<LevelState>>,
) {
    if game_state.get() == &GameState::InGame {
        change_level_state.set(LevelState::Introduction);
    }
}
