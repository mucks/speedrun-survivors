use std::collections::HashMap;

use bevy::prelude::*;

use crate::plugins::health::{self, Health};
use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    enemy::Enemy,
    player::player_attach,
};

use super::weapon_type::WeaponType;

const SWORD_DAMAGE: f32 = 1.;
const SWORD_SWING_TIME: f32 = 8.5;
const SWORD_COOLDOWN: f32 = 5.;

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (sword_controls, update_sword_hits).run_if(in_state(AppState::GameRunning)),
        );
    }
}

#[derive(Debug, Component)]
pub struct SwordController {
    pub hitbox: f32,
    pub swing_time: f32,
    pub cooldown: f32,
    pub is_swinging: bool,
}

fn create_sword_effect_anim_hashmap() -> HashMap<String, animation::Animation> {
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

pub fn spawn_sword(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/weapon/sword.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32., 32.),
        8,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );

    let texture_effect_handle = asset_server.load("sprites/weapon/sword-effect.png");
    let texture_effect_atlas = TextureAtlas::from_grid(
        texture_effect_handle,
        Vec2::new(32., 32.),
        4,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let texture_atlas_effect_handle = texture_atlases.add(texture_effect_atlas);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_effect_handle,
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
            animation_bank: create_sword_effect_anim_hashmap(),
        })
        .insert(player_attach::PlayerAttach::new(Vec2::new(55., 10.)))
        .insert(SwordController {
            hitbox: 40.,
            swing_time: 0.,
            cooldown: 0.,
            is_swinging: false,
        })
        .insert(WeaponType::Sword);

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
        .insert(player_attach::PlayerAttach::new(Vec2::new(35., 15.)))
        .insert(SwordController {
            hitbox: 40.,
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
            sword_controller.swing_time -= 0.1;
            sword_controller.is_swinging = true;
        } else {
            animator.current_animation = "Idle".to_string();
            sword_controller.is_swinging = false;
        }

        if sword_controller.swing_time <= 0. && sword_controller.cooldown <= 0. {
            if buttons.just_pressed(MouseButton::Left) {
                sword_controller.swing_time = SWORD_SWING_TIME;
                sword_controller.cooldown = SWORD_COOLDOWN;
            }
        }
    }
}

pub fn update_sword_hits(
    sword_query: Query<
        (&Transform, Entity, &SwordController),
        (With<SwordController>, Without<Enemy>),
    >,
    mut enemy_query: Query<(&mut Enemy, &mut Transform, Entity), Without<SwordController>>,
    mut ev_health_change: EventWriter<health::HealthChangeEvent>,
) {
    if let Some((transform, _, sword)) = sword_query.iter().next() {
        let s = Vec2::new(transform.translation.x, transform.translation.y);

        if !sword.is_swinging {
            return;
        }

        for (mut _enemy, transform, ent) in enemy_query.iter_mut() {
            if Vec2::distance(
                s,
                Vec2::new(transform.translation.x, transform.translation.y),
            ) <= sword.hitbox
            {
                ev_health_change.send(health::HealthChangeEvent {
                    entity: ent,
                    health_change: -SWORD_DAMAGE,
                    target_type: health::HealthChangeTargetType::Enemy,
                });
            }
        }
    }
}
