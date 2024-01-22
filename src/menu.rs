use bevy::prelude::*;
use bevy::ui::*;
use bevy_ecs_ldtk::prelude::*;

pub fn spawn_menu_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/built_in/Main_Menu.ldtk"),
        ..default()
    });
}

pub fn draw_menu_ui(
    mut commands: Commands
) {
    commands.spawn(NodeBundle {
        ..default()
    });
}
