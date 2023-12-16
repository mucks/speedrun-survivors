use std::collections::HashMap;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    cursor_info::OffsetedCursorPosition,
    enemy::Enemy,
    health::Health,
    player_attach,
};

use super::weapon_type::WeaponType;

#[derive(Debug, Component)]
pub struct SwordController {
    pub hitbox: f32,
    pub swing_time: f32,
    pub cooldown: f32,
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
            end: 3,
            looping: false,
            cooldown: 0.1,
        },
    );
    hash_map
}

pub fn spawn_sword(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sword.png");
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
                transform: Transform::from_scale(Vec3::splat(2.5)),
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
            animation_bank: create_sword_anim_hashmap(),
        })
        .insert(player_attach::PlayerAttach::new(Vec2::new(25., 10.)))
        .insert(SwordController {
            hitbox: 12.,
            swing_time: 0.,
            cooldown: 0.,
            is_swinging: false,
        })
        .insert(WeaponType::Sword);
}

pub fn sword_controls(
    mut sword_query: Query<(&mut SwordController, &mut Transform, &mut Animator)>,
    buttons: Res<Input<MouseButton>>,
) {
    for (mut sword_controller, mut _transform, mut animator) in sword_query.iter_mut() {
        if sword_controller.cooldown > 0. {
            sword_controller.cooldown -= 0.1;
        }

        if sword_controller.swing_time > 0. {
            animator.current_animation = "Swing".to_string();
            sword_controller.swing_time -= 0.15;
            sword_controller.is_swinging = true;
        } else {
            animator.current_animation = "Idle".to_string();
            sword_controller.is_swinging = false;
        }

        if sword_controller.swing_time <= 0. && sword_controller.cooldown <= 0. {
            if buttons.pressed(MouseButton::Left) {
                sword_controller.swing_time = 3.5;
                sword_controller.cooldown = 5.;
            }
        }
    }
}

pub fn update_sword_hits(
    sword_query: Query<
        (&Transform, Entity, &SwordController),
        (With<SwordController>, Without<Enemy>),
    >,
    mut enemy_query: Query<(&mut Enemy, &mut Transform, &mut Health), Without<SwordController>>,
) {
    if let Some((transform, _, sword)) = sword_query.iter().next() {
        let s = Vec2::new(transform.translation.x, transform.translation.y);

        if !sword.is_swinging {
            return;
        }

        for (mut enemy, transform, mut health) in enemy_query.iter_mut() {
            if Vec2::distance(
                s,
                Vec2::new(transform.translation.x, transform.translation.y),
            ) <= 32.
            {
                health.active_health -= 1.;
            }
        }
    }
}
