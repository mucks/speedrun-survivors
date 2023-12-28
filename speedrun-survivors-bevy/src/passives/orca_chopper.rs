use crate::enemy::Enemy;
use crate::player::Player;
use crate::plugins::assets::GameAssets;
use crate::plugins::gameplay_effects::{
    GameplayEffectPluginState, GameplayStat, GameplayStatsRecalculatedEvent,
};
use crate::plugins::health::{HealthUpdateEvent, TargetType};
use crate::state::{for_game_states, AppState};
use bevy::prelude::*;
use rand::Rng;

pub struct OrcaChopperPlugin;

const ORCA_HIT_DISTANCE: f32 = 50.;

impl Plugin for OrcaChopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameInitializing), on_enter_game_init)
            .add_systems(
                Update,
                (on_update, orca_move, orca_attack).run_if(in_state(AppState::GameRunning)),
            )
            .insert_resource(OrcaChopperPluginState::default());
    }
}

fn on_enter_game_init(mut orca_state: ResMut<OrcaChopperPluginState>) {
    orca_state.total_spawned = 0;
}

fn on_update(
    mut commands: Commands,
    mut orca_state: ResMut<OrcaChopperPluginState>,
    player: Query<&Transform, With<Player>>,
    game_assets: Res<GameAssets>,
    mut rx_gameplay: EventReader<GameplayStatsRecalculatedEvent>,
    gameplay_state: Res<GameplayEffectPluginState>,
) {
    // There was some recalculate event
    if rx_gameplay.iter().len() < 1 {
        return;
    }

    // Make sure we got a player
    let Ok(player_location) = player.get_single() else {
        return;
    };

    // Get the number of total orcas we should have
    let expected = gameplay_state
        .player_effects
        .get_stat(GameplayStat::OrcaCount) as u32;

    // Update local stats from gameplay state
    orca_state.speed = gameplay_state
        .player_effects
        .get_stat(GameplayStat::OrcaSpeed) as f32;
    orca_state.damage = gameplay_state
        .player_effects
        .get_stat(GameplayStat::OrcaDamage) as f32;

    // No need to spawn none
    if expected <= 0 {
        return;
    }

    // Spawn in the required number of orcas
    for i in orca_state.total_spawned..expected {
        orca_state.total_spawned += 1;
        spawn_orca_chopper(&mut commands, &player_location.translation, &game_assets);
    }
}

/// Move and rotate each orca chopper
fn orca_move(
    time: Res<Time>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut orcas: Query<(&mut OrcaChopper, &mut Transform)>,
    orca_state: Res<OrcaChopperPluginState>,
) {
    // Project camera viewport to world space
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };
    let Some(viewport) = camera.logical_viewport_rect() else {
        return;
    };
    let Some(top_left) = camera.viewport_to_world_2d(camera_transform, Vec2::default()) else {
        return;
    };
    let Some(bottom_right) = camera.viewport_to_world_2d(camera_transform, viewport.max) else {
        return;
    };

    // Get the orca speed from the gameplay system
    let move_by = orca_state.speed * time.delta_seconds();

    for (mut orca, mut transform) in orcas.iter_mut() {
        // Distance to camera location
        let distance_before = transform
            .translation
            .distance(camera_transform.translation());

        // Move & rotate each orca
        transform.translation.x += orca.heading.cos() * move_by;
        transform.translation.y += orca.heading.sin() * move_by;
        transform.rotate(Quat::from_rotation_z(30.0 * time.delta_seconds()));

        // The orca might leave the screen by such a distance,
        // that every future tick would only flip flop the heading around
        // So we need to make sure it is getting closer
        // TODO this is not fully foolproof - might get stuck bouncing up and down with a permanent X location
        //  might have to add some angle towards center of screen
        let moving_away = distance_before
            < transform
                .translation
                .distance(camera_transform.translation());

        // Bounce off of the screen edge
        if moving_away {
            if transform.translation.x < top_left.x || transform.translation.x > bottom_right.x {
                orca.heading = std::f32::consts::PI - orca.heading;
            }
            if transform.translation.y > top_left.y || transform.translation.y < bottom_right.y {
                orca.heading = std::f32::consts::TAU - orca.heading;
            }
        }
    }
}

/// Deal damage to enemies in contact
fn orca_attack(
    mut orcas: Query<(&OrcaChopper, &Transform)>,
    mut enemies: Query<(Entity, &Enemy, &Transform)>,
    mut tx_health: EventWriter<HealthUpdateEvent>,
    orca_state: Res<OrcaChopperPluginState>,
) {
    for (orca, orca_transform) in orcas.iter() {
        for (entity, enemy, transform) in enemies.iter_mut() {
            if Vec2::distance(
                orca_transform.translation.truncate(),
                transform.translation.truncate(),
            ) <= ORCA_HIT_DISTANCE
            {
                tx_health.send(HealthUpdateEvent {
                    entity,
                    health_change: orca_state.damage,
                    target_type: TargetType::Enemy(enemy.kind),
                });
            }
        }
    }
}

/// Spawn a new orca chopper on top of the player
fn spawn_orca_chopper(
    commands: &mut Commands,
    player_location: &Vec3,
    game_assets: &Res<GameAssets>,
) {
    let mut rng = rand::thread_rng();

    let mut spawn_transform = Transform::from_translation(player_location.clone());

    commands
        .spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: game_assets.orca.clone(),
                ..Default::default()
            },
            for_game_states(),
        ))
        .insert(OrcaChopper {
            heading: rng.gen_range(0.0..std::f32::consts::TAU),
        });
}

#[derive(Default, Resource)]
struct OrcaChopperPluginState {
    total_spawned: u32,
    speed: f32,
    damage: f32,
}

#[derive(Component)]
pub struct OrcaChopper {
    heading: f32,
}
