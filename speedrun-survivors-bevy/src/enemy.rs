use bevy::prelude::*;

use crate::state::AppState;
use crate::{health::Health, player::PlayerMovement};

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
}

pub fn update_enemy_hits(
    enemy_query: Query<(&Transform, Entity), (With<Enemy>, Without<PlayerMovement>)>,
    mut player_query: Query<(&mut PlayerMovement, &mut Transform, &mut Health), Without<Enemy>>,
    mut commands: Commands,
) {
    let mut enemy_list = Vec::new();
    for (transform, entity) in enemy_query.iter() {
        enemy_list.push(EnemyInfo {
            translation: Vec2::new(transform.translation.x, transform.translation.y),
            entity,
        });
    }

    for (mut player, transform, mut health) in player_query.iter_mut() {
        for enemy in enemy_list.iter() {
            if Vec2::distance(
                enemy.translation,
                Vec2::new(transform.translation.x, transform.translation.y),
            ) <= 36.
            {
                health.active_health -= 1.;
            }
        }
    }
}
