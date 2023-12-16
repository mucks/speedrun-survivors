use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    animation::{self, Animator},
    cursor_info::OffsetedCursorPosition,
    health::{add_health_bar, Health},
    player_attach::PlayerAttach,
    weapon::weapon_type::WeaponType,
};

#[derive(Debug, Clone, Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

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
            end: 3,
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
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(9., 10.),
        3,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(5.)),
            ..Default::default()
        })
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 0.05,
            last_animation: "Walk".to_string(),
            current_animation: "Walk".to_string(),
            animation_bank: create_player_anim_hashmap(),
        })
        .insert(PlayerMovement { speed: 100. })
        .insert(Health {
            active_health: 200.,
            max_health: 200.,
            is_player: true,
        })
        .id();

    add_health_bar(&mut commands, entity, Vec3::default(), 5.);
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
