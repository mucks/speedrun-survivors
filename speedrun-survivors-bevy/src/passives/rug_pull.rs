use crate::enemy::Enemy;
use crate::player::Player;
use crate::plugins::assets::GameAssets;
use crate::plugins::gameplay_effects::{
    GameplayEffectPluginState, GameplayStat, GameplayStatsRecalculatedEvent,
};
use crate::state::{for_game_states, AppState};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

/// The maximum distance between a rug and an enemy that will get pulled
const RUG_GRAB_RANGE: f32 = 50.0;

pub struct RugPullPlugin;

impl Plugin for RugPullPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameInitializing), on_enter_game_init)
            .add_systems(
                Update,
                (on_update, rug_move, rug_pull_enemies).run_if(in_state(AppState::GameRunning)),
            )
            .insert_resource(RugPullPluginState::default());
    }
}

fn on_enter_game_init(mut rug_state: ResMut<RugPullPluginState>) {
    *rug_state = Default::default();
}

/// Update stats when required, spawn rugs
fn on_update(
    time: Res<Time>,
    mut commands: Commands,
    mut rug_state: ResMut<RugPullPluginState>,
    game_assets: Res<GameAssets>,
    mut rx_gameplay: EventReader<GameplayStatsRecalculatedEvent>,
    gameplay_state: Res<GameplayEffectPluginState>,
    player: Query<&Transform, With<Player>>,
) {
    // There was some recalculate event
    if rx_gameplay.iter().len() > 0 {
        rug_state.interval = gameplay_state
            .player_effects
            .get_stat(GameplayStat::RugPullInterval) as f32;
        rug_state.speed = gameplay_state
            .player_effects
            .get_stat(GameplayStat::RugPullSpeed) as f32;
        rug_state.max_ttl = gameplay_state
            .player_effects
            .get_stat(GameplayStat::RugPullTTL) as f32;

        //TODO see note below rug_state.damage = 0.1;
    }

    // Make sure we got a player
    let Ok(player_loc) = player.get_single().map(|tf| tf.translation.truncate()) else {
        return;
    };

    // Update the time since last spawn
    rug_state.time_last_spawn += time.delta_seconds();

    // Spawn a new rug
    if rug_state.time_last_spawn > rug_state.interval {
        rug_state.time_last_spawn = 0.0;
        rug_state.rug_count += 1;
        spawn_rug(
            &mut commands,
            player_loc,
            &game_assets,
            rug_state.rug_count,
            rug_state.max_ttl,
        );
    }
}

/// Spawn a rug heading into a random direction
fn spawn_rug(
    commands: &mut Commands,
    player_loc: Vec2,
    game_assets: &Res<GameAssets>,
    id: u64,
    max_ttl: f32,
) {
    let mut rng = thread_rng();
    let mut spawn_transform = Transform::from_translation((player_loc, 1.0).into());
    let heading = rng.gen_range(0.0..std::f32::consts::TAU);
    let ttl = rng.gen_range(max_ttl / 2.0..max_ttl);
    spawn_transform.rotation = Quat::from_rotation_z(heading);

    commands
        .spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: game_assets.rug.clone(),
                ..Default::default()
            },
            for_game_states(),
        ))
        .insert(Rug { id, ttl, heading });
}

fn rug_move(
    time: Res<Time>,
    mut commands: Commands,
    mut rugs: Query<(Entity, &mut Rug, &mut Transform), Without<Enemy>>,
    rug_state: Res<RugPullPluginState>,
    mut enemies: Query<(Entity, &Enemy, &Transform), (Without<Rug>, Without<RugPulled>)>,
    mut rug_pulled: Query<
        (Entity, &Enemy, &Transform, &RugPulled),
        (Without<Rug>, With<RugPulled>),
    >,
) {
    let delta = time.delta_seconds();
    let move_by = rug_state.speed * delta;

    for (entity, mut rug, mut rug_tf) in rugs.iter_mut() {
        // Reduce time to live
        rug.ttl -= delta;

        // Current location
        let rug_loc = rug_tf.translation.truncate();

        // Handle deletion
        if rug.ttl < 0.0 {
            commands.entity(entity).despawn_recursive();

            for (entity, mut enemy, enemy_tf, rug_pulled) in rug_pulled.iter_mut() {
                if rug.id == rug_pulled.rug_id {
                    commands.entity(entity).remove::<RugPulled>();
                }
            }

            return;
        }

        // Add the RugPulled component to enemies in range
        for (entity, enemy, enemy_tf) in enemies.iter_mut() {
            if Vec2::distance(rug_loc, enemy_tf.translation.truncate()) > RUG_GRAB_RANGE {
                continue;
            }

            // Need to be careful here as enemy might be un-spawning
            if commands.get_entity(entity).is_some() {
                commands
                    .entity(entity)
                    .insert(RugPulled::new(rug.heading, rug.id));
            }
        }

        // Update location
        rug_tf.translation.x += rug.heading.cos() * move_by;
        rug_tf.translation.y += rug.heading.sin() * move_by;
    }
}

/// Move enemies affected by the rug pull ability
fn rug_pull_enemies(
    time: Res<Time>,
    rug_state: Res<RugPullPluginState>,
    mut enemies: Query<(&mut Transform, &RugPulled), (With<Enemy>, Without<Rug>)>,
    // mut tx_health: EventWriter<HealthUpdateEvent>,
) {
    let delta = time.delta_seconds();
    let move_by = rug_state.speed * delta;
    // let delta_damage = delta * rug_state.damage;

    for (mut enemy_tf, rug_pulled) in enemies.iter_mut() {
        enemy_tf.translation.x += rug_pulled.heading.cos() * move_by;
        enemy_tf.translation.y += rug_pulled.heading.sin() * move_by;

        // Apply damage over time TODO: since the damage is a DoT, it applies every tick - which is bad :) Should rework logic to impact damage or so
        // tx_health.send(HealthUpdateEvent {
        //     entity,
        //     health_change: -delta_damage,
        //     target_type: TargetType::Enemy(enemy.kind),
        // });
    }
}

/// Rug Pull Plugin State
#[derive(Default, Resource)]
struct RugPullPluginState {
    time_last_spawn: f32,
    interval: f32,
    speed: f32,
    damage: f32,
    max_ttl: f32,
    rug_count: u64,
}

/// Rug component
#[derive(Component)]
struct Rug {
    id: u64,
    ttl: f32,
    heading: f32,
}

#[derive(Component)]
pub struct RugPulled {
    pub heading: f32,
    pub rug_id: u64,
}

impl RugPulled {
    fn new(heading: f32, rug_id: u64) -> Self {
        Self { heading, rug_id }
    }
}
