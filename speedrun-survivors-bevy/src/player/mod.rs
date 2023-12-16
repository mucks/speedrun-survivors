use std::collections::HashMap;

use bevy::prelude::*;

use crate::plugins::health::{add_health_bar, Health};
use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    cursor_info::OffsetedCursorPosition,
    weapon::weapon_type::WeaponType,
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

fn create_player_anim_hashmap() -> HashMap<String, animation::Animation> {
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
        "Walk".to_string(),
        animation::Animation {
            start: 1,
            end: 4,
            looping: true,
            cooldown: 0.1,
        },
    );
    hash_map
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // player
    let texture_handle = asset_server.load("sprites/player/pepe.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32., 56.),
        4,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar = add_health_bar(&mut commands, Vec3::default(), 5.);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(2.)),
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
            animation_bank: create_player_anim_hashmap(),
        })
        .insert(Player {})
        .insert(PlayerMovement { speed: 100. })
        .insert(Health::new(200., 200., 10.0, Some(health_bar)));
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Animator)>,
    mut weapon_query: Query<
        (&mut TextureAtlasSprite, &mut PlayerAttach),
        (With<WeaponType>, Without<PlayerMovement>),
    >,
    cursor_res: ResMut<OffsetedCursorPosition>,
) {
    for (player_movement, mut transform, mut animator) in query.iter_mut() {
        animator.current_animation = "Idle".to_string();

        // supports WASD & DVORAK
        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Comma) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y += player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::O) {
            animator.current_animation = "Walk".to_string();
            transform.translation.y -= player_movement.speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            animator.current_animation = "Walk".to_string();
            transform.translation.x -= player_movement.speed * time.delta_seconds();
            // turn the sprite around if moving left
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);

            for (mut weapon, mut pa) in weapon_query.iter_mut() {
                weapon.flip_x = true;
                pa.flip_x = true;
            }
        }
        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::E) {
            animator.current_animation = "Walk".to_string();
            transform.translation.x += player_movement.speed * time.delta_seconds();
            transform.rotation = Quat::default();

            for (mut weapon, mut pa) in weapon_query.iter_mut() {
                weapon.flip_x = false;
                pa.flip_x = false;
            }
        }
    }
}
