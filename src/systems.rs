use crate::{WorldCamera, Bassist};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;


const VIEWPORT_X: f32 = 500.0;
// fit camera to level when the camera has whitespace
pub fn fit_camera_to_level(
    mut camera_query: Query<(&mut bevy::render::camera::OrthographicProjection, &mut Transform), With<WorldCamera>>,
    mut level_query: Query<(&Handle<LdtkLevel>), (Without<Bassist>, Without<WorldCamera>)>,
    bassist_query: Query<&Transform, (With<Bassist>, Without<WorldCamera>)>,
    window_query: Query<&Window>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (mut projection, mut camera_transform) in camera_query.iter_mut() {
        for (level_handle) in level_query.iter_mut(){
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let window = window_query.single();
                let level = &ldtk_level.level;
                let bassist_transform = bassist_query.single();

                // TODO: Fix the camera juggling between beginning and end when bassist is out of
                // bounds
                if (camera_transform.translation.x + VIEWPORT_X >= level.px_wid as f32) {
                    camera_transform.translation = Vec3::new(level.px_wid as f32 - VIEWPORT_X, level.px_hei as f32 / 2.0, 0.0);
                } else {
                    camera_transform.translation = Vec3::new(bassist_transform.translation.x, level.px_hei as f32 / 2.0, 0.0);  
                }

                // A scale that brings the top and the bottom of the world into frame.
                // (px_hei corresponds to the height of the window {pixels increase as window
                // height increases} so it doesn't matter how
                // large the viewport of the camera is)
                projection.scale = level.px_hei as f32 / window.height() as f32;
            }
        }       
    }
} 
