use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::{WorldCamera, Bassist, LevelState};

// Two identical functions coming right up
// fit camera to level when the camera has whitespace
pub fn fit_camera_to_window (
    mut camera_query: Query<(&mut bevy::render::camera::OrthographicProjection, &mut Transform), With<WorldCamera>>,
    level_query: Query<(&Handle<LdtkLevel>), (Without<Bassist>, Without<WorldCamera>)>,
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
                let scale_factor: f32 = level.px_hei as f32 / window.height();
                projection.scale = scale_factor;
                camera_transform.translation.y = level.px_hei as f32 / 2.0;
                // if camera is out of bounds to the left then translate x multiplied by the scale
                // factor
                if (camera_transform.translation.x - window.width() / 2.0) <= 0.0 { 
                    camera_transform.translation.x = (window.width() / 2.0) * scale_factor;
                }
            }
        }       
    }
} 


pub fn handle_level_camera_translations(
    mut camera_query: Query<&mut Transform, With<WorldCamera>>,
    level_query: Query<(&Handle<LdtkLevel>), (Without<Bassist>, Without<WorldCamera>)>,
    window_query: Query<&Window>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    level_state: Res<State<LevelState>>,
) {
    for level_handle in level_query.iter(){
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let window = window_query.single();
            let level = &ldtk_level.level;
            let mut camera_transform = camera_query.single_mut();

            if level_state.get() == &LevelState::Ending {
                camera_transform.translation.x = level.px_wid as f32 - window.width() as f32 / 2.0;
            }
        }
    }       
    
} 

