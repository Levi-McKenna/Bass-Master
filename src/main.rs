mod player;
mod level;
mod input;
mod menu;
mod load_screen;
mod camera;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use bevy::render::settings::Backends;
use player::*;
use level::*;
use menu::*;
use input::state_inputs;
use load_screen::*;
use camera::*;

// States for game status
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
    // TODO: Switch default to MenuAssetLoading when the menu assets are ready to be implemented
    MenuAssetLoading,
    #[default]
    AssetLoading,
    AssetsLoaded,
}

#[derive(Component)]
pub struct WorldCamera;

#[derive(Resource)]
pub struct MovementTimer(Timer);

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            ..default()
        },
        WorldCamera
    ));
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PreGameAssetLoadSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }
        }))
        .add_plugins(LdtkPlugin)
        .insert_resource(MovementTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
        .insert_resource(LevelSelection::Index(0))
        // main menu state management
        .add_state::<GameState>()
        .add_state::<LevelState>()
        // LoadScreen systems
        .add_loading_state(
            LoadingState::new(GameState::MenuAssetLoading)
                .continue_to_state(GameState::MainMenu)
        )
        // Asset Collections go here 
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::AssetsLoaded)
        )
        .add_collection_to_loading_state::<_, CharacterAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::AssetLoading)
        .add_systems(OnEnter(GameState::AssetLoading), (spawn_load_screen))
        .add_systems(OnExit(GameState::AssetLoading), (spawn_character, load_world, fit_camera_to_window, handle_level_camera_translations))
        // all systems for pre-level start
        .add_systems(OnEnter(GameState::AssetsLoaded), exit_load_screen)
        .add_systems(OnExit(GameState::AssetsLoaded), set_player_bounds.after(PreGameAssetLoadSet))
        // MainMenu systems
        .add_systems(OnEnter(GameState::MainMenu), (level_start, spawn_menu_world))
        .add_systems(Update, (fit_camera_to_window, handle_level_camera_translations).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), (despawn_character, despawn_world))
        // InGame systems
        .add_systems(OnEnter(GameState::InGame), (level_start))
        .add_systems(Update, (translate_camera_to_character, handle_level_camera_translations, player_movement, fit_camera_to_window).run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), (despawn_world, despawn_character))
        .add_systems(Startup, (setup))
        .add_systems(Update, (state_inputs))
        .run();
}
