use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn load_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/built_in/Bass_Master_Level_Test.ldtk"),
        ..default()
    });
}

