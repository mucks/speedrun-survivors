use std::collections::HashMap;

pub mod bullet;

use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    cursor_info::OffsetedCursorPosition,
    player::player_attach,
};
use bevy::{prelude::*, window::PrimaryWindow};

use self::bullet::Bullet;

const BULLET_LIFETIME: f32 = 10.0;

const BULLET_SPEED: f32 = 1000.;

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
            end: 5,
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

pub fn spawn_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gun.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(9., 9.),
        5,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
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
        })
        .insert(player_attach::PlayerAttach::new(Vec2::new(15., -5.)))
        .insert(GunController {
            shoot_timer: 0.,
            shoot_cooldown: 0.1,
        });
}

pub fn gun_controls(
    mut cursor_res: ResMut<OffsetedCursorPosition>,
    mut gun_query: Query<(&mut GunController, &mut Transform, &mut Animator)>,
    mut cursor: EventReader<CursorMoved>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (mut gun_controller, mut transform, mut animator) in gun_query.iter_mut() {
        gun_controller.shoot_timer -= time.delta_seconds();

        if gun_controller.shoot_timer > 0. {
            animator.current_animation = "Shoot".to_string();
        } else {
            animator.current_animation = "Idle".to_string();
        }

        let Ok(primary) = primary_query.get_single() else {
            return;
        };

        let mut cursor_position = match cursor.iter().last() {
            Some(cursor_moved) => cursor_moved.position,
            None => Vec2::new(
                cursor_res.x + primary.width() / 2.,
                cursor_res.y + primary.height() / 2.,
            ),
        };

        cursor_position.x -= primary.width() / 2.;
        cursor_position.y -= primary.height() / 2.;

        cursor_res.x = cursor_position.x;
        cursor_res.y = cursor_position.y;

        let diff = cursor_position - Vec2::new(transform.translation.x, transform.translation.y);
        let angle = diff.y.atan2(diff.x);
        transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

        if gun_controller.shoot_timer <= 0. {
            if buttons.pressed(MouseButton::Left) {
                let mut spawn_transform = Transform::from_scale(Vec3::splat(5.0));
                spawn_transform.translation = transform.translation;
                spawn_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
                gun_controller.shoot_timer = gun_controller.shoot_cooldown;
                commands
                    .spawn((
                        SpriteBundle {
                            transform: spawn_transform,
                            texture: asset_server.load("bullet.png"),
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
            }
        }
    }
}
