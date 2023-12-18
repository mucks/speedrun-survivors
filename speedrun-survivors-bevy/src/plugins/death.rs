use bevy::prelude::*;

use crate::{
    player::{player_attach, Player},
    state::{AppState, ForState},
    weapon::hammer::HammerStomp,
};

use super::{
    assets::GameAssets,
    coin_rewards::CoinAccumulated,
    health::{ActiveHealthReachedZeroEvent, Health, HealthChangeEvent, HealthChangeTargetType},
    status_effect::{StatusEffect, StatusEffectController, StatusEffectType},
};

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActiveHealthReachedZeroEvent>();
        app.add_systems(Update, on_health_reached_zero);
    }
}

fn spawn_skull_on_player(commands: &mut Commands, position: Vec2, game_assets: &Res<GameAssets>) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets.skull.clone(),
                transform: Transform {
                    scale: Vec3::splat(0.5),
                    translation: Vec3::new(position.x, position.y, 5.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(player_attach::PlayerAttach::new(Vec2::ZERO));
}

fn on_health_reached_zero(
    mut ev: EventReader<ActiveHealthReachedZeroEvent>,
    mut commands: Commands,
    mut status_query: Query<(&mut StatusEffectController, &Transform, &mut Health), With<Player>>,
    mut event_stream: EventWriter<CoinAccumulated>,
    mut next_state: ResMut<NextState<AppState>>,
    mut ev_stomp: EventWriter<HammerStomp>,
    game_assets: Res<GameAssets>,
) {
    for ev in ev.iter() {
        if ev.target_type == HealthChangeTargetType::Player {
            let Ok((mut status_cont, player_tr, mut player_health)) =
                status_query.get_mut(ev.entity)
            else {
                return;
            };

            // Player died but has death is temporary status effect
            if status_cont
                .effects
                .iter()
                .find(|effect| effect.effect_type == StatusEffectType::DeathIsTemporary)
                .is_some()
            {
                status_cont.effects = vec![];
                next_state.set(AppState::GameOver);
                return; // Unspawn will happen due to state change
            } else {
                player_health.active_health = player_health.max_health / 2.;

                ev_stomp.send(HammerStomp {
                    hitbox: 200.,
                    knockback: 2000.,
                    translation: player_tr.translation,
                });

                status_cont.effects.push(StatusEffect {
                    effect_type: StatusEffectType::DeathIsTemporary,
                    duration: 10.,
                    current_duration: 10.,
                });
                spawn_skull_on_player(
                    &mut commands,
                    Vec2::new(player_tr.translation.x, player_tr.translation.y),
                    &game_assets,
                );
            }
        }

        if ev.target_type == HealthChangeTargetType::Enemy {
            event_stream.send(CoinAccumulated { coin: 100 });
            commands.entity(ev.entity).despawn_recursive(); // TODO PANICK thread 'Compute Task Pool (3)' panicked at speedrun-survivors-bevy/src/plugins/death.rs:95:22
                                                            // TODO Attempting to create an EntityCommands for entity 1731v0, which doesn't exist.
        }
    }
}
