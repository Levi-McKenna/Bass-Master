// TODO: Manage load screen states
mod player;
mod level;
mod input;
mod menu;
mod load_screen;

use bevy::prelude::*;
use bevy::render::*;
use bevy::render::settings::*;
use bevy::render::camera::*;
use bevy_ecs_ldtk::prelude::*;
use player::*;
use level::*;
use menu::*;
use input::state_inputs;
use load_screen::*;

// States for game status
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    LoadScreen,
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
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }
        }))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        // main menu state management
        .add_state::<GameState>()
        .add_state::<LevelState>()
        // LoadScreen systems
        .add_systems(OnEnter(GameState::LoadScreen), (spawn_load_screen, load_world, spawn_character))
        .add_systems(OnExit(GameState::LoadScreen), (despawn_load_screen, load_finished_state_change))
        // MainMenu systems
        .add_systems(OnEnter(GameState::MainMenu), (spawn_menu_world, spawn_character))
        .add_systems(Update, (fit_camera_to_window, handle_level_camera_translations).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), (despawn_character, despawn_world))
        // InGame systems
/*         .add_systems(OnEnter(GameState::InGame), ()) */
        .add_systems(Update, (player_movement, handle_level_camera_translations, fit_camera_to_window).run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), (despawn_world, despawn_character))
        .add_systems(Startup, (setup))
        .add_systems(Update, (state_inputs))
        .run();
}
