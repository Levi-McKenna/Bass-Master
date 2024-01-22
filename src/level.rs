use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::{WorldCamera, Bassist};

// Marker Component
#[derive(Component)]
pub struct CurrentLevel();

pub fn load_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut level_handle: Handle<LdtkAsset> = asset_server.load("levels/built_in/Bass_Master_Level_Test.ldtk");
    commands.spawn((LdtkWorldBundle {
        ldtk_handle: level_handle,
        ..default()
    },
        CurrentLevel {}
    ));
}

// TODO: FIGURE OUT WHAT THE FUCK TO DO WITH THIS PIECE OF SHIT CRATE FOR LDTK
pub fn despawn_world(
    mut commands: Commands,
    level_query: Query<Entity, (With<Handle<LdtkAsset>>)>,
) {
    level_query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

const VIEWPORT_X: f32 = 500.0;
// fit camera to level when the camera has whitespace
pub fn fit_camera_to_level_player(
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

                // a lot of overhead with contructing a new Vec3 so i just separated the y and x
                // translations to be seperate floats
                camera_transform.translation.y = level.px_hei as f32 / 2.0;
                if (bassist_transform.translation.x + VIEWPORT_X >= level.px_wid as f32) {
                    camera_transform.translation.x = level.px_wid as f32 - VIEWPORT_X;
                } else {
                    camera_transform.translation.x = bassist_transform.translation.x;  
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

