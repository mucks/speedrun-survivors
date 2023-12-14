use bevy::prelude::*;

use crate::player::PlayerMovement;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &mut Transform, Entity), Without<PlayerMovement>>,
    player_query: Query<(&PlayerMovement, &Transform), Without<Enemy>>,
    mut commands: Commands,
) {
    if let Ok((_player_movement, _player_transform)) = player_query.get_single() {
        for (enemy, mut transform, entitiy) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(_player_transform.translation - transform.translation)
                * enemy.speed
                * time.delta_seconds();

            transform.translation += moving;

            if enemy.health <= 0. {
                commands.entity(entitiy).despawn();
            }
        }
    }
}
