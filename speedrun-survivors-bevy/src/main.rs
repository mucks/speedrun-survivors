use std::collections::HashMap;

use bevy::{prelude::*, transform::commands};
use bevy_ecs_ldtk::prelude::*;

mod animation;
mod bullet;
mod cursor_info;
mod enemy;
mod enemy_spawner;
mod health;
mod player;
mod player_attach;
mod player_camera;

mod weapon;

fn main() {
    App::new()
        .insert_resource(cursor_info::OffsetedCursorPosition { x: 0., y: 0. })
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Update, player::move_player)
        .add_systems(Update, bullet::update_bullets)
        .add_systems(Update, bullet::update_bullet_hits)
        .add_systems(Update, weapon::gun::gun_controls)
        .add_systems(Update, player_attach::attach_objects)
        .add_systems(Update, animation::animate_sprite)
        .add_systems(Update, enemy::update_enemies)
        .add_systems(Update, enemy_spawner::update_spawning)
        .add_systems(Update, weapon::sword::update_sword_hits)
        .add_systems(Update, weapon::sword::sword_controls)
        .add_systems(Update, player_camera::sync_player_camera)
        .add_systems(Startup, spawn_ldtk_level)
        .insert_resource(LevelSelection::Index(0))
        .add_systems(Startup, player::spawn_player)
        // .add_systems(Startup, spawn_gun)
        .add_systems(Startup, weapon::sword::spawn_sword)
        .add_systems(Startup, spawn_enemy_spawner)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, health::update_health_bar)
        .add_systems(Update, enemy::update_enemy_hits)
        .run();
}

fn spawn_ldtk_level(asset_server: Res<AssetServer>, mut commands: Commands) {
    let level_witdh = 256. * 10.;
    let level_height = 256. * 10.;

    let mut transform = Transform::from_scale(Vec3::new(10., 10., 0.1));
    transform.translation = Vec3::new(-level_witdh / 2., -level_height / 2., -10.);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        transform,
        ..Default::default()
    });
}

fn spawn_enemy_spawner(mut commands: Commands) {
    commands
        .spawn(TransformBundle { ..default() })
        .insert(enemy_spawner::EnemySpawner {
            cooldown: 1.,
            timer: 1.,
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
