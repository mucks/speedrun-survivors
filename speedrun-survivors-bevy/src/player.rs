use bevy::prelude::*;

use crate::{
    animation::Animator, cursor_info::OffsetedCursorPosition, gun::GunController,
    player_attach::PlayerAttach, sword::SwordController,
};

#[derive(Debug, Clone, Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Animator)>,
    mut gun_query: Query<
        &mut TextureAtlasSprite,
        (
            With<GunController>,
            With<SwordController>,
            Without<PlayerMovement>,
        ),
    >,
    cursor_res: ResMut<OffsetedCursorPosition>,
) {
    for (player_movement, mut transform, mut animator) in query.iter_mut() {
        animator.current_animation = "Idle".to_string();

        // supports WASD & DVORAK
        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Comma) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y += player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::O) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y -= player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            animator.current_animation = "Walk".to_string();
            transform.translation.x -= player_movement.speed * time.delta_seconds();
            // turn the sprite around if moving left
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::E) {
            animator.current_animation = "Walk".to_string();
            transform.translation.x += player_movement.speed * time.delta_seconds();
            transform.rotation = Quat::default();
        }

        for mut sprite in gun_query.iter_mut() {
            if cursor_res.x - transform.translation.x >= 0. {
                sprite.flip_y = false;
            } else {
                sprite.flip_y = true;
            }
        }
    }
}
