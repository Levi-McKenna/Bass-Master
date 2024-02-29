use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_asset_loader::prelude::*;
use std::path::{PathBuf};
use crate::{LevelResource};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SongState {
    #[default]
    NoSong,
    Introduction,
    Playing
}

#[derive(Debug, Resource)]
pub struct SongDuration(Time);

pub fn spawn_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_path: Res<LevelResource>,
) {
    // set the path and path extension and coerce to str
    let path = &level_path.0;
    let mut song_path = PathBuf::from(r"./levels/level_songs/temp");
    song_path.set_file_name(path.file_name().unwrap());
    song_path.set_extension("wav");
    let path_str = song_path.to_str().unwrap();

    commands.spawn((AudioBundle {
        source: asset_server.load(path_str),
        settings: PlaybackSettings {
            paused: true,
            ..default()
        },
    }));
    commands.insert_resource(SongDuration(Time::new(Instant::now())));
}

pub fn pause_song_time(
    mut song_time: ResMut<SongDuration>,
) {
    song_time.0.pause();
}

pub fn update_time(
    mut song_time: ResMut<SongDuration>,
) {
    if song_time.0.is_paused() {
        song_time.0.unpause();
    }
    song_time.0.update();
}

pub fn print_song_time(
    song_time: Res<SongDuration>,
) {
    println!("{:?}", song_time.0.elapsed_seconds());
}
