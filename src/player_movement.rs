use bevy::prelude::*;

// States to control the direction the player moves when jumping
#[derive(Default)]
pub enum Inverse {
    #[default]
    Up,
    Down
}

// States for player movement actions
#[derive(Default)]
pub enum MoveState {
    #[default]
    PlatformMove,
    JumpMove
}

// PLayer struct
#[derive(Component, Default)]
pub struct Bassist {
    movement: MoveState,
    inverse: Inverse
}

pub fn spawn_character(
        mut commands: Commands,
        asset_server: Res<AssetServer>
) {
    let character_texture = asset_server.load("textures/bass_master_sprite.png");
    // Spawn Bassist sprite at the center of the screen with a high Z-index
    commands.spawn((SpriteBundle {
        texture: character_texture,
        transform: Transform::from_xyz(500.0, 0.0, 100.0),
        ..default()
    },
    Bassist::default()
    ));
}

const PLAYER_SPEED: f32 = 1.0;

pub fn player_movement(
        mut character_query: Query<(&mut Transform, &mut Bassist)>,
        input: Res<Input<KeyCode>>,
        time: Res<Time>
) {
    for (mut transform, mut bassist) in character_query.iter_mut() {
        if (input.just_pressed(KeyCode::X)) {
            transform.translation += Vec3::new(30.0, 0.0, 0.0);
        }

        // match &bassist.movement {
        //     MoveState::PlatformMove => transform.translation += Vec3::new(1.0, 0.0, 0.0),
        //     MoveState::JumpMove => transform.translation += Vec3::new(PLAYER_SPEED, 20.0, 0.0),
        //     _ => panic!("!! Enum MoveState not set for Bassist in player_move query !!")
        // } 
    }
}
