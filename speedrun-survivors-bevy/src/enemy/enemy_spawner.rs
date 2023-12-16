use std::collections::HashMap;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::plugins::health::Health;
use crate::plugins::status_effect::{StatusEffect, StatusEffectController};
use crate::state::{AppState, ForState};
use crate::{
    animation::{self, Animator},
    enemy::Enemy,
};

use super::enemy_type::EnemyType;

pub struct SpawnEnemiesPlugin;

impl Plugin for SpawnEnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
            .add_systems(
                Update,
                (update_spawning).run_if(in_state(AppState::GameRunning)),
            );
    }
}

fn on_enter_game_running(mut commands: Commands) {
    commands
        .spawn((
            TransformBundle { ..default() },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(EnemySpawner {
            cooldown: 1.,
            timer: 1.,
        }); //TODO: there should be some way to say this is for gamestate... so it auto unspawns
}
fn on_exit_game_running(mut _commands: Commands) {
    // The bundle will be auto deleted at is tagged ForState
}

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
        if spawner.timer > 0f32 {
            return;
        }

        let Ok(primary) = primary_query.get_single() else {
            return;
        };

        spawner.timer = spawner.cooldown;

        let enemy_type = EnemyType::random();

        let texture_atlas_handle = texture_atlases.add(enemy_type.texture_atlas(&asset_server));
        let mut spawn_transform = Transform::from_scale(enemy_type.scale());

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

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: spawn_transform,
                    ..default()
                },
                ForState {
                    states: vec![AppState::GameRunning],
                },
            ))
            .insert(Animator {
                animation_bank: create_enemy_anim_hashmap(),
                timer: 0.,
                cooldown: 0.05,
                last_animation: "Walk".to_string(),
                current_animation: "Walk".to_string(),
                destroy_on_end: false,
            })
            .insert(Enemy {
                speed: 100.,
                attack: 1.,
            })
            .insert(StatusEffectController { effects: vec![] })
            .insert(enemy_type.health());

        //TODO lets not have healthbars for enemies as there will be hundreds and they mostly die in 1 hit probably...
        // add_health_bar(&mut commands, entity, spawn_transform.translation, 1.);
    }
}
