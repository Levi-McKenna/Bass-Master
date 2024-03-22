mod player;
mod level;
mod input;
mod menu;
mod load_screen;
mod camera;
mod bass_ui;
mod song;
mod bass;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::render::settings::Backends;
use belly::prelude::*;
use bevy::window::WindowMode;
use bevy::winit::WinitWindows;
use winit::window::Icon;
use player::*;
use level::*;
use menu::*;
use input::*;
use load_screen::*;
use camera::*;
use bass_ui::*;
use song::*;
use bass::pitch_detector::*;

// States for game status
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
    Ending,
    #[default]
    MenuAssetLoading,
    AssetLoading,
    AssetsLoaded,
}

#[derive(Component)]
pub struct WorldCamera;

#[derive(Resource)]
pub struct LevelScore(i32);

#[derive(Resource)]
pub struct CurrentBassNote {
    chord: String,
    fret: i8,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            ..default()
        },
        WorldCamera
    ));
}

fn pause_game_clock(
    mut time: ResMut<Time>,
) {
    time.pause();
}

fn unpause_game_clock(
    mut time: ResMut<Time>,
) {
    time.unpause();
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("./assets/textures/Bass-Master-Icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }
        }).set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
        }))
        .add_plugins((LdtkPlugin, BellyPlugin))
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LevelScore(0))
        .insert_resource(CurrentBassNote {
            chord: "".to_string(),
            fret: -1,
        })
        .add_event::<PlayEvent>()
        .add_event::<QuitEvent>()
        .add_event::<ExitLevelEvent>()
        .add_event::<WorldEvent>()
        .add_event::<NoteCollision>()
        .add_event::<BassInput>()
        // main menu state management
        .add_state::<GameState>()
        .add_state::<LevelState>()
        .add_state::<SongState>()
        .add_state::<NoteState>()
        // Ldtk Entities
        .register_ldtk_entity::<JumpBundle>("Jump")
        // Asset loading state for the main menu
        .add_loading_state(
            LoadingState::new(GameState::MenuAssetLoading)
                .continue_to_state(GameState::MainMenu)
        )
        .add_systems(OnExit(GameState::MenuAssetLoading), draw_main_menu_ui)
        // MainMenu Systems
        .add_systems(Update, (close_event, insert_world_dir).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), despawn_ui)
        // Asset loading state that continues to pre-level systems
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::AssetsLoaded)
        )
        // Dynamic Asset Loading Location
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(GameState::AssetLoading, "textures/Bass_Tablature/dynamic_assets.assets.ron",)
        // Asset colection that need to be loaded in the GameState::AssetLoading state
        .add_collection_to_loading_state::<_, CharacterAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, BassPickAsset>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, VerticalBassStrumAsset>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, BassStringAsset>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, BassNoteAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, FretNumberAssets>(GameState::AssetLoading)
        // systems to spawn assets into the world
        .add_systems(OnEnter(GameState::AssetLoading), (insert_level_metadata, spawn_load_screen))
        .add_systems(OnExit(GameState::AssetLoading), (spawn_score, spawn_music, spawn_bass_ui, spawn_character, load_world))
        // all systems for pre-level start
        .add_systems(OnEnter(GameState::AssetsLoaded), (set_player_bounds))
        .add_systems(Update, exit_load_screen.run_if(in_state(GameState::AssetsLoaded)))
        .add_systems(OnExit(GameState::AssetsLoaded), (fit_camera_to_window, spawn_bass_notes, insert_beat_coords))
        // MainMenu systems
        /* .add_systems(OnEnter(GameState::MainMenu), (level_start, spawn_menu_world))
        .add_systems(Update, (fit_camera_to_window, handle_level_camera_translations).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), (despawn_character, despawn_world)) */
        // InGame systems
        .add_systems(OnEnter(GameState::InGame), (read_audiostream, level_start, unpause_game_clock))
        .add_systems(Update, (translate_bass_notes, update_level_clock).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (player_movement, read_input_stream, print_if_true, write_note_collision).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (game_state_end, update_score, manage_level_states, handle_level_camera_translations).run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), (pause_level_clock, pause_game_clock))
/*         .add_systems(OnExit(GameState::InGame), (despawn_world, despawn_character)) */
        // GameState::Paused 
        .add_systems(OnEnter(GameState::Paused), (draw_game_menu_ui, pause_song))
        .add_systems(Update, (exit_level_event, play_event).run_if(in_state(GameState::Paused)))
        .add_systems(OnExit(GameState::Paused), (despawn_ui))
        // GameState::Ending
        .add_systems(OnEnter(GameState::Ending), (despawn_clock_time, despawn_world, despawn_character, despawn_bass_ui, despawn_music, reset_camera, level_exit, reset_score, despawn_score).before(load_main_menu))
        .add_systems(Update, (load_main_menu).run_if(in_state(GameState::Ending)))
        .add_systems(Startup, (set_window_icon, setup))
        .add_systems(Update, state_inputs)
        .run();
}
