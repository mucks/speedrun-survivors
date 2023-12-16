use bevy::prelude::*;

use crate::player::PlayerMovement;
use crate::plugins::health::{self, Health};
use crate::state::AppState;

pub mod enemy_spawner;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
            .add_systems(
                Update,
                (update_enemies, update_enemy_hits).run_if(in_state(AppState::GameRunning)),
            );
    }
}

fn on_enter_game_running(mut commands: Commands) {}
fn on_exit_game_running(mut commands: Commands) {}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub attack: f32,
}

pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &mut Transform, Entity, &Health), Without<PlayerMovement>>,
    player_query: Query<(&PlayerMovement, &Transform), Without<Enemy>>,
    mut commands: Commands,
) {
    if let Ok((_player_movement, _player_transform)) = player_query.get_single() {
        for (enemy, mut transform, entity, health) in enemy_query.iter_mut() {
            let moving = Vec3::normalize(_player_transform.translation - transform.translation)
                * enemy.speed
                * time.delta_seconds();

            transform.translation += moving;
        }
    }
}

pub struct EnemyInfo {
    pub translation: Vec2,
    pub entity: Entity,
    pub attack: f32,
}

pub fn update_enemy_hits(
    enemy_query: Query<(&Transform, Entity, &Enemy), (With<Enemy>, Without<PlayerMovement>)>,
    mut player_query: Query<
        (&mut PlayerMovement, &mut Transform, &mut Health, Entity),
        Without<Enemy>,
    >,
    mut ev_health_change: EventWriter<health::HealthChangeEvent>,
) {
    let mut enemy_list = Vec::new();
    for (transform, entity, enemy) in enemy_query.iter() {
        enemy_list.push(EnemyInfo {
            translation: Vec2::new(transform.translation.x, transform.translation.y),
            entity,
            attack: enemy.attack,
        });
    }

    for (mut _player, transform, mut health, ent) in player_query.iter_mut() {
        for enemy in enemy_list.iter() {
            if Vec2::distance(
                enemy.translation,
                Vec2::new(transform.translation.x, transform.translation.y),
            ) <= 36.
            {
                ev_health_change.send(health::HealthChangeEvent {
                    entity: ent,
                    health_change: -enemy.attack,
                    target_type: health::HealthChangeTargetType::Player,
                });
            }
        }
    }
}
