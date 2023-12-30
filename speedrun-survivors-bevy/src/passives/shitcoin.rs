use crate::enemy::Enemy;
use crate::player::Player;
use crate::plugins::assets::GameAssets;
use crate::plugins::gameplay_effects::{
    GameplayEffectPluginState, GameplayStat, GameplayStatsRecalculatedEvent,
};
use crate::plugins::health::{HealthUpdateEvent, TargetType};
use crate::plugins::vfx_manager::{PlayVFX, VFX};
use crate::state::{for_game_states, AppState};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

/// The speed with which the cluster bomb flies
const CLUSTER_BOMB_MOVE_SPEED: f32 = 700.0;
/// Gravitational effect on cluster bomb
const CLUSTER_BOMB_GRAVITY_MULTIPLIER: f32 = 0.7;
/// The cluster bomb will not detonate before this much time passed
const CLUSTER_MIN_TIME_TO_BOOM: f32 = 0.18;
/// The cluster bomb will detonate after no more than this time
const CLUSTER_MAX_TIME_TO_BOOM: f32 = 1.0;

/// The scale for sub-munitions
const SUB_MUNITION_SCALE: f32 = 0.3;
/// The speed with which the sub-munitions fly
const SUB_MUNITION_MOVE_SPEED: f32 = 1400.0;
/// The sub-munition will not detonate before this much time passed
const SUB_MUNITION_MIN_TIME_TO_BOOM: f32 = 0.05;
/// The sub-munition will detonate after no more than this time
const SUB_MUNITION_MAX_TIME_TO_BOOM: f32 = 0.25;
/// The sub-munition will affect targets within this range
const SUB_MUNITION_BLAST_RADIUS: f32 = 70.0; //TODO maybe also a gameplay stat

pub struct ShitcoinClusterPlugin;

impl Plugin for ShitcoinClusterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameInitializing), on_enter_game_init)
            .add_systems(
                Update,
                (on_update, cluster_move, sub_munition_move)
                    .run_if(in_state(AppState::GameRunning)),
            )
            .add_systems(
                Update,
                on_stats_recalculated.run_if(on_event::<GameplayStatsRecalculatedEvent>()),
            )
            .insert_resource(ShitcoinClusterPluginState::default());
    }
}

/// Reset plugin state
fn on_enter_game_init(mut shitcoin_state: ResMut<ShitcoinClusterPluginState>) {
    *shitcoin_state = Default::default();
}

/// Update the plugin state to reflect changes in the gameplay system state
fn on_stats_recalculated(
    mut shitcoin_state: ResMut<ShitcoinClusterPluginState>,
    gameplay_state: Res<GameplayEffectPluginState>,
) {
    shitcoin_state.interval = gameplay_state
        .player_effects
        .get_stat(GameplayStat::ShitcoinInterval) as f32;
    shitcoin_state.munitions = gameplay_state
        .player_effects
        .get_stat(GameplayStat::ShitcoinMunitions) as u32;
    shitcoin_state.damage = gameplay_state
        .player_effects
        .get_stat(GameplayStat::ShitcoinDamage) as f32;
}

/// Update stats when required, spawn shitcoin cluster bombs
fn on_update(
    time: Res<Time>,
    mut commands: Commands,
    mut shitcoin_state: ResMut<ShitcoinClusterPluginState>,
    game_assets: Res<GameAssets>,
    player: Query<&Transform, With<Player>>,
) {
    // Make sure we got a player
    let Ok(player_loc) = player.get_single().map(|tf| tf.translation.truncate()) else {
        return;
    };

    // Update the time since last spawn
    shitcoin_state.time_last_spawn += time.delta_seconds();

    // Spawn a new cluster bomb
    if shitcoin_state.interval > 0.0 && shitcoin_state.time_last_spawn > shitcoin_state.interval {
        shitcoin_state.time_last_spawn = 0.0;
        spawn_cluster_bomb(&mut commands, player_loc, &game_assets);
    }
}

/// Spawn a shitcoin cluster bomb
fn spawn_cluster_bomb(commands: &mut Commands, player_loc: Vec2, game_assets: &Res<GameAssets>) {
    let mut rng = thread_rng();
    let spawn_transform = Transform::from_translation((player_loc, 5.0).into());

    commands
        .spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: game_assets.shitcoin.clone(),
                ..Default::default()
            },
            for_game_states(),
        ))
        .insert(ShitcoinClusterBomb {
            heading: rng.gen_range(0.0..std::f32::consts::TAU),
            time_till_boom: rng.gen_range(CLUSTER_MIN_TIME_TO_BOOM..CLUSTER_MAX_TIME_TO_BOOM),
        });
}

/// Move the shitcoin cluster bombs in an arch, update time until boom
fn cluster_move(
    time: Res<Time>,
    mut commands: Commands,
    mut cluster_bombs: Query<
        (Entity, &mut ShitcoinClusterBomb, &mut Transform),
        Without<ShitcoinSubMunition>,
    >,
    game_assets: Res<GameAssets>,
    shitcoin_state: Res<ShitcoinClusterPluginState>,
    mut tx_vfx: EventWriter<PlayVFX>,
) {
    let delta = time.delta_seconds();
    let move_by = CLUSTER_BOMB_MOVE_SPEED * delta;
    let target_dir = 1.5 * std::f32::consts::PI;
    let gravity_delta = delta * CLUSTER_BOMB_GRAVITY_MULTIPLIER;

    for (entity, mut cluster, mut transform) in cluster_bombs.iter_mut() {
        cluster.time_till_boom -= delta;

        if cluster.time_till_boom < 0.0 {
            // Unspawn the cluster bomb
            commands.entity(entity).despawn_recursive();

            // Spawn sub-munitions
            spawn_sub_munitions(
                &mut commands,
                transform.translation.truncate(),
                &game_assets,
                &shitcoin_state,
            );

            // Spawn tiny explosion VFX?
            tx_vfx.send(PlayVFX {
                vfx: VFX::ExplosionXL,
                location: transform.translation,
            });

            // No need to update location and heading
            return;
        }

        transform.translation.x += cluster.heading.cos() * move_by;
        transform.translation.y += cluster.heading.sin() * move_by;

        // Recalculate the heading to simulate gravity
        if cluster.heading > std::f32::consts::PI / 2.0 {
            cluster.heading += gravity_delta * (target_dir - cluster.heading);
        } else {
            cluster.heading += gravity_delta * (-target_dir - cluster.heading);
            cluster.heading =
                (cluster.heading + 2.0 * std::f32::consts::PI) % (2.0 * std::f32::consts::PI);
        }
    }
}

/// Spawn a sub-munitions
fn spawn_sub_munitions(
    commands: &mut Commands,
    cluster_loc: Vec2,
    game_assets: &Res<GameAssets>,
    shitcoin_state: &Res<ShitcoinClusterPluginState>,
) {
    let mut rng = thread_rng();
    let mut spawn_transform = Transform::from_translation((cluster_loc, 5.0).into());
    spawn_transform.scale *= SUB_MUNITION_SCALE;

    for _ in 0..shitcoin_state.munitions {
        commands
            .spawn((
                SpriteBundle {
                    transform: spawn_transform,
                    texture: game_assets.shitcoin.clone(),
                    ..Default::default()
                },
                for_game_states(),
            ))
            .insert(ShitcoinSubMunition {
                heading: rng.gen_range(0.0..std::f32::consts::TAU),
                time_till_boom: rng
                    .gen_range(SUB_MUNITION_MIN_TIME_TO_BOOM..SUB_MUNITION_MAX_TIME_TO_BOOM),
            });
    }
}

/// Move the sub-munitions in a straight line and update time until boom
fn sub_munition_move(
    time: Res<Time>,
    mut commands: Commands,
    mut sub_munitions: Query<
        (Entity, &mut ShitcoinSubMunition, &mut Transform),
        (Without<ShitcoinClusterBomb>, Without<Enemy>),
    >,
    shitcoin_state: Res<ShitcoinClusterPluginState>,
    mut enemies: Query<
        (Entity, &Enemy, &Transform),
        (Without<ShitcoinClusterBomb>, Without<ShitcoinSubMunition>),
    >,
    mut tx_health: EventWriter<HealthUpdateEvent>,
    mut tx_vfx: EventWriter<PlayVFX>,
) {
    let delta = time.delta_seconds();
    let move_by = SUB_MUNITION_MOVE_SPEED * delta;

    for (entity, mut munition, mut munition_tf) in sub_munitions.iter_mut() {
        munition.time_till_boom -= delta;

        if munition.time_till_boom < 0.0 {
            // Unspawn the sub-munition
            commands.entity(entity).despawn_recursive();

            let munition_loc = munition_tf.translation.truncate();

            // Check for enemies in range
            for (entity, enemy, enemy_tf) in enemies.iter_mut() {
                if Vec2::distance(munition_loc, enemy_tf.translation.truncate())
                    > SUB_MUNITION_BLAST_RADIUS
                {
                    continue;
                }

                // Apply damage
                tx_health.send(HealthUpdateEvent {
                    entity,
                    health_change: -shitcoin_state.damage,
                    target_type: TargetType::Enemy(enemy.kind),
                });
            }

            // Spawn tiny explosion VFX
            tx_vfx.send(PlayVFX {
                vfx: VFX::ExplosionXS,
                location: munition_tf.translation,
            });

            // No need to update location
            return;
        }

        munition_tf.translation.x += munition.heading.cos() * move_by;
        munition_tf.translation.y += munition.heading.sin() * move_by;
    }
}

/// Shitcoin Cluster Plugin State
#[derive(Default, Resource)]
struct ShitcoinClusterPluginState {
    time_last_spawn: f32,
    interval: f32,
    munitions: u32,
    damage: f32,
}

/// State for a shitcoin cluster bomb
#[derive(Component)]
struct ShitcoinClusterBomb {
    heading: f32,
    time_till_boom: f32,
}

/// State for a shitcoin sub-munition
#[derive(Component)]
struct ShitcoinSubMunition {
    heading: f32,
    time_till_boom: f32,
}
