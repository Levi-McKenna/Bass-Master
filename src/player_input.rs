use bevy::prelude::*;
use crate::{GameState};

pub fn state_inputs(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if (input.just_pressed(KeyCode::Escape)) {
        game_state.set(GameState::MainMenu);
    }

    if (input.just_pressed(KeyCode::S)) {
        game_state.set(GameState::InGame);
    }
}
