use crate::player::{Player, PlayerEvent};
use crate::plugins::assets::GameAssets;
use crate::plugins::coin_rewards::CoinAccumulated;
use crate::plugins::gameplay_effects::{
    GameplayEffectPluginState, GameplayStat, GameplayStatsRecalculatedEvent,
};
use crate::state::{AppState, ForState};
use bevy::prelude::*;
use leafwing_input_manager::orientation::Orientation;
use rand::Rng;

const CONSUME_DISTANCE: f32 = 50.;
const PICKUP_MOVE_SPEED: f32 = 500.;

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(
                Update,
                (on_update, pickup_magnet, pickup_consume).run_if(in_state(AppState::GameRunning)),
            )
            .add_event::<PickupEvent>()
            .insert_resource(PickupPluginState::default());
    }
}

/// Reset plugin state to default values
fn on_enter_game_running(mut pickup_state: ResMut<PickupPluginState>) {
    pickup_state.attract_distance = CONSUME_DISTANCE;
}

/// Updates stats from the gameplay system
fn on_update(
    mut commands: Commands,
    mut pickup_state: ResMut<PickupPluginState>,
    game_assets: Res<GameAssets>,
    mut rx_gameplay: EventReader<GameplayStatsRecalculatedEvent>,
    gameplay_state: Res<GameplayEffectPluginState>,
    mut rx_pickup: EventReader<PickupEvent>,
) {
    // If there was some recalculate event, update stats
    if rx_gameplay.iter().len() > 0 {
        pickup_state.attract_distance = gameplay_state
            .player_effects
            .get_stat(GameplayStat::PickupDistance) as f32;
    }

    // Check for events
    for ev in rx_pickup.iter() {
        match ev {
            PickupEvent::SpawnCoinExpLoc(coin, exp, loc) => {
                spawn_pickup(&mut commands, *loc, &game_assets, PickupType::Exp(*exp));
                spawn_pickup(&mut commands, *loc, &game_assets, PickupType::Coin(*coin));
            }
        }
    }
}

/// This function handles moving pickups towards the player
fn pickup_magnet(
    time: Res<Time>,
    pickup_state: Res<PickupPluginState>,
    player: Query<&Transform, (With<Player>, Without<Pickup>)>,
    mut pickups: Query<&mut Transform, (With<Pickup>, Without<Player>)>,
) {
    // Make sure we got a player
    let Ok(player) = player.get_single().map(|player| player.translation) else {
        return;
    };

    let player_2d = player.truncate();
    for mut tf in pickups.iter_mut() {
        if Vec2::distance(tf.translation.truncate(), player_2d) > pickup_state.attract_distance {
            continue;
        }

        let moving =
            Vec3::normalize(player - tf.translation) * PICKUP_MOVE_SPEED * time.delta_seconds();

        tf.translation += moving;
    }
}

/// Once an item touches the player it shall be consumed
fn pickup_consume(
    mut commands: Commands,
    player: Query<&Transform, (With<Player>, Without<Pickup>)>,
    pickups: Query<(Entity, &Pickup, &Transform), (With<Pickup>, Without<Player>)>,
    mut tx_coin: EventWriter<CoinAccumulated>,
    mut tx_exp: EventWriter<PlayerEvent>,
) {
    // Make sure we got a player
    let Ok(player) = player.get_single().map(|tf| tf.translation.truncate()) else {
        return;
    };

    for (entity, pickup, tf) in pickups.iter() {
        if Vec2::distance(tf.translation.truncate(), player) > CONSUME_DISTANCE {
            continue;
        }

        // Send the proper message
        match pickup.kind {
            PickupType::Exp(exp) => {
                tx_exp.send(PlayerEvent::ExpGained(exp));
            }
            PickupType::Coin(coin) => {
                tx_coin.send(CoinAccumulated { coin });
            }
        }

        // Delete the entity
        commands
            .get_entity(entity)
            .and_then(|entity| Some(entity.despawn_recursive()));
    }
}

fn spawn_pickup(
    commands: &mut Commands,
    location: Vec3,
    game_assets: &Res<GameAssets>,
    kind: PickupType,
) {
    // Modify location with some randomness
    let mut rng = rand::thread_rng();
    let mut spawn_transform = Transform::from_translation(location);
    spawn_transform.translation.x += rng.gen_range(-20.0..20.0);
    spawn_transform.translation.y += rng.gen_range(-20.0..20.0);

    commands
        .spawn((
            SpriteBundle {
                transform: spawn_transform,
                texture: match kind {
                    PickupType::Exp(_) => game_assets.pickup_exp.clone(),
                    PickupType::Coin(_) => game_assets.pickup_coin.clone(),
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(Pickup { kind });
}

#[derive(Event)]
pub enum PickupEvent {
    SpawnCoinExpLoc(u64, u64, Vec3),
}

impl PickupEvent {
    pub fn new(coins: u64, exp: u64, loc: Vec3) -> Self {
        Self::SpawnCoinExpLoc(coins, exp, loc)
    }
}

#[derive(Default, Resource)]
struct PickupPluginState {
    attract_distance: f32,
}

#[derive(Component)]
pub struct Pickup {
    kind: PickupType,
}

#[derive(Clone)]
pub enum PickupType {
    Exp(u64),
    Coin(u64),
}

impl PickupType {
    fn new_coin(coins: u64) -> Self {
        PickupType::Coin(coins)
    }

    fn new_exp(exp: u64) -> Self {
        PickupType::Exp(exp)
    }
}
