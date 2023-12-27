use std::collections::HashMap;

pub mod bullet;

use crate::plugins::assets::GameAssets;
use crate::plugins::sfx_manager::{PlaySFX, SFX};
use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    player::player_attach,
    GameAction,
};
use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::action_state::ActionState;

use self::bullet::Bullet;

use super::weapon_type::WeaponType;

const BULLET_LIFETIME: f32 = 10.0;

const BULLET_SPEED: f32 = 1000.;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bullet::update_bullets,
                bullet::update_bullet_hits,
                gun_controls,
            )
                .run_if(in_state(AppState::GameRunning)),
        );
    }
}

#[derive(Component)]
pub struct GunController {
    pub shoot_timer: f32,
    pub shoot_cooldown: f32,
}

fn create_gun_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();
    hash_map.insert(
        "Shoot".to_string(),
        animation::Animation {
            start: 1,
            end: 2,
            looping: false,
            cooldown: 0.1,
        },
    );
    hash_map.insert(
        "Idle".to_string(),
        animation::Animation {
            start: 1,
            end: 1,
            looping: true,
            cooldown: 0.1,
        },
    );
    hash_map
}

pub fn spawn_gun(commands: &mut Commands, game_assets: &Res<GameAssets>) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: game_assets.weapons.get(&WeaponType::Gun).unwrap().clone(),
                transform: Transform::from_scale(Vec3::splat(5.)),
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 0.05,
            last_animation: "Shoot".to_string(),
            current_animation: "Shoot".to_string(),
            animation_bank: create_gun_anim_hashmap(),
            destroy_on_end: false,
        })
        .insert(player_attach::PlayerAttach::new(Vec2::new(15., -5.)))
        .insert(GunController {
            shoot_timer: 0.,
            shoot_cooldown: 0.1,
        })
        .insert(WeaponType::Gun);
}

pub fn gun_controls(
    mut gun_query: Query<(&mut GunController, &mut Transform, &mut Animator)>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    actions: Query<&ActionState<GameAction>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut tx_sfx: EventWriter<PlaySFX>,
) {
    let action = actions.single();

    for (mut gun_controller, mut transform, mut animator) in gun_query.iter_mut() {
        let Ok(window) = primary_query.get_single() else {
            return;
        };
        let Ok((camera, camera_transform)) = query_camera.get_single() else {
            return;
        };

        gun_controller.shoot_timer -= time.delta_seconds();

        if gun_controller.shoot_timer > 0. {
            animator.current_animation = "Shoot".to_string();
        } else {
            animator.current_animation = "Idle".to_string();
        }

        // Aim gun at world location
        let Some(cursor_world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        else {
            return;
        };

        let diff =
            cursor_world_position - Vec2::new(transform.translation.x, transform.translation.y);
        let angle = diff.y.atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

        // The character can walk left or right, but the gun should not flip as we rotate it here and flip it on the y axis so it doesnt go upside down
        if cursor_world_position.x < transform.translation.x && transform.scale.y > 0. {
            transform.scale.y *= -1.;
        }
        if cursor_world_position.x >= transform.translation.x && transform.scale.y < 0. {
            transform.scale.y *= -1.;
        }

        if gun_controller.shoot_timer <= 0. {
            if action.pressed(GameAction::Action1) {
                let mut spawn_transform = Transform::from_scale(Vec3::splat(2.0));
                spawn_transform.translation = transform.translation;
                spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
                gun_controller.shoot_timer = gun_controller.shoot_cooldown;
                commands
                    .spawn((
                        SpriteBundle {
                            transform: spawn_transform,
                            texture: asset_server.load("sprites/misc/bullet.png"),
                            ..Default::default()
                        },
                        ForState {
                            states: vec![AppState::GameRunning],
                        },
                    ))
                    .insert(Bullet {
                        lifetime: BULLET_LIFETIME,
                        speed: BULLET_SPEED,
                        direction: diff.normalize(),
                    });
                tx_sfx.send(PlaySFX {
                    sfx: SFX::AttackGun,
                    location: None,
                })
            }
        }
    }
}
