mod player_movement;
mod level_loader;
mod systems;

use bevy::prelude::*;
use bevy::render::*;
use bevy::render::settings::*;
use bevy::render::camera::*;
use bevy_ecs_ldtk::prelude::*;
use systems::*;
use player_movement::*;
use level_loader::*;
use menu::*;
use player_input::state_inputs;

enum GameState {
    MainMenu,
    InGame,
    Paused,
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
        .add_state(GameState::MainMenu)
        .add_systems(OnEnter(GameState::MainMenu), draw_menu_ui)
        .add_systems(OnEnter(GameState::InGame), (spawn_character, load_world))
        .add_systems(Update, (player_movement, fit_camera_to_level).run_if(in_state(GameState::InGame)))
        .add_systems(Startup, setup)
        .add_systems(Update, state_inputs)
        .run();
}
