use crate::enemy::Enemy;
use crate::player::Player;
use crate::plugins::assets::GameAssets;
use crate::plugins::gameplay_effects::GameplayEffectPluginState;
use crate::plugins::health::{HealthUpdateEvent, TargetType};
use crate::state::{AppState, ForState};
use bevy::prelude::*;
use rand::Rng;

pub struct OrcaChopperPlugin;

const ORCA_HIT_DISTANCE: f32 = 50.;

impl Plugin for OrcaChopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(
                Update,
                (on_update, orca_move, orca_attack).run_if(in_state(AppState::GameRunning)),
            )
            .insert_resource(OrcaChopperPluginState::default());
    }
}

fn on_enter_game_running(mut orca_state: ResMut<OrcaChopperPluginState>) {
    orca_state.total_spawned = 0;
}

fn on_update(
    mut commands: Commands,
    mut orca_state: ResMut<OrcaChopperPluginState>,
    player: Query<&Transform, With<Player>>,
    game_assets: Res<GameAssets>,
) {
    let Ok(player_location) = player.get_single() else {
        return;
    };

    // Check if we need to spawn more orcas
    // TODO this would be better with messages
    //  not sure how to do this atm - some SpawnOrcas(total_required) - compare against OrcaChopperPluginState::total_spawned
    //  could be send by gameplay effect system if certain stats ares update
    if orca_state.total_spawned < 1
    /*TODO gameplay_state.player_effects.orca_chopper_num*/
    {
        orca_state.total_spawned += 1;
        spawn_orca_chopper(&mut commands, &player_location.translation, &game_assets);
    }

    // TODO gameplay stats:
    //  - number
    //  - speed
    //  - damage

    // TODO orca hacks deal damage when touching enemies
}

/// Move and rotate each orca chopper
fn orca_move(
    time: Res<Time>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut orcas: Query<(&mut OrcaChopper, &mut Transform)>,
    gameplay_state: Res<GameplayEffectPluginState>,
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

    // Get the orca speed from the gameplay system TODO
    let move_by = 400.0 * time.delta_seconds();

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

fn orca_attack(
    mut orcas: Query<(&OrcaChopper, &Transform)>,
    mut enemies: Query<(Entity, &Enemy, &Transform)>,
    mut tx_health: EventWriter<HealthUpdateEvent>,
) {
    for (orca, orca_transform) in orcas.iter() {
        for (entity, enemy, transform) in enemies.iter_mut() {
            if Vec2::distance(
                Vec2::new(orca_transform.translation.x, orca_transform.translation.y),
                Vec2::new(transform.translation.x, transform.translation.y),
            ) <= ORCA_HIT_DISTANCE
            {
                tx_health.send(HealthUpdateEvent {
                    entity,
                    health_change: -0.2, //TODO get from gameplay system
                    target_type: TargetType::Enemy(enemy.kind),
                });
            }
        }
    }
}

/// Add a new orca chopper on the player
fn spawn_orca_chopper(
    commands: &mut Commands,
    player_location: &Vec3,
    game_assets: &Res<GameAssets>,
) {
    // Spawn a new orca on top of the player with a random heading
    let mut rng = rand::thread_rng();

    let mut spawn_transform = Transform::from_translation(player_location.clone());

    commands
        .spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: game_assets.orca.clone(),
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(OrcaChopper {
            heading: rng.gen_range(0.0..std::f32::consts::TAU),
        });
}

#[derive(Default, Resource)]
struct OrcaChopperPluginState {
    total_spawned: u32,
}

#[derive(Component)]
pub struct OrcaChopper {
    heading: f32,
}
