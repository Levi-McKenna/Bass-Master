use bevy::prelude::*;
use crate::{GameState, LevelState};

pub fn state_inputs(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) && game_state.get() == &GameState::InGame {
        change_game_state.set(GameState::Paused);
    } else if input.just_pressed(KeyCode::Escape) && game_state.get() == &GameState::Paused {
        change_game_state.set(GameState::InGame);
    } 
}

/* pub fn on_bass_note_hit(
    
) {

} */
