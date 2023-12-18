use std::collections::HashMap;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    animation,
    enemy::Enemy,
    player::player_attach,
    plugins::{
        assets::GameAssets,
        health::{HealthChangeEvent, HealthChangeTargetType},
        menu::GameConfigState,
    },
    state::{AppState, ForState},
    GameAction,
};

use super::{weapon_animation_effect::WeaponAnimationEffect, weapon_type::WeaponType};

const FLAME_HITBOX: f32 = 80.;
const FLAME_DAMAGE: f32 = 0.1;

pub struct FlameThrowerPlugin;

impl Plugin for FlameThrowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flame_thrower_controls, update_flame_hits).run_if(in_state(AppState::GameRunning)),
        );
    }
}

#[derive(Debug, Component)]
pub struct FlameThrowerController {
    pub hitbox: f32,
    pub cooldown: f32,
    pub is_firing: bool,
    pub timer: Timer,
}

#[derive(Debug, Component)]
pub struct Flame {
    pub hitbox: f32,
    pub damage: f32,
}

fn update_flame_hits(
    mut query: Query<(&Flame, &Transform)>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut ev: EventWriter<HealthChangeEvent>,
) {
    for (flame, flame_transform) in query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            if Vec2::distance(
                Vec2::new(flame_transform.translation.x, flame_transform.translation.y),
                Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y),
            ) <= flame.hitbox
            {
                ev.send(HealthChangeEvent {
                    entity: enemy_entity,
                    health_change: -flame.damage,
                    target_type: HealthChangeTargetType::Enemy,
                });
            }
        }
    }
}

fn create_flame_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert(
        "Fire".to_string(),
        animation::Animation {
            start: 1,
            end: 6,
            looping: false,
            cooldown: 0.05,
        },
    );
    hash_map
}

fn spawn_flame_effect(commands: &mut Commands, game_assets: &Res<GameAssets>, translation: Vec3) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets
                    .weapon_animation_effects
                    .get(&WeaponAnimationEffect::FlameThrowerFlame)
                    .unwrap()
                    .clone(),
                transform: Transform {
                    scale: Vec3::splat(5.5),
                    translation,
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
            cooldown: 0.,
            last_animation: "Fire".to_string(),
            current_animation: "Fire".to_string(),
            animation_bank: create_flame_anim_hashmap(),
            destroy_on_end: true,
        })
        .insert(Flame {
            hitbox: FLAME_HITBOX,
            damage: FLAME_DAMAGE,
        })
        .insert(WeaponAnimationEffect::FlameThrowerFlame);
}

fn flame_thrower_controls(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &mut FlameThrowerController,
        &mut animation::Animator,
        &Transform,
    )>,
    actions: Query<&ActionState<GameAction>>,
    game_assets: Res<GameAssets>,
) {
    let action = actions.single();

    for (mut controller, mut animator, transform) in query.iter_mut() {
        if controller.is_firing {
            animator.current_animation = "Fire".to_string();

            controller.timer.tick(time.delta());

            if controller.timer.finished() {
                spawn_flame_effect(&mut commands, &game_assets, transform.translation);
            }
        } else {
            animator.current_animation = "Idle".to_string();
        }

        if action.pressed(GameAction::Action1) {
            controller.is_firing = true;
        } else {
            controller.is_firing = false;
        }
    }
}

fn create_flame_thrower_anim_hashmap() -> HashMap<String, animation::Animation> {
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
        "Fire".to_string(),
        animation::Animation {
            start: 1,
            end: 3,
            looping: true,
            cooldown: 0.1,
        },
    );
    hash_map
}

pub fn spawn_flame_thrower(
    commands: &mut Commands,
    game_config: &Res<GameConfigState>,
    game_assets: &Res<GameAssets>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets
                    .weapons
                    .get(&WeaponType::FlameThrower)
                    .unwrap()
                    .clone(),
                transform: Transform::from_scale(Vec3::splat(0.5)),
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 1.,
            last_animation: "Idle".to_string(),
            current_animation: "Idle".to_string(),
            animation_bank: create_flame_thrower_anim_hashmap(),
            destroy_on_end: false,
        })
        .insert(player_attach::PlayerAttach::new(
            game_config.hero.weapon_offset(WeaponType::FlameThrower),
        ))
        .insert(FlameThrowerController {
            hitbox: 10.,
            cooldown: 5.,
            is_firing: false,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
        .insert(WeaponType::FlameThrower);
}
