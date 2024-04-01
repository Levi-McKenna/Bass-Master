use bevy::prelude::*;
use std::collections::HashMap;
use crate::{GameState, bass::pitch_detector::StreamReceiver, LevelScore, CurrentBassNote};

#[derive(Event)]
pub struct BassInput(bool);

pub fn state_inputs(
    input: Res<Input<KeyCode>>,
    game_state: ResMut<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) && game_state.get() == &GameState::InGame {
        change_game_state.set(GameState::Paused);
    } else if input.just_pressed(KeyCode::Escape) && game_state.get() == &GameState::Paused {
        change_game_state.set(GameState::InGame);
    } 
}

pub fn read_input_stream(
    mut input_events: EventWriter<BassInput>,
    current_note: Res<CurrentBassNote>,
    receiver: Res<StreamReceiver>,
) {
    // hashmap containing chord to frequency corresponding values
    let freq_to_note: HashMap<&str, [(f64, f64); 11]> = HashMap::from([
        ("E", [
            (20., 21.),
            (22., 23.),
            (23., 24.),
            (24., 25.),
            (26., 27.),
            (27., 28.),
            (29., 30.),
            (31., 32.),
            (32., 33.),
            (34., 36.),
            (36., 38.),
        ]),
        ("A", [
            (26., 28.),
            (28., 30.),
            (30., 32.),
            (32., 34.),
            (34., 36.),
            (36., 38.),
            (38., 40.),
            (40., 42.),
            (42., 45.),
            (45., 47.),
            (47., 49.),
        ]),
        ("D", [
            (36., 37.),
            (39., 40.),
            (41., 42.),
            (43., 44.),
            (46., 47.),
            (49., 50.),
            (52., 53.),
            (55., 56.),
            (58., 59.),
            (61., 63.),
            (65., 66.),
        ]),
        ("G", [
            (48., 50.),
            (52., 53.),
            (54., 55.5),
            (58., 59.),
            (61., 62.),
            (65., 66.),
            (69., 70.),
            (73., 74.),
            (77., 78.),
            (82., 83.),
            (87., 88.),
        ]),
    ]);

    for estimate in receiver.try_iter() {
        let freq_bounds = freq_to_note.get(current_note.chord.as_str()).unwrap();
        if estimate >= freq_bounds[current_note.fret as usize].0 && estimate <= freq_bounds[current_note.fret as usize].1 {
            println!("Chord -> {}, Fret -> {}", current_note.chord, current_note.fret);
            input_events.send(BassInput(true));
        } else {
            input_events.send(BassInput(false));
        }
    }
    println!("Yeah we reading it");
}

pub fn print_if_true(
    mut correct_events: EventReader<BassInput>,
    mut score: ResMut<LevelScore>,
) {
    for events in correct_events.iter() {
        if events.0 {
            score.0 += 200;
        } else {
            score.0 -= 50;
        }
        println!("Score -> {}", score.0);
    }
    correct_events.clear();
}

pub fn reset_score(
    mut score: ResMut<LevelScore>,
) {
    score.0 = 0;
}
