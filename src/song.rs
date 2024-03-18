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

#[derive(Component)]
pub struct Song;

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
    },
        Song,
    ));
}

pub fn despawn_music(
    mut commands: Commands,
    song_query: Query<Entity, With<Song>>,
) {
    if let Ok(song) = song_query.get_single() {
        commands.entity(song).despawn_recursive();
    }
}

pub fn pause_song(
    mut song_settings_query: Query<&AudioSink>,
) {
    let mut song_settings = song_settings_query.single_mut();
    song_settings.pause();
}

