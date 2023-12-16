use std::collections::HashMap;

use bevy::prelude::*;

use crate::player::Player;
use crate::plugins::health::{self, Health};
use crate::plugins::status_effect::{
    StatusEffect, StatusEffectEvent, StatusEffectEventType, StatusEffectType,
};
use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    enemy::Enemy,
    player::player_attach,
};

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

fn on_hammer_stomp(
    mut hammer_stomp: EventReader<HammerStomp>,
    mut enemy_query: Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    mut ev_status: EventWriter<StatusEffectEvent>,
) {
    for ev in hammer_stomp.iter() {
        for (transform, ent) in enemy_query.iter_mut() {
            let distance = (transform.translation - ev.translation).length();
            if distance < ev.hitbox {
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
    }
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("hammer.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32., 32.),
        3,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
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
        })
        .insert(player_attach::PlayerAttach::new(Vec2::new(50., 30.)))
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
    buttons: Res<Input<MouseButton>>,
    mut ev_stomp: EventWriter<HammerStomp>,
) {
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
            if buttons.pressed(MouseButton::Left) {
                hammer.stomp_time = 3.5;
                hammer.cooldown = 5.;
            }
        }
    }
}
