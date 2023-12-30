use std::collections::HashMap;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::menu::MenuGameConfig;
use crate::plugins::assets::GameAssets;
use crate::plugins::gameplay_effects::{GameplayEffectPluginState, GameplayTag};
use crate::plugins::health::{self};
use crate::state::{for_game_states, AppState};
use crate::{
    animation::{self, Animator},
    enemy::Enemy,
    player::player_attach,
    GameAction,
};

use super::weapon_animation_effect::WeaponAnimationEffect;
use super::weapon_type::WeaponType;

const SWORD_DAMAGE: f32 = 1.;
const SWORD_SWING_TIME: f32 = 8.5;
const SWORD_COOLDOWN: f32 = 0.9;

const SWORD_EFFECT_HITBOX: f32 = 50.;
const SWORD_EFFECT_SPEED: f32 = 15.;

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sword_controls,
                //update_sword_hits,
                move_sword_swing_effect,
                update_sword_effect_hits,
            )
                .run_if(in_state(AppState::GameRunning)),
        );
    }
}

#[derive(Debug, Component)]
pub struct SwordController {
    pub hitbox: f32,
    pub swing_time: f32,
    pub is_swinging: bool,
}

fn create_sword_effect_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert(
        "Swing".to_string(),
        animation::Animation {
            start: 1,
            end: 4,
            looping: false,
            cooldown: 0.1,
        },
    );
    hash_map
}

fn create_sword_anim_hashmap() -> HashMap<String, animation::Animation> {
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
        "Swing".to_string(),
        animation::Animation {
            start: 1,
            end: 8,
            looping: false,
            cooldown: 0.05,
        },
    );
    hash_map
}

#[derive(Component)]
pub struct SwordEffect {
    pub speed: f32,
    pub direction: Vec3,
    pub hitbox: f32,
}

fn move_sword_swing_effect(
    mut sword_query: Query<(&mut Transform, &SwordEffect, Entity)>,
    mut commands: Commands,
) {
    for (mut transform, effect, ent) in sword_query.iter_mut() {
        transform.translation.x += effect.speed * effect.direction.x;
        commands.entity(ent).remove::<WeaponAnimationEffect>();
    }
}

fn spawn_sword_swing_effect(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    translation: Vec3,
    flip_x: bool,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets
                    .weapon_animation_effects
                    .get(&WeaponAnimationEffect::SwordSwing)
                    .unwrap()
                    .clone(),
                transform: Transform {
                    scale: Vec3::splat(5.),
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            },
            for_game_states(),
        ))
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 10.,
            last_animation: "Swing".to_string(),
            current_animation: "Swing".to_string(),
            animation_bank: create_sword_effect_anim_hashmap(),
            destroy_on_end: true,
        })
        .insert(SwordEffect {
            speed: SWORD_EFFECT_SPEED,
            hitbox: SWORD_EFFECT_HITBOX,
            direction: if flip_x {
                Vec3::new(-1., 0., 0.)
            } else {
                Vec3::new(1., 0., 0.)
            },
        })
        .insert(WeaponAnimationEffect::SwordSwing);
}

pub fn spawn_sword(
    commands: &mut Commands,
    game_config: &Res<MenuGameConfig>,
    game_assets: &Res<GameAssets>,
) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets.weapons.get(&WeaponType::Sword).unwrap().clone(),
                transform: Transform::from_scale(Vec3::splat(2.5)),
                ..Default::default()
            },
            for_game_states(),
        ))
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 10.,
            last_animation: "Idle".to_string(),
            current_animation: "Idle".to_string(),
            animation_bank: create_sword_anim_hashmap(),
            destroy_on_end: false,
        })
        .insert(player_attach::PlayerAttach::new(
            game_config.hero.weapon_offset(WeaponType::Sword),
        ))
        .insert(SwordController {
            hitbox: 40.,
            swing_time: 0.,
            is_swinging: false,
        })
        .insert(WeaponType::Sword);
}

pub fn sword_controls(
    mut sword_query: Query<(
        &mut SwordController,
        &mut Transform,
        &mut Animator,
        &TextureAtlasSprite,
    )>,
    actions: Query<&ActionState<GameAction>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut gameplay_state: ResMut<GameplayEffectPluginState>,
) {
    let action = actions.single();

    for (mut sword_controller, transform, mut animator, ta) in sword_query.iter_mut() {
        if sword_controller.swing_time > 0. {
            // this if clause is run once on swing start
            if !sword_controller.is_swinging {
                spawn_sword_swing_effect(
                    &mut commands,
                    &game_assets,
                    transform.translation,
                    ta.flip_x,
                );
            }

            animator.current_animation = "Swing".to_string();
            sword_controller.swing_time -= 0.1;
            sword_controller.is_swinging = true;
        } else {
            animator.current_animation = "Idle".to_string();
            sword_controller.is_swinging = false;
        }

        if sword_controller.swing_time <= 0.
            && action.just_pressed(GameAction::Action1)
            && gameplay_state
                .player_tags
                .add_tag(GameplayTag::Attack, SWORD_COOLDOWN)
        {
            sword_controller.swing_time = SWORD_SWING_TIME;
        }
    }
}
fn update_sword_effect_hits(
    sword_effect_query: Query<
        (&Transform, Entity, &SwordEffect),
        (With<SwordEffect>, Without<Enemy>),
    >,
    mut enemy_query: Query<(&Enemy, &mut Transform, Entity), Without<SwordEffect>>,
    mut tx_health: EventWriter<health::HealthUpdateEvent>,
) {
    if let Some((transform, _, sword_effect)) = sword_effect_query.iter().next() {
        let s = Vec2::new(transform.translation.x, transform.translation.y);

        for (enemy, transform, ent) in enemy_query.iter_mut() {
            if Vec2::distance(s, transform.translation.truncate()) <= sword_effect.hitbox {
                tx_health.send(health::HealthUpdateEvent {
                    entity: ent,
                    health_change: -SWORD_DAMAGE,
                    target_type: health::TargetType::Enemy(enemy.kind),
                });
            }
        }
    }
}
