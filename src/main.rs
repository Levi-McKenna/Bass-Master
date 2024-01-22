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
        .add_systems(OnEnter(GameState::MainMenu), (draw_menu_ui, spawn_menu_world))
        .add_systems(OnExit(GameState::MainMenu), despawn_world)
        .add_systems(OnEnter(GameState::InGame), (spawn_character, load_world))
        .add_systems(OnExit(GameState::InGame), (despawn_world, despawn_character))
        // TODO: Manage the load screen state for when all setup is finished. (Possibly insert a
        // resource that is a certain value when all are finished)
/*         .add_systems(OnEnter(GameState::LoadScreen), (setup_load_screen, spawn_character, load_world)) */
        .add_systems(Update, (player_movement, fit_camera_to_level_player).run_if(in_state(GameState::InGame)))
        .add_systems(Startup, (setup))
        .add_systems(Update, state_inputs)
        .run();
}
