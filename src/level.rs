use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::{WorldCamera, Bassist, GameState};

// Marker Component
#[derive(Component)]
pub struct CurrentLevel();

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    #[default]
    OutOfLevel,
    Introduction,
    Playing,
    Ending
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "levels/built_in/Bass_Master_Level_Test.ldtk")]
    handle: Handle<LdtkAsset>
}

pub fn load_world(
    mut commands: Commands,
    level_collection: Res<LevelAssets>,
) {
    commands.spawn((LdtkWorldBundle {
        ldtk_handle: level_collection.handle.clone(),
        ..default()
    },
        CurrentLevel {}
    ));
}

pub fn despawn_world(
    mut commands: Commands,
    level_query: Query<Entity, (With<Handle<LdtkAsset>>)>,
) {
    level_query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}


pub fn level_start(
    game_state: Res<State<GameState>>,
    mut change_level_state: ResMut<NextState<LevelState>>,
) {
    if game_state.get() == &GameState::InGame {
        change_level_state.set(LevelState::Introduction);
    }
}
