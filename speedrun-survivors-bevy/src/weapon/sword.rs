use std::collections::HashMap;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::menu::MenuGameConfig;
use crate::plugins::assets::GameAssets;
use crate::plugins::gameplay_effects::{GameplayEffectPluginState, GameplayTag};
use crate::plugins::health::{self};
use crate::plugins::vfx_manager::{PlayVFX, VFX};
use crate::state::{for_game_states, AppState};
use crate::{
    animation::{self, Animator},
    enemy::Enemy,
    player::player_attach,
    GameAction,
};

use super::weapon_type::WeaponType;

const SWORD_DAMAGE: f32 = 1.;
const SWORD_SWING_TIME: f32 = 8.5;
const SWORD_COOLDOWN: f32 = 0.9;

const SWORD_EFFECT_HIT_DISTANCE: f32 = 50.;
const SWORD_EFFECT_SPEED: f32 = 1000.;
const SWORD_EFFECT_TIME_TO_LIVE: f32 = 1.;

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
    pub swing_time: f32,
    pub is_swinging: bool,
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

#[derive(Clone, Component)]
pub struct SwordEffect {
    pub time_to_live: f32,
    pub direction: Vec3,
    pub hit_list: Vec<Entity>,
}

fn move_sword_swing_effect(
    time: Res<Time>,
    mut sword_query: Query<(&mut Transform, &mut SwordEffect, Entity)>,
    mut commands: Commands,
) {
    let delta = time.delta_seconds();
    let move_by = SWORD_EFFECT_SPEED * delta;

    for (mut transform, mut effect, entity) in sword_query.iter_mut() {
        // Reduce time to live
        effect.time_to_live -= delta;

        // Check if we need to delete it
        if effect.time_to_live <= 0. {
            commands.entity(entity).despawn_recursive();
        } else {
            // Update position
            transform.translation.x += move_by * effect.direction.x;
            //TODO use an orientation so we can move this into 8 directions depending on WASD ?
        }
    }
}

fn spawn_sword_swing_effect(
    commands: &mut Commands,
    game_assets: &Res<GameAssets>,
    translation: Vec3,
    flip_x: bool,
) -> Entity {
    commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..default()
            },
            SwordEffect {
                time_to_live: SWORD_EFFECT_TIME_TO_LIVE,
                direction: if flip_x {
                    Vec3::new(-1., 0., 0.)
                } else {
                    Vec3::new(1., 0., 0.)
                },
                hit_list: vec![],
            },
            for_game_states(),
        ))
        .id()
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
    mut tx_vfx: EventWriter<PlayVFX>,
) {
    let action = actions.single();

    for (mut sword_controller, transform, mut animator, ta) in sword_query.iter_mut() {
        if sword_controller.swing_time > 0. {
            // this if clause is run once on swing start
            if !sword_controller.is_swinging {
                tx_vfx.send(PlayVFX {
                    vfx: VFX::SwordShockwave,
                    location: Vec3::default(), // This will be a child of the sword effect created below, its position will be relative, so keep at 0
                    scale: match ta.flip_x {
                        true => Some(Vec3::new(-1.0, 1.0, 1.0)),
                        false => None,
                    },
                    entity: Some(spawn_sword_swing_effect(
                        &mut commands,
                        &game_assets,
                        transform.translation,
                        ta.flip_x,
                    )),
                });
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
    mut sword_effect_query: Query<(&Transform, Entity, &mut SwordEffect), Without<Enemy>>,
    mut enemy_query: Query<(&Enemy, &mut Transform, Entity), Without<SwordEffect>>,
    mut tx_health: EventWriter<health::HealthUpdateEvent>,
) {
    for (transform, _, mut sword_effect) in sword_effect_query.iter_mut() {
        let eff_loc = transform.translation.truncate();

        for (enemy, enemy_loc, entity) in enemy_query.iter_mut() {
            // Skip enemy if already hit
            if sword_effect.hit_list.contains(&entity) {
                continue;
            }

            // Within hit distance?
            if Vec2::distance(eff_loc, enemy_loc.translation.truncate())
                <= SWORD_EFFECT_HIT_DISTANCE
            {
                // Mark as hit
                sword_effect.hit_list.push(entity);

                tx_health.send(health::HealthUpdateEvent {
                    entity,
                    health_change: -SWORD_DAMAGE,
                    target_type: health::TargetType::Enemy(enemy.kind),
                });
            }
        }
    }
}
