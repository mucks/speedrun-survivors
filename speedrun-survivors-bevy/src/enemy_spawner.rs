use std::collections::HashMap;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    animation::{self, Animator},
    enemy::Enemy,
    health::{add_health_bar, Health},
};

#[derive(Component)]
pub struct EnemySpawner {
    pub cooldown: f32,
    pub timer: f32,
}
pub fn create_enemy_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();

    hash_map.insert(
        "Walk".to_string(),
        animation::Animation {
            start: 1,
            end: 2,
            looping: true,
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

    return hash_map;
}
pub fn update_spawning(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut spawner_query: Query<&mut EnemySpawner>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for mut spawner in spawner_query.iter_mut() {
        spawner.timer -= time.delta_seconds();
        if spawner.timer <= 0. {
            let Ok(primary) = primary_query.get_single() else {
                return;
            };

            spawner.timer = spawner.cooldown;
            let texture_handle = asset_server.load("enemy.png");
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(32., 32.),
                2,
                1,
                Some(Vec2::new(1., 1.)),
                None,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let mut spawn_transform = Transform::from_scale(Vec3::splat(5.));

            let mut rng = rand::thread_rng();

            if rng.gen_range(0..2) == 1 {
                if rng.gen_range(0..2) == 1 {
                    spawn_transform.translation = Vec3::new(
                        primary.width() / 2.,
                        rng.gen_range(-primary.height() / 2.0..primary.height() / 2.0),
                        0.,
                    );
                } else {
                    spawn_transform.translation = Vec3::new(
                        -primary.width() / 2.,
                        rng.gen_range(-primary.height() / 2.0..primary.height() / 2.0),
                        0.,
                    );
                }
            } else {
                if rng.gen_range(0..2) == 1 {
                    spawn_transform.translation = Vec3::new(
                        rng.gen_range(-primary.width() / 2.0..primary.width() / 2.0),
                        primary.height() / 2.,
                        0.,
                    );
                } else {
                    spawn_transform.translation = Vec3::new(
                        rng.gen_range(-primary.width() / 2.0..primary.width() / 2.0),
                        -primary.height() / 2.,
                        0.,
                    );
                }
            }

            let entity = commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: spawn_transform,
                    ..default()
                })
                .insert(Animator {
                    animation_bank: create_enemy_anim_hashmap(),
                    timer: 0.,
                    cooldown: 0.05,
                    last_animation: "Walk".to_string(),
                    current_animation: "Walk".to_string(),
                })
                .insert(Enemy { speed: 100. })
                .insert(Health {
                    active_health: 2.,
                    max_health: 2.,
                    is_player: false,
                })
                .id();

            add_health_bar(&mut commands, entity, spawn_transform.translation, 1.);
        }
    }
}
