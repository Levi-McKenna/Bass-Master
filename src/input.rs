use bevy::prelude::*;
use crate::{GameState, LevelState, NoteCollision, bass::pitch_detector::StreamReceiver, LevelScore};

#[derive(Event)]
pub struct BassInput(bool);

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

pub fn read_input_stream(
    receiver: Res<StreamReceiver>,
    mut input_events: EventWriter<BassInput>,
    mut collision_events: EventReader<NoteCollision>,
) {
    for (fret, chord) in receiver.try_iter() {
        for collision in collision_events.iter() {
            if fret == collision.fret && chord == collision.chord {
                input_events.send(BassInput(true));
            } else {
                input_events.send(BassInput(false));
            }
        }
        collision_events.clear();
    }
}

pub fn print_if_true(
    mut correct_events: EventReader<BassInput>,
    mut score: ResMut<LevelScore>,
) {
    for events in correct_events.iter() {
        if events.0 {
            score.0 += 100;
        } else {
            score.0 -= 50;
        }
        println!("{}", score.0);
    }
    correct_events.clear();
}

pub fn reset_score(
    mut score: ResMut<LevelScore>,
) {
    score.0 = 0;
}
