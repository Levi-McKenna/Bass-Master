mod player;
mod level;
mod input;
mod menu;
mod load_screen;
mod camera;
mod bass_ui;
mod song;

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
use input::state_inputs;
use load_screen::*;
use camera::*;
use bass_ui::*;
use song::*;

// States for game status
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
    // TODO: Switch default to MenuAssetLoading when the menu assets are ready to be implemented
    #[default]
    MenuAssetLoading,
    AssetLoading,
    AssetsLoaded,
}

#[derive(Component)]
pub struct WorldCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            ..default()
        },
        WorldCamera
    ));
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
        // main menu state management
        .add_state::<GameState>()
        .add_state::<LevelState>()
        .add_state::<SongState>()
        .add_state::<NoteState>()
        // Ldtk Entities
        .register_ldtk_entity::<JumpBundle>("Jump")
        .register_ldtk_entity::<JumpBundle>("JumpReverse")
        // Asset loading state for the main menu
        .add_loading_state(
            LoadingState::new(GameState::MenuAssetLoading)
                .continue_to_state(GameState::MainMenu)
        )
        .add_systems(OnEnter(GameState::MenuAssetLoading), draw_menu_ui)
        // Asset loading state that continues to pre-level systems
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::AssetsLoaded)
        )
        // Dynamic Asset Loading Location
        // TODO: FIX THIS SHIT (STANDARDDYNAMICASSETCOLLECTION)
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(GameState::AssetLoading, "textures/Bass_Tablature/dynamic_assets.assets.ron",)
        // Asset colection that need to be loaded in the GameState::AssetLoading state
        .add_collection_to_loading_state::<_, CharacterAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, BassPickAsset>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, VerticalBassStrumAsset>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, BassStringAsset>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, BassNoteAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, FretNumberAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, LevelSongAssets>(GameState::AssetLoading)
        // systems to spawn assets into the world
        .add_systems(OnEnter(GameState::AssetLoading), (insert_level_metadata, spawn_load_screen))
        .add_systems(OnExit(GameState::AssetLoading), (spawn_music, spawn_bass_ui, spawn_character, handle_level_camera_translations, load_world))
        // all systems for pre-level start
        .add_systems(Update, exit_load_screen.run_if(in_state(GameState::AssetsLoaded)))
        .add_systems(OnExit(GameState::AssetsLoaded), (pause_song_time, set_player_bounds, spawn_bass_notes))
        // MainMenu systems
        /* .add_systems(OnEnter(GameState::MainMenu), (level_start, spawn_menu_world))
        .add_systems(Update, (fit_camera_to_window, handle_level_camera_translations).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), (despawn_character, despawn_world)) */
        // InGame systems
        .add_systems(OnEnter(GameState::InGame), (insert_beat_coords, fit_camera_to_window, level_start))
        .add_systems(Update, (player_movement, translate_bass_notes).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (manage_note_state,manage_level_states, handle_level_camera_translations, update_time).run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), (despawn_world, despawn_character))
        // SongState Introduction
        .add_systems(Startup, (set_window_icon, pause_level_clock, setup))
        .add_systems(Update, (state_inputs))
        .run();
}
