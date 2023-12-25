use crate::enemy::Enemy;
use crate::player::Player;
use crate::plugins::assets::GameAssets;
use crate::plugins::camera_shake::{CameraImpact, CameraImpactStrength};
use crate::plugins::gameplay_effects::{
    GameplayEffectPluginState, GameplayStat, GameplayStatsRecalculatedEvent,
};
use crate::plugins::health::{HealthUpdateEvent, TargetType};
use crate::state::{AppState, ForState};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

/// Pushes the whale above the screen when spawned, so that it appears to drop in
const WHALE_Y_OFFSET: f32 = 50.0;
/// The speed with which the whale will fall
const WHALE_MOVE_SPEED: f32 = 300.0;
/// How much to flatten the whale each tick after impact
const WHALE_FLATTEN_AMOUNT: f32 = 0.15;
/// The whale will not detonate before this much time passed
const WHALE_MIN_TIME_TO_BOOM: f32 = 0.1;
/// The whale will detonate after no more than this time
const WHALE_MAX_TIME_TO_BOOM: f32 = 2.5;

pub struct WhaleDumpPlugin;

impl Plugin for WhaleDumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(
                Update,
                (on_update, whale_move, whale_impact).run_if(in_state(AppState::GameRunning)),
            )
            .insert_resource(WhaleDumpPluginState::default());
    }
}

fn on_enter_game_running(mut whale_state: ResMut<WhaleDumpPluginState>) {
    *whale_state = Default::default();
}

fn on_update(
    time: Res<Time>,
    mut commands: Commands,
    mut whale_state: ResMut<WhaleDumpPluginState>,
    game_assets: Res<GameAssets>,
    mut rx_gameplay: EventReader<GameplayStatsRecalculatedEvent>,
    gameplay_state: Res<GameplayEffectPluginState>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // There was some recalculate event
    if rx_gameplay.iter().len() > 0 {
        whale_state.interval = gameplay_state
            .player_effects
            .get_stat(GameplayStat::WhaleInterval) as f32;
        whale_state.damage = gameplay_state
            .player_effects
            .get_stat(GameplayStat::WhaleDamage) as f32;
        whale_state.area = gameplay_state
            .player_effects
            .get_stat(GameplayStat::WhaleArea) as f32;
    }

    // Update the time since last spawn
    whale_state.time_last_spawn += time.delta_seconds();

    // Spawn if necessary
    if whale_state.time_last_spawn > whale_state.interval {
        whale_state.time_last_spawn = 0.0;
        spawn_whale(&mut commands, spawn_location(camera_query), &game_assets);
    }
}

/// Returns a random location to spawn the whale
fn spawn_location(camera_query: Query<(&Camera, &GlobalTransform)>) -> Option<Vec2> {
    // Project camera viewport to world space
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return None;
    };
    let Some(screen_width) = camera.logical_viewport_rect().map(|rect| rect.max.x) else {
        return None;
    };
    let Some(top_left) = camera.viewport_to_world_2d(camera_transform, Vec2::default()) else {
        return None;
    };

    let mut rng = thread_rng();

    Some(Vec2::new(
        top_left.x + rng.gen_range(0.0..=screen_width),
        top_left.y - WHALE_Y_OFFSET,
    ))
}
fn spawn_whale(commands: &mut Commands, location: Option<Vec2>, game_assets: &Res<GameAssets>) {
    let Some(location) = location else {
        return;
    };

    let mut rng = thread_rng();
    let mut spawn_transform = Transform::from_translation((location, 5.0).into());

    commands
        .spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: game_assets.whale.clone(),
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(Whale {
            time_till_boom: rng.gen_range(WHALE_MIN_TIME_TO_BOOM..WHALE_MAX_TIME_TO_BOOM),
        });
}

fn whale_move(
    time: Res<Time>,
    mut commands: Commands,
    mut whales: Query<(Entity, &mut Whale, &mut Transform), Without<WhaleFlattens>>,
) {
    let delta = time.delta_seconds();
    let move_by = WHALE_MOVE_SPEED * delta;

    for (entity, mut whale, mut transform) in whales.iter_mut() {
        whale.time_till_boom -= delta;
        transform.translation.y -= move_by;

        if whale.time_till_boom < 0.0 {
            commands.entity(entity).insert(WhaleFlattens::default());
        }
    }
}

fn whale_impact(
    mut commands: Commands,
    mut whales: Query<(Entity, &mut WhaleFlattens, &mut Transform), Without<Enemy>>,
    mut enemies: Query<(Entity, &Enemy, &Transform), Without<WhaleFlattens>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>, Without<WhaleFlattens>)>,
    mut tx_health: EventWriter<HealthUpdateEvent>,
    whale_state: Res<WhaleDumpPluginState>,
    mut tx_impact: EventWriter<CameraImpact>,
) {
    let Ok(player_loc) = player.get_single().map(|tf| tf.translation.truncate()) else {
        return;
    };

    for (entity, mut whale, mut whale_tf) in whales.iter_mut() {
        whale_tf.scale.y -= WHALE_FLATTEN_AMOUNT;

        // Not time to explode just yet
        if whale_tf.scale.y > 0.1 {
            continue;
        }

        // Create a shake based on distance to the player
        let dist_player = Vec2::distance(whale_tf.translation.truncate(), player_loc);
        tx_impact.send(CameraImpact {
            strength: CameraImpactStrength::strength_by_distance(dist_player),
        });

        // For each enemy, check if inside area of effect
        for (entity, enemy, enemy_tf) in enemies.iter_mut() {
            if Vec2::distance(
                whale_tf.translation.truncate(),
                enemy_tf.translation.truncate(),
            ) > whale_state.area
            {
                continue;
            }

            // Apply damage
            tx_health.send(HealthUpdateEvent {
                entity,
                health_change: -whale_state.damage,
                target_type: TargetType::Enemy(enemy.kind),
            });
        }

        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Default, Resource)]
struct WhaleDumpPluginState {
    time_last_spawn: f32,
    interval: f32,
    damage: f32,
    area: f32,
}

#[derive(Component)]
struct Whale {
    time_till_boom: f32,
}

#[derive(Default, Component)]
struct WhaleFlattens {}
