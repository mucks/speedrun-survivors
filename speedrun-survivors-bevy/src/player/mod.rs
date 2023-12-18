use std::collections::HashMap;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::heroes::HeroType;
use crate::plugins::assets::GameAssets;
use crate::plugins::health::{add_health_bar, Health};
use crate::plugins::menu::MenuGameConfig;
use crate::plugins::status_effect::StatusEffectController;
use crate::state::{AppState, ForState};
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
                move_player,
                player_attach::attach_objects,
                player_camera::sync_player_camera,
            )
                .run_if(in_state(AppState::GameRunning)),
        );
    }
}

fn on_enter_game_running(mut commands: Commands) {}
fn on_exit_game_running(mut commands: Commands) {}

#[derive(Debug, Clone, Component)]
pub struct PlayerMovement {
    pub speed: f32,
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
        .insert(PlayerMovement { speed: 100. })
        .insert(Health::new(200., 200., 10.0, Some(health_bar)))
        .insert(StatusEffectController { effects: vec![] });
}

pub fn move_player(
    time: Res<Time>,
    actions: Query<&ActionState<GameAction>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Animator)>,
    mut weapon_query: Query<
        (&mut TextureAtlasSprite, &mut PlayerAttach, &WeaponType),
        (Without<PlayerMovement>, Without<WeaponAnimationEffect>),
    >,
    // TODO: refactor this, probably better to use WeaponAttack for the effects
    mut weapon_animation_effect_query: Query<
        &mut TextureAtlasSprite,
        (
            With<WeaponAnimationEffect>,
            Without<PlayerMovement>,
            Without<WeaponType>,
        ),
    >,
) {
    let action = actions.single();

    for (player_movement, mut transform, mut animator) in query.iter_mut() {
        animator.current_animation = "Idle".to_string();

        if action.pressed(GameAction::MoveUp) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y += player_movement.speed * time.delta_seconds();
        }
        if action.pressed(GameAction::MoveDown) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y -= player_movement.speed * time.delta_seconds();
        }
        if action.pressed(GameAction::MoveLeft) {
            animator.current_animation = "Walk".to_string();
            transform.translation.x -= player_movement.speed * time.delta_seconds();
            // turn the sprite around if moving left
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);

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
            animator.current_animation = "Walk".to_string();
            transform.translation.x += player_movement.speed * time.delta_seconds();
            transform.rotation = Quat::default();

            for (mut weapon, mut pa, kind) in weapon_query.iter_mut() {
                weapon.flip_x = false;
                pa.flip_x = false;
            }
            for (mut weapon) in weapon_animation_effect_query.iter_mut() {
                weapon.flip_x = false;
            }
        }
    }
}
