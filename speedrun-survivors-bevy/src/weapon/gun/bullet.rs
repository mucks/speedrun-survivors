use bevy::prelude::*;

use crate::{enemy::Enemy, plugins::health};

#[derive(Component)]
pub struct Bullet {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
}

pub fn update_bullets(
    mut bullet_query: Query<(&mut Bullet, &mut Transform, Entity)>,
    time: ResMut<Time>,
    mut commands: Commands,
) {
    for (mut bullet, mut transform, entity) in bullet_query.iter_mut() {
        bullet.lifetime -= time.delta_seconds();
        let moving = bullet.speed * bullet.direction * time.delta_seconds();
        transform.translation += Vec3::new(moving.x, moving.y, 0.);
        if bullet.lifetime <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletInfo {
    pub translation: Vec2,
    pub entity: Entity,
}

pub fn update_bullet_hits(
    bullet_query: Query<(&Transform, Entity), (With<Bullet>, Without<Enemy>)>,
    mut enemy_query: Query<(&Enemy, &mut Transform, &health::Health, Entity), Without<Bullet>>,
    mut commands: Commands,
    mut tx_health: EventWriter<health::HealthUpdateEvent>,
) {
    let mut bullet_list = Vec::new();
    for (transform, entity) in bullet_query.iter() {
        bullet_list.push(BulletInfo {
            translation: Vec2::new(transform.translation.x, transform.translation.y),
            entity,
        });
    }
    for (enemy, transform, mut health, ent) in enemy_query.iter_mut() {
        bullet_list.retain(|bullet| {
            let distance = Vec2::distance(
                bullet.translation,
                Vec2::new(transform.translation.x, transform.translation.y),
            );
            if distance <= 36. {
                tx_health.send(health::HealthUpdateEvent {
                    entity: ent,
                    health_change: -1.,
                    target_type: health::TargetType::Enemy(enemy.kind),
                });

                commands.entity(bullet.entity).despawn();
                false
            } else {
                true
            }
        });
    }
}
