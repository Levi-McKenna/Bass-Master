use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle};

#[derive(Component)]
pub struct ExitAnimation;

pub fn spawn_exit_animation(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position_x: f32,
    position_y: f32,
) {
    commands.spawn((AsepriteBundle {
        aseprite: asset_server.load("textures/animations/Portal-Out.aseprite"),
        animation: AsepriteAnimation::from("portal out"),
        transform: Transform {
            scale: Vec3::splat(0.2),
            translation: Vec3::new(position_x, position_y, 105.),
            ..default()
        },
        ..default()
    },
        ExitAnimation,
    ));
}

pub fn toggle_portal_animation(
    mut commands: Commands,
    mut animation_query: Query<(&mut AsepriteAnimation, Entity), With<ExitAnimation>>,
) {
    for (mut animation, entity) in animation_query.iter_mut() {
        animation.play();
        // 14 is the final frame of the animation
        if animation.current_frame() == 13 {
           commands.entity(entity).despawn_recursive(); 
        }
    }
}
