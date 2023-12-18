use std::collections::HashMap;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::player::Player;
use crate::plugins::assets::GameAssets;
use crate::plugins::audio_manager::{PlaySFX, SFX};
use crate::plugins::camera_shake::{CameraImpact, CameraImpactStrength};
use crate::plugins::health::{self, Health};
use crate::plugins::menu::MenuGameConfig;
use crate::plugins::status_effect::{
    StatusEffect, StatusEffectEvent, StatusEffectEventType, StatusEffectType,
};
use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    enemy::Enemy,
    player::player_attach,
    GameAction,
};

use super::weapon_animation_effect::WeaponAnimationEffect;
use super::weapon_type::WeaponType;

const HAMMER_KNOCKBACK: f32 = 1000.;
const HAMMER_DAMAGE: f32 = 1.;
const HAMMER_HITBOX: f32 = 100.;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HammerStomp>();
        app.add_systems(
            Update,
            (
                hammer_controls,
                player_attach::attach_objects,
                on_hammer_stomp,
            )
                .run_if(in_state(AppState::GameRunning)),
        );
    }
}

#[derive(Debug, Event)]
pub struct HammerStomp {
    pub hitbox: f32,
    pub knockback: f32,
    pub translation: Vec3,
}

#[derive(Debug, Component)]
pub struct HammerController {
    pub hitbox: f32,
    pub stomp_time: f32,
    pub cooldown: f32,
    pub knockback: f32,
    pub is_stomping: bool,
}

fn spawn_hammer_effect(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    translation: Vec3,
) {
    let texture_atlas = WeaponAnimationEffect::HammerStomp.texture_atlas(&asset_server);
    let texture_atlas = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas,
                transform: Transform {
                    translation,
                    scale: Vec3::splat(3.5),
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 10.,
            last_animation: "Stomp".to_string(),
            current_animation: "Stomp".to_string(),
            animation_bank: create_hammer_effect_anim_hashmap(),
            destroy_on_end: true,
        });
}

fn on_hammer_stomp(
    mut hammer_stomp: EventReader<HammerStomp>,
    mut enemy_query: Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    mut ev_status: EventWriter<StatusEffectEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut impact_tx: EventWriter<CameraImpact>,
    mut sfx_tx: EventWriter<PlaySFX>,
) {
    for ev in hammer_stomp.iter() {
        spawn_hammer_effect(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            ev.translation,
        );

        let mut hit_count = 0;
        for (transform, ent) in enemy_query.iter_mut() {
            let distance = (transform.translation - ev.translation).length();
            if distance < ev.hitbox {
                hit_count += 1;
                let knockback = (transform.translation - ev.translation).normalize()
                    * ev.knockback
                    * (1. - distance / ev.hitbox);

                ev_status.send(StatusEffectEvent {
                    effect: StatusEffect {
                        effect_type: StatusEffectType::Knockback(knockback),
                        duration: 0.5,
                        current_duration: 0.5,
                    },
                    entity: ent,
                    event_type: StatusEffectEventType::Apply,
                });
            }
        }

        if hit_count > 0 {
            impact_tx.send(CameraImpact {
                strength: CameraImpactStrength::Medium,
            });
            sfx_tx.send(PlaySFX {
                sfx: SFX::AttackHammerHit,
                location: None,
            })
        } else {
            sfx_tx.send(PlaySFX {
                sfx: SFX::AttackHammerMiss,
                location: None,
            })
        }
    }
}

fn create_hammer_effect_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert(
        "Stomp".to_string(),
        animation::Animation {
            start: 1,
            end: 10,
            looping: false,
            cooldown: 0.1,
        },
    );
    hash_map
}

fn create_hammer_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert(
        "Idle".to_string(),
        animation::Animation {
            start: 1,
            end: 1,
            looping: true,
            cooldown: 0.1,
        },
    );
    hash_map.insert(
        "Stomp".to_string(),
        animation::Animation {
            start: 1,
            end: 3,
            looping: false,
            cooldown: 0.1,
        },
    );
    hash_map
}

pub fn spawn_hammer(
    commands: &mut Commands,
    game_config: &Res<MenuGameConfig>,
    game_assets: &Res<GameAssets>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets
                    .weapons
                    .get(&WeaponType::Hammer)
                    .unwrap()
                    .clone(),
                transform: Transform::from_scale(Vec3::splat(3.5)),
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 10.,
            last_animation: "Idle".to_string(),
            current_animation: "Idle".to_string(),
            animation_bank: create_hammer_anim_hashmap(),
            destroy_on_end: false,
        })
        .insert(player_attach::PlayerAttach::new(
            game_config.hero.weapon_offset(WeaponType::Hammer),
        ))
        .insert(HammerController {
            hitbox: HAMMER_HITBOX,
            stomp_time: 0.,
            cooldown: 0.,
            knockback: HAMMER_KNOCKBACK,
            is_stomping: false,
        })
        .insert(WeaponType::Hammer);
}

pub fn hammer_controls(
    mut hammer_query: Query<(&mut HammerController, &Transform, &mut Animator)>,
    actions: Query<&ActionState<GameAction>>,
    mut ev_stomp: EventWriter<HammerStomp>,
) {
    let action = actions.single();

    for (mut hammer, transform, mut animator) in hammer_query.iter_mut() {
        if hammer.cooldown > 0. {
            hammer.cooldown -= 0.1;
        }

        if hammer.stomp_time > 0. {
            animator.current_animation = "Stomp".to_string();
            hammer.stomp_time -= 0.15;
            hammer.is_stomping = true;

            if hammer.stomp_time <= 0. {
                ev_stomp.send(HammerStomp {
                    hitbox: hammer.hitbox,
                    knockback: hammer.knockback,
                    translation: transform.translation,
                });
            }
        } else {
            animator.current_animation = "Idle".to_string();
            hammer.is_stomping = false;
        }

        if hammer.stomp_time <= 0. && hammer.cooldown <= 0. {
            if action.pressed(GameAction::Action1) {
                hammer.stomp_time = 3.5;
                hammer.cooldown = 5.;
            }
        }
    }
}
