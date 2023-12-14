use std::collections::HashMap;

use bevy::prelude::*;

mod animation;
mod bullet;
mod cursor_info;
mod enemy;
mod enemy_spawner;
mod gun;
mod player;
mod player_attach;

fn main() {
    App::new()
        .insert_resource(cursor_info::OffsetedCursorPosition { x: 0., y: 0. })
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Update, player::move_player)
        .add_systems(Update, bullet::update_bullets)
        .add_systems(Update, bullet::update_bullet_hits)
        .add_systems(Update, gun::gun_controls)
        .add_systems(Update, player_attach::attach_objects)
        .add_systems(Update, animation::animate_sprite)
        .add_systems(Update, enemy::update_enemies)
        .add_systems(Update, enemy_spawner::update_spawning)
        .add_systems(Startup, setup)
        .run();
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

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

    commands
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
        .insert(player::PlayerMovement { speed: 100. });

    // gun
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
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(5.)),
            ..Default::default()
        })
        .insert(animation::Animator {
            timer: 0.,
            cooldown: 0.05,
            last_animation: "Shoot".to_string(),
            current_animation: "Shoot".to_string(),
            animation_bank: create_gun_anim_hashmap(),
        })
        .insert(player_attach::PlayerAttach {
            offset: Vec2::new(15., -5.),
        })
        .insert(gun::GunController {
            shoot_timer: 0.,
            shoot_cooldown: 0.1,
        });

    commands
        .spawn(TransformBundle { ..default() })
        .insert(enemy_spawner::EnemySpawner {
            cooldown: 1.,
            timer: 1.,
        });
}
