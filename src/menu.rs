use bevy::prelude::*;
use bevy::ui::*;

pub fn draw_menu_ui(
    mut commands: Commands
) {
    commands.spawn(NodeBundle {
        ..default()
    });
}
