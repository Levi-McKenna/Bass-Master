use bevy::prelude::*;
use crate::LevelScore;

#[derive(Component)]
pub struct ScoreUI;

pub fn spawn_score(
    mut commands: Commands,
) {
    // scorespawn
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        },
        ScoreUI,
    )).with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "0",
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

pub fn update_score(
    mut score_text: Query<&mut Text, With<Label>>,
    score: Res<LevelScore>,
) {
    let mut sections = score_text.single_mut();
    for section in sections.sections.iter_mut() {
        section.value = score.0.to_string();
    }
}

pub fn despawn_score(
    mut commands: Commands,
    score_query: Query<Entity, With<ScoreUI>>,
) {
    for score in score_query.iter() {
        commands.entity(score).despawn_recursive();
    }
}
