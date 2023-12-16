use crate::enemy::EnemyPlugin;
use crate::enemy_spawner::SpawnEnemiesPlugin;
use crate::player::PlayerPlugin;
use crate::plugins::assets::AssetsPlugin;
use crate::plugins::coin_rewards::CoinRewardsPlugin;
use crate::plugins::hud::HudPlugin;
use crate::plugins::menu::MenuPlugin;
use crate::state::{AppState, ForState, StatesPlugin};
use bevy::prelude::*;
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

mod plugins;
mod state;
mod weapon;

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(cursor_info::OffsetedCursorPosition { x: 0., y: 0. })
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Speedrun Survivors".to_string(),
                        // resolution: WindowResolution::new(1024.0, 768.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .add_plugins((
            AssetsPlugin,
            MenuPlugin,
            StatesPlugin,
            SpawnEnemiesPlugin,
            EnemyPlugin,
            PlayerPlugin,
            HudPlugin,
            CoinRewardsPlugin,
        ))
        .add_systems(
            Update,
            (
                bullet::update_bullets,
                bullet::update_bullet_hits,
                weapon::gun::gun_controls,
                weapon::sword::update_sword_hits,
                weapon::sword::sword_controls,
                animation::animate_sprite,
                health::update_health_bar,
            )
                .run_if(in_state(AppState::GameRunning)),
        )
        // .add_systems(Startup, spawn_gun)
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(AppState::GameRunning),
            (
                on_enter_game_running,
                spawn_ldtk_level,
                weapon::sword::spawn_sword,
            ),
        )
        .add_systems(OnExit(AppState::GameRunning), (on_exit_game_running,))
        .run();
}

fn on_enter_game_running(mut commands: Commands) {
    //TODO run logic when a new game starts
    commands.insert_resource(LevelSelection::Index(0));
}

fn on_exit_game_running(mut commands: Commands) {
    //TODO run logic when game ends
    commands.insert_resource(LevelSelection::Index(1));
}

fn spawn_ldtk_level(asset_server: Res<AssetServer>, mut commands: Commands) {
    let level_witdh = 256. * 10.;
    let level_height = 256. * 10.;

    let mut transform = Transform::from_scale(Vec3::new(10., 10., 0.1));
    transform.translation = Vec3::new(-level_witdh / 2., -level_height / 2., -10.);

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("level.ldtk"),
            transform,
            ..Default::default()
        },
        ForState {
            states: vec![AppState::GameRunning],
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
