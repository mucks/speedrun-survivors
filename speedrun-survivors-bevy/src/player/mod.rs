use std::collections::HashMap;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::data::hero::HeroType;
use crate::data::level::Level;
use crate::data::map::MapId;
use crate::plugins::assets::GameAssets;
use crate::plugins::gameplay_effects::{GameplayEffectEvent, GameplayEffectPluginState};
use crate::plugins::health::{add_health_bar, Health};
use crate::plugins::menu::MenuGameConfig;
use crate::plugins::status_effect::{StatusEffect, StatusEffectController, StatusEffectType};
use crate::state::{AppState, ForState};
use crate::weapon::hammer::HammerStomp;
use crate::weapon::weapon_animation_effect::WeaponAnimationEffect;
use crate::{
    animation::{self, Animator},
    weapon::weapon_type::WeaponType,
    GameAction,
};

use self::player_attach::PlayerAttach;

pub mod player_attach;
pub mod player_camera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::GameRunning),
            (on_enter_game_running, spawn_player),
        )
        .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
        .add_systems(
            Update,
            (
                process_events,
                move_player,
                player_attach::attach_objects,
                player_camera::sync_player_camera,
            )
                .run_if(in_state(AppState::GameRunning)),
        )
        .add_event::<PlayerEvent>()
        .insert_resource(PlayerStats::default());
    }
}

fn on_enter_game_running(mut commands: Commands) {}
fn on_exit_game_running(mut commands: Commands) {}

pub fn process_events(
    mut rx_player: EventReader<PlayerEvent>,
    mut commands: Commands,
    mut status_query: Query<(&mut StatusEffectController, &Transform, &mut Health), With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tx_stomp: EventWriter<HammerStomp>,
    game_assets: Res<GameAssets>,
    mut player_stats: ResMut<PlayerStats>,
    mut tx_gameplay: EventWriter<GameplayEffectEvent>,
) {
    for ev in rx_player.iter() {
        match ev {
            PlayerEvent::Died => {
                let test = status_query.iter().len();
                let Ok((mut status_cont, player_tr, mut player_health)) =
                    status_query.get_single_mut()
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

                    tx_stomp.send(HammerStomp {
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
            PlayerEvent::ExpGained(exp) => {
                // Add experience
                player_stats.total_exp += exp;

                // Check if the player leveled up
                if let Some(new_level) = player_stats.level.has_leveled_up(player_stats.total_exp) {
                    player_stats.level = new_level;
                    tx_gameplay.send(GameplayEffectEvent::LevelUp(new_level));
                    eprintln!("Player leveled up: {:?}", player_stats.level);
                }

                // Calculate progress towards the next level
                player_stats.level_progress = player_stats
                    .level
                    .percent_to_level_up(player_stats.total_exp);
            }
            _ => {
                eprintln!("PLAYER EVENT {ev:?} NOT IMPLEMENTED");
            }
        }
    }
}

#[derive(Debug, Event)]
pub enum PlayerEvent {
    Died,
    ExpGained(u64),
    Ability1,
    Ability2,
}

#[derive(Resource)]
pub struct PlayerStats {
    pub total_exp: u64,
    pub level: Level,
    pub level_progress: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            total_exp: 0,
            level: Level(1),
            level_progress: 0.,
        }
    }
}

#[derive(Component)]
pub struct Player {}

fn create_player_anim_hashmap(hero_type: HeroType) -> HashMap<String, animation::Animation> {
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
    hash_map.insert("Walk".to_string(), hero_type.walk_animation());
    hash_map
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_config: Res<MenuGameConfig>,
    game_assets: Res<GameAssets>,
) {
    // player

    let hero_type = game_config.hero.clone();

    let texture_atlas = hero_type.texture_atlas(&game_assets);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar = add_health_bar(&mut commands, Vec3::default(), 5.);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(hero_type.splat_scale())),
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 0.05,
            last_animation: "Walk".to_string(),
            current_animation: "Walk".to_string(),
            animation_bank: create_player_anim_hashmap(hero_type),
            destroy_on_end: false,
        })
        .insert(Player {})
        .insert(Health::new(200., 200., 10.0, Some(health_bar)))
        .insert(StatusEffectController { effects: vec![] });
}

pub fn move_player(
    time: Res<Time>,
    actions: Query<&ActionState<GameAction>>,
    mut query: Query<(&mut Transform, &mut Animator), With<Player>>,
    mut weapon_query: Query<
        (&mut TextureAtlasSprite, &mut PlayerAttach, &WeaponType),
        (Without<Player>, Without<WeaponAnimationEffect>),
    >,
    // TODO: refactor this, probably better to use WeaponAttack for the effects
    mut weapon_animation_effect_query: Query<
        &mut TextureAtlasSprite,
        (With<WeaponAnimationEffect>, Without<WeaponType>),
    >,
    gameplay: Res<GameplayEffectPluginState>,
    game_assets: Res<GameAssets>,
) {
    let action = actions.single();

    for (mut transform, mut animator) in query.iter_mut() {
        let mut movement = Vec2::ZERO;

        if action.pressed(GameAction::MoveUp) {
            movement.y += 1.0;
        }
        if action.pressed(GameAction::MoveDown) {
            movement.y -= 1.0;
        }
        if action.pressed(GameAction::MoveLeft) {
            movement.x -= 1.0;
            transform.scale.x = movement.x.signum() * f32::abs(transform.scale.x);

            for (mut weapon, mut pa, kind) in weapon_query.iter_mut() {
                if kind != &WeaponType::Gun {
                    weapon.flip_x = true;
                    pa.flip_x = true;
                }
            }
            for mut weapon in weapon_animation_effect_query.iter_mut() {
                weapon.flip_x = true;
            }
        }
        if action.pressed(GameAction::MoveRight) {
            movement.x += 1.0;
            transform.scale.x = movement.x.signum() * f32::abs(transform.scale.x);

            for (mut weapon, mut pa, kind) in weapon_query.iter_mut() {
                weapon.flip_x = false;
                pa.flip_x = false;
            }
            for mut weapon in weapon_animation_effect_query.iter_mut() {
                weapon.flip_x = false;
            }
        }

        // Normalize speed
        if movement.length_squared() > 1.0 {
            movement = movement.normalize();
        }

        // Move player at a constant speed
        let mut new_transform = transform.clone();
        new_transform.translation.x +=
            movement.x * gameplay.player_effects.move_speed * time.delta_seconds();
        new_transform.translation.y +=
            movement.y * gameplay.player_effects.move_speed * time.delta_seconds();

        if !game_assets.map.0.is_at_border(new_transform) {
            transform.translation = new_transform.translation;
        }

        // If the vector has a length, the player is moving
        if movement.length() > 0. {
            animator.current_animation = "Walk".to_string();
        } else {
            animator.current_animation = "Idle".to_string();
        }
    }
}

fn spawn_skull_on_player(commands: &mut Commands, position: Vec2, game_assets: &Res<GameAssets>) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets.skull.clone(),
                transform: Transform {
                    scale: Vec3::splat(0.5),
                    translation: Vec3::new(position.x, position.y, 50.),
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
