use bevy::prelude::*;
use crate::{LevelState, GameState};

#[derive(Component)]
pub struct LoadScreenNodeBundle;

pub fn spawn_load_screen(
    mut commands: Commands
) {
    commands.spawn((NodeBundle {
        ..default()
    },
    LoadScreenNodeBundle
    ));
}

pub fn despawn_load_screen(
    mut commands: Commands,
    load_screen_query: Query<Entity, With<LoadScreenNodeBundle>>,
) {
    load_screen_query.for_each(|load_screen_nodes| {
        commands.entity(load_screen_nodes).despawn();
    });
}

pub fn load_finished_state_change(
    mut change_level_state: ResMut<NextState<LevelState>>,
) {
    change_level_state.set(LevelState::Introduction);
}
