use bevy::prelude::*;
use crate::{GameState};

pub fn state_inputs(
    input: Res<Input<KeyCode>>,
    mut game_state: Res<State<GameState>>
) {
    if (input::just_pressed(KeyCode::Escape)) {
        game_state.set(GameState::MainMenu).unwrap();
    }
}
