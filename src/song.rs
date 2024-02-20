use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_asset_loader::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SongState {
    #[default]
    NoSong,
    Introduction,
    Playing
}

#[derive(Debug, Resource)]
pub struct SongDuration(Time);

#[derive(AssetCollection, Resource)]
pub struct LevelSongAssets {
    #[asset(path = "levels/built_in/level_songs/Everlong_Snippet.wav")]
    song: Handle<AudioSource>,
}

pub fn spawn_music(
    mut commands: Commands,
    level_song_assets: Res<LevelSongAssets>,
) {
    commands.spawn((AudioBundle {
        source: level_song_assets.song.clone(),
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
