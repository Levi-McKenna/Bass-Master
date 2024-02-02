use bevy::prelude::*;
use crate::GameState;

#[derive(Component)]
pub struct LoadScreenNodeBundle;

pub fn spawn_load_screen(
    mut commands: Commands
) {
    commands.spawn((
        NodeBundle {
            background_color: BackgroundColor(Color::GRAY),
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

pub fn exit_load_screen(
    mut commands: Commands,
    load_screen_query: Query<Entity, With<LoadScreenNodeBundle>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    load_screen_query.for_each(|load_screen_node_bundles| {
        commands.entity(load_screen_node_bundles).despawn_recursive();
    });
    change_game_state.set(GameState::InGame);
}

