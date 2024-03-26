use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy::sprite::Anchor;
use belly::prelude::*;
use serde::Deserialize;
use std::ops::Index;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use crate::{WorldCamera, LevelState, CurrentJumpCoord, JumpCoords, LevelResource, LevelClock, Bassist, LevelScore, CurrentBassNote};

#[derive(Event)]
pub struct NoteCollision {
    pub chord: String,
    pub fret: i8,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum NoteState {
    #[default]
    Rest,
    Playing,
}

// identifier structs
#[derive(Component)]
pub struct BassUI;

#[derive(Component)]
pub struct BassNotes { 
    note: i8,
    fret: i8,
    chord: String,
    speed_manipulation: f32,
}

#[derive(Component)]
pub struct BassFrets;

#[derive(Component)]
pub struct BassPick;

#[derive(Component)]
pub struct ScoreUI;

// structs
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Resource)]
pub struct Note {
    String: String,
    Fret: i8,
    Note: i8,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Resource)]
pub struct MusicJson {
    BPM: i16,
    Duration: f32,
    Beats: i8,
    NoteValue: i8,
    Notes: Vec<Note>,
}

#[derive(Resource)]
pub struct IntroTimer(pub Timer);

#[derive(Component)]
pub struct BassString(String);

// Asset Collections
#[derive(AssetCollection, Resource)]
pub struct VerticalBassStrumAsset {
    #[asset(texture_atlas(tile_size_x = 10., tile_size_y = 250., columns = 10, rows = 250, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "textures/bass_strum_vertical.png")]
    sprite: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct BassStringAsset {
    #[asset(texture_atlas(tile_size_x = 450., tile_size_y = 9., columns = 450, rows = 9, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "textures/bass_string.png")]
    sprite: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct BassPickAsset {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 64, rows = 64, padding_x = 0., padding_y = 0., offset_x = 0., offset_y = 0.))]
    #[asset(path = "textures/pick_sprite.png")]
    pub sprite: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct BassNoteAssets {
    #[asset(key = "note.half")]
    half: Handle<TextureAtlas>,
    #[asset(key = "note.quarter")]
    quarter: Handle<TextureAtlas>,
    #[asset(key = "note.eighth")]
    eighth: Handle<TextureAtlas>,
    #[asset(key = "note.sixteenth")]
    sixteenth: Handle<TextureAtlas>,
    #[asset(key = "note.thirty-second")]
    thirty_second: Handle<TextureAtlas>,
}

impl Index<i8> for BassNoteAssets {
    type Output = Handle<TextureAtlas>;
    fn index(&self, n: i8) -> &Handle<TextureAtlas> {
        match n {
            2 => &self.half,
            4 => &self.quarter,
            8 => &self.eighth,
            16 => &self.sixteenth,
            32 => &self.thirty_second,
            _ => panic!("!! Out of scope of our note assets !!")
        }
    } 
}

#[derive(AssetCollection, Resource)]
pub struct FretNumberAssets {
    #[asset(key = "fret.0")]
    zero: Handle<TextureAtlas>,
    #[asset(key = "fret.1")]
    one: Handle<TextureAtlas>,
    #[asset(key = "fret.2")]
    two: Handle<TextureAtlas>,
    #[asset(key = "fret.3")]
    three: Handle<TextureAtlas>,
    #[asset(key = "fret.4")]
    four: Handle<TextureAtlas>,
    #[asset(key = "fret.5")]
    five: Handle<TextureAtlas>,
    #[asset(key = "fret.6")]
    six: Handle<TextureAtlas>,
    #[asset(key = "fret.7")]
    seven: Handle<TextureAtlas>,
    #[asset(key = "fret.8")]
    eight: Handle<TextureAtlas>,
    #[asset(key = "fret.9")]
    nine: Handle<TextureAtlas>,
    #[asset(key = "fret.10")]
    ten: Handle<TextureAtlas>,
}

impl Index<i8> for FretNumberAssets {
    type Output = Handle<TextureAtlas>;
    fn index(&self, n: i8) -> &Handle<TextureAtlas> {
        match n {
            0 => &self.zero,
            1 => &self.one,
            2 => &self.two,
            3 => &self.three,
            4 => &self.four,
            5 => &self.five,
            6 => &self.six,
            7 => &self.seven,
            8 => &self.eight,
            9 => &self.nine,
            10 => &self.ten,
            _ => panic!("!! Out of scope of our note assets !!")
        }
    } 
}

const HORIZONTAL_BASS_WIDTH: f32 = 450.;
const VERTICAL_BASS_HEIGHT: f32 = 50.;
const NOTE_WIDTH: f32 = 10.;
const NOTE_OFFSET: f32 = 10.;
const INTRODUCTION_LENGTH_SECONDS: f32 = 4.;

pub fn spawn_bass_ui(
    mut commands: Commands,
    bass_pick_asset: Res<BassPickAsset>,
    bass_note_assets: Res<BassNoteAssets>,
    vertical_bass_strum_asset: Res<VerticalBassStrumAsset>,
    bass_string_asset: Res<BassStringAsset>,
)  {
    // spawn for guitar UI strings and picks
    commands.spawn((SpriteSheetBundle {
        texture_atlas: vertical_bass_strum_asset.sprite.clone(),
        transform: Transform::from_xyz(10.0, 330.0, 100.0),
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::new(2., VERTICAL_BASS_HEIGHT)),
            anchor: Anchor::TopCenter,
            color: Color::rgba(1.0, 1.0, 1.0, 0.5),
            ..default()
        },
        ..default()
    },
        BassUI,
    )).with_children(|parent| {
            // spawn for horizontal strings
            for i in 0..4 {
                parent.spawn((
                    SpriteSheetBundle {
                        texture_atlas: bass_string_asset.sprite.clone(),
                        // translate the y-axis down for each string
                        transform: Transform::from_xyz(0.0, (-10. * i as f32) - 10., 50.0),
                        sprite: TextureAtlasSprite {
                            custom_size: Some(Vec2::new(450., 1.5)),
                            anchor: Anchor::CenterLeft,
                            color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                            ..default()
                        },
                        ..default()
                    },
                    // Component for querying specific strings
                    match i {
                        0 => BassString("G".to_string()),
                        1 => BassString("D".to_string()),
                        2 => BassString("A".to_string()),
                        3 => BassString("E".to_string()),
                        _ => panic!("No string correlates to i-index")
                    },
                ));
            }
            // spawn for bass pick
            parent.spawn((SpriteSheetBundle {
                texture_atlas: bass_pick_asset.sprite.clone(),
                // small y offset for looks :}
                transform: Transform::from_xyz(0.0, -1.0, 200.0),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(16.)),
                    anchor: Anchor::Center,
                    flip_y: true,
                    ..default()
                },
                ..default()
            },
            BassPick
            ));
        });
}

pub fn insert_level_metadata(
    mut commands: Commands,
    level_path_resource: Res<LevelResource>,
) {
    // switch extension to json
    let mut level_path = level_path_resource.0.clone();
    level_path.set_extension("json");
    let level_path = format!("./assets/{}", level_path.to_str().unwrap());

    commands.insert_resource(parse_bass_json(level_path).unwrap());
}

pub fn spawn_bass_notes(
    mut commands: Commands,
    string_query: Query<(&Parent, &Transform, &BassString), (With<Handle<TextureAtlas>>)>,
    fret_number_assets: Res<FretNumberAssets>,
    bass_note_assets: Res<BassNoteAssets>,
    tablature: Res<MusicJson>,
) {
    let mut position_x = 0.;

    for (note) in &tablature.Notes {
        for (parent, string_transform, string_letter) in string_query.iter() {
            // check for the correct string
            if note.String == string_letter.0 {
                // scale the string's transform to be parented by the bass note
                let string_y = string_transform.translation.y + VERTICAL_BASS_HEIGHT + 5.;

                // note spawns
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: bass_note_assets[note.Note].clone(),
                        transform: Transform::from_xyz(450. + position_x, -50. - 5., 100.),
                        visibility: Visibility::Hidden,
                        sprite: TextureAtlasSprite {
                            anchor: Anchor::TopLeft,
                            custom_size: Some(Vec2::splat(10.)),
                            ..default()
                        },
                        ..default()
                    },
                    BassNotes { 
                        note: note.Note,
                        fret: note.Fret,
                        chord: note.String.clone(),
                        speed_manipulation: note_speed_manipulation(note.Note, tablature.BPM, tablature.NoteValue), 
                    }
                )).with_children(|parent| {
                        parent.spawn((
                            SpriteSheetBundle {
                                texture_atlas: fret_number_assets[note.Fret].clone(),
                                transform: Transform::from_xyz(0., string_y, 500.),
                                sprite: TextureAtlasSprite {
                                    anchor: Anchor::Center,
                                    custom_size: Some(Vec2::splat(5.)),
                                    ..default()
                                },
                                ..default()
                            },
                            BassFrets
                        ));
                    }).set_parent(parent.get());

                // Fret number spawns
            }
        }
        position_x += NOTE_OFFSET;
    }
}

pub fn despawn_bass_ui(
    mut commands: Commands,
    bass_ui_query: Query<Entity, With<BassUI>>,
) {
    if let Ok(bass_ui) = bass_ui_query.get_single() {
        commands.entity(bass_ui).despawn_recursive();
    }
}

pub fn write_note_collision(
    mut commands: Commands,
    mut writer: EventWriter<NoteCollision>,
    mut current_note: ResMut<CurrentBassNote>,
    bass_note_query: Query<(&Transform, &BassNotes, Entity)>,
    pick_query: Query<&Transform, (With<BassPick>, Without<BassNotes>)>,
) {
    let pick_transform = pick_query.single();

    for (bass_note_transform, bass_note, entity) in &bass_note_query {
        if bass_note_transform.translation.x <= pick_transform.translation.x &&
        bass_note_transform.translation.x >= pick_transform.translation.x - 5. {
            writer.send(NoteCollision { chord: bass_note.chord.clone(), fret: bass_note.fret });
            *current_note = CurrentBassNote {
                chord: bass_note.chord.clone(),
                fret: bass_note.fret,
            };
            commands.entity(entity).despawn_recursive();
        }
    }
}


pub fn translate_bass_notes(
    mut bass_note_query: Query<(&mut Transform, &mut Visibility, &BassNotes), (With<BassNotes>, Without<BassPick>)>,
    level_state: Res<State<LevelState>>,
    mut intro_timer: ResMut<IntroTimer>,
    pick_query: Query<&Transform, (With<BassPick>, Without<BassNotes>)>,
    mut audio_query: Query<&AudioSink>,
    mut change_note_state: ResMut<NextState<NoteState>>,
    mut writer: EventWriter<NoteCollision>,
    tablature: Res<MusicJson>,
    mut time: ResMut<LevelClock>,
) {
    let pick_transform = pick_query.single();
    let mut audio_settings = audio_query.single_mut();

    intro_timer.0.tick(time.0.delta());
    for (mut bass_note_transform, mut bass_note_visibility, note) in &mut bass_note_query {
        // check to see if translations are needed
        if bass_note_transform.translation.x + NOTE_WIDTH <= HORIZONTAL_BASS_WIDTH {
            *bass_note_visibility = Visibility::Visible;
        }

        if intro_timer.0.finished()  {
            // unpause song if needed
            if audio_settings.is_paused() {
                audio_settings.play();
            }

            let mut speed_scale: f32;
            if bass_note_transform.translation.x <= pick_transform.translation.x ||
            bass_note_transform.translation.x - NOTE_OFFSET <= pick_transform.translation.x {
                speed_scale = note.speed_manipulation;
            } else {
                speed_scale = note_speed_manipulation(tablature.NoteValue, tablature.BPM, tablature.NoteValue);
            }
            bass_note_transform.translation.x -= (NOTE_WIDTH + NOTE_OFFSET) * (time.0.delta_seconds() / speed_scale);
        } else {
            bass_note_transform.translation.x -= (HORIZONTAL_BASS_WIDTH) * (time.0.delta_seconds() / intro_timer.0.duration().as_secs_f32());
/*             println!("Elapsed -> {}, Intro Time -> {}", time.0.elapsed_seconds(), intro_time.0); */
        }
    }
}

fn note_speed_manipulation(
    note: i8,
    bpm: i16,
    note_value: i8,
) -> f32 {
    // scale for the beats per minute
    let bottom_value = (note / note_value) as i16 * bpm;

    // (https://dobrian.github.io/cmp/topics/basics-of-music-theory/6.representing-rhythm.html#:~:text=At%20quarter%20%3D%2060%2C%20each%20quarter,seconds%20long%2C%20or%202000%20milliseconds) for more info about this equation
    let note_length: f32 = 60000. / bottom_value as f32;
    // scale for seconds
    note_length / 1000.
}

fn parse_bass_json<P: AsRef<Path>>(path: P) -> Result<MusicJson, Box<dyn Error>> {
    let json_file = File::open(path)?;
    let reader = BufReader::new(json_file);
    // read JSON contents
    let tablature = serde_json::from_reader(reader)?;

    Ok(tablature)
}

/* // This is for managing the note state that controls the type of player movement in player.rs.
pub fn manage_note_state(
    bass_note_query: Query<&Transform, (With<BassNotes>, Without<BassPick>)>,
    pick_query: Query<&Transform, (With<BassPick>, Without<BassNotes>)>,
    mut change_note_state: ResMut<NextState<NoteState>>,
    note_state: Res<State<NoteState>>,
    mut grid_coord_index: ResMut<CurrentJumpCoord>,
    grid_coords: Res<JumpCoords>,
) {
    let pick_transform = pick_query.single();

    for note_transform in bass_note_query.iter() {

        if (note_transform.translation.x <= pick_transform.translation.x && note_transform.translation.x >= pick_transform.translation.x - 2.) &&
            grid_coord_index.0 < grid_coords.0.len() && note_state.get() != &NoteState::Playing{
            change_note_state.set(NoteState::Playing);
        } else if note_state.get() != &NoteState::Rest{
            change_note_state.set(NoteState::Rest);
        }
    }
} */

pub fn spawn_score(
    mut commands: Commands,
) {
    // scorespawn
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        },
        ScoreUI,
    )).with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "0",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::rgba(0., 0.9098039215686274, 1., 1.),
                        ..default()
                    }
                ),
                ..default()
            }.with_style(Style {
                 ..default()   
            }),
            Label));
    });
}

pub fn update_score(
    mut text_query: Query<&mut Text>,
    score: Res<LevelScore>,
) {
    let mut sections = text_query.single_mut();
    for mut section in sections.sections.iter_mut() {
        section.value = score.0.to_string();
    }
}

pub fn despawn_score(
    mut commands: Commands,
    score_query: Query<Entity, With<ScoreUI>>,
) {
    for score in score_query.iter() {
        commands.entity(score).despawn_recursive();
    }
}
