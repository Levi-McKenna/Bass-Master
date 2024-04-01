use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::{GameState};

#[derive(Component)]
pub struct LoadScreenNodeBundle;

pub fn spawn_load_screen(
    mut commands: Commands
) {
    commands.spawn((
        NodeBundle {
            background_color: BackgroundColor(Color::rgba(0., 0.011764705882352941, 0.18823529411764706, 1.)),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        LoadScreenNodeBundle,
    )).with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Loading Assets...",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::rgba(0., 0.9098039215686274, 1., 1.),
                        ..default()
                    }
                ),
                ..default()
            }.with_style(Style {
                 ..default()   
            }),
            Label));
    });
}

// honestly a little verbose and not needed but i guess it's nice to have just in case
pub fn exit_load_screen(
    mut commands: Commands,
    load_screen_query: Query<Entity, With<LoadScreenNodeBundle>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    level_query: Query<&Handle<LdtkLevel>>,
    mut level_event: EventReader<LevelEvent>,
) {
    for level_handle in level_query.iter() {
        // get level iid
        let level = ldtk_levels.get(level_handle);
        let level_iid = &level.expect("No Level With An IID").level.iid;
        // check for if level has spawned
        for event in level_event.iter() {
            if event == &LevelEvent::Transformed(level_iid.to_string()) {
                load_screen_query.for_each(|load_screen_node_bundles| {
                    commands.entity(load_screen_node_bundles).despawn_recursive();
                });
                change_game_state.set(GameState::InGame);
            }
        }
    }
}

