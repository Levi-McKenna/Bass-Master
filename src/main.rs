mod player_movement;

use bevy::prelude::*;
use player_movement::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_character))
        .add_systems(Update, (player_move))
        .run();
}
