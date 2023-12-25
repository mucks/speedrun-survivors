use crate::enemy::enemy_type::EnemyType;
use crate::player::Player;
use crate::plugins::health::{self, Health};
use crate::plugins::pickup::PickupEvent;
use crate::state::AppState;
use bevy::prelude::*;

pub mod enemy_spawner;
pub mod enemy_type;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
            .add_systems(
                Update,
                (process_events, update_enemies, update_enemy_hits)
                    .run_if(in_state(AppState::GameRunning)),
            )
            .add_event::<EnemyEvent>();
    }
}

fn on_enter_game_running(mut commands: Commands) {}
fn on_exit_game_running(mut commands: Commands) {}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub attack: f32,
    pub kind: EnemyType,
}

#[derive(Debug, Event)]
pub enum EnemyEvent {
    Spawned(EnemyType),
    Died(Entity, EnemyType),
    Ability1(EnemyType),
    Ability2(EnemyType),
}

pub fn process_events(
    mut commands: Commands,
    mut rx_enemy: EventReader<EnemyEvent>,
    mut tx_pickup: EventWriter<PickupEvent>,
    mut query_tf: Query<&Transform, With<Enemy>>,
) {
    for ev in rx_enemy.iter() {
        match ev {
            EnemyEvent::Died(entity, kind) => {
                let Ok(tf) = query_tf.get(*entity) else {
                    continue;
                };

                tx_pickup.send(PickupEvent::new(
                    kind.get_coin_reward(),
                    kind.get_exp_reward(),
                    tf.translation,
                ));
                commands
                    .get_entity(*entity)
                    .and_then(|entity| Some(entity.despawn_recursive()));
            }
            _ => {
                eprintln!("EnemyEvent message of type {ev:?} not implemented!");
            }
        }
    }
}

pub fn update_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &mut Transform, Entity, &Health), Without<Player>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (enemy, mut transform, entity, health) in enemy_query.iter_mut() {
        let moving = Vec3::normalize(player_transform.translation - transform.translation)
            * enemy.speed
            * time.delta_seconds();

        transform.translation += moving;
        transform.scale.x = moving.x.signum() * -f32::abs(transform.scale.x);
    }
}

pub struct EnemyInfo {
    pub translation: Vec2,
    pub entity: Entity,
    pub attack: f32,
}

pub fn update_enemy_hits(
    enemy_query: Query<(&Transform, Entity, &Enemy), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &mut Health, Entity), Without<Enemy>>,
    mut tx_health: EventWriter<health::HealthUpdateEvent>,
) {
    let mut enemy_list = Vec::new();
    for (transform, entity, enemy) in enemy_query.iter() {
        enemy_list.push(EnemyInfo {
            translation: Vec2::new(transform.translation.x, transform.translation.y),
            entity,
            attack: enemy.attack,
        });
    }

    for (transform, mut health, entity) in player_query.iter_mut() {
        for enemy in enemy_list.iter() {
            if Vec2::distance(enemy.translation, transform.translation.truncate()) <= 36. {
                tx_health.send(health::HealthUpdateEvent {
                    entity,
                    health_change: -enemy.attack,
                    target_type: health::TargetType::Player,
                });
            }
        }
    }
}
