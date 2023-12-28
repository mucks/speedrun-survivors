use std::collections::HashMap;
use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::plugins::assets::GameAssets;
use crate::plugins::status_effect::StatusEffectController;
use crate::state::{for_game_states, AppState};
use crate::{
    animation::{self, Animator},
    enemy::Enemy,
};

use super::enemy_type::EnemyType;

pub struct SpawnEnemiesPlugin;

impl Plugin for SpawnEnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameInitializing), on_enter_game_init)
            .add_systems(
                Update,
                (update_spawning).run_if(in_state(AppState::GameRunning)),
            )
            .insert_resource(EnemySpawnerState::default());
    }
}

fn on_enter_game_init(mut spawner: ResMut<EnemySpawnerState>) {
    *spawner = EnemySpawnerState::default();
}

#[derive(Resource)]
pub struct EnemySpawnerState {
    pub timer: Timer,
}

impl Default for EnemySpawnerState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}

pub fn create_enemy_anim_hashmap(walk_frames: usize) -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();

    hash_map.insert(
        "Walk".to_string(),
        animation::Animation {
            start: 1,
            end: walk_frames,
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
    mut spawner: ResMut<EnemySpawnerState>,
    time: Res<Time>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    spawner.timer.tick(time.delta());

    if !spawner.timer.finished() {
        return;
    }

    let Ok(primary) = primary_query.get_single() else {
        return;
    };

    // Reduce timer period for every monster spawned
    // TODO find good logic for this, this is very self limiting xD
    //  should also use GameplayEffectPluginState spawn rate modifiers
    let current_duration = spawner.timer.duration().clone();
    if current_duration > Duration::from_millis(500) {
        spawner
            .timer
            .set_duration(current_duration - Duration::from_millis(10));
    } else if current_duration > Duration::from_millis(120) {
        spawner
            .timer
            .set_duration(current_duration - Duration::from_millis(1));
    }

    // eprintln!("SPAWN TIMER NOW: {current_duration:?}");

    let mut rng = rand::thread_rng();

    let enemy_type = EnemyType::random();
    let texture_atlas_handle = game_assets.enemies.get(&enemy_type).unwrap().clone();
    let mut spawn_transform = Transform::from_scale(enemy_type.scale());

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
            for_game_states(),
        ))
        .insert(Animator {
            animation_bank: create_enemy_anim_hashmap(enemy_type.frames()),
            timer: 0.,
            cooldown: 0.05,
            last_animation: "Walk".to_string(),
            current_animation: "Walk".to_string(),
            destroy_on_end: false,
        })
        .insert(Enemy {
            speed: 100.,
            attack: 1.,
            kind: enemy_type,
        })
        .insert(StatusEffectController { effects: vec![] })
        .insert(enemy_type.health());
}
