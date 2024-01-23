mod player;
mod level;
mod input;
mod menu;
mod load;

use bevy::prelude::*;
use bevy::render::*;
use bevy::render::settings::*;
use bevy::render::camera::*;
use bevy_ecs_ldtk::prelude::*;
use player::*;
use level::*;
use menu::*;
use input::state_inputs;
use load::*;

// States for game status
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    FinishedGame,
    Paused,
    LoadScreen,
}

#[derive(Component)]
pub struct WorldCamera;

#[derive(Resource)]
pub struct LevelFinished(bool);

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
        .insert_resource(LevelFinished(false))
        .add_systems(OnEnter(GameState::MainMenu), (draw_menu_ui, spawn_menu_world, fit_camera_to_level))
        .add_systems(OnExit(GameState::MainMenu), despawn_world)
        .add_systems(OnEnter(GameState::InGame), (spawn_character, load_world))
        .add_systems(OnExit(GameState::InGame), (despawn_world, despawn_character))
        .add_systems(Update, (player_movement, handle_camera_translations, fit_camera_to_level).run_if(in_state(GameState::InGame)))
        .add_systems(Startup, (setup))
        .add_systems(Update, (state_inputs))
        .run();
}
