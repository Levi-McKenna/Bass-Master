use bevy::prelude::*;

pub fn setup_load_screen(
    mut commands: Commands
) {
    commands.spawn(NodeBundle {
        ..default()
    });
}
