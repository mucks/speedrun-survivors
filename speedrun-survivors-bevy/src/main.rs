use crate::enemy::enemy_spawner::SpawnEnemiesPlugin;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::plugins::assets::AssetsPlugin;
use crate::plugins::audio_manager::AudioManagerPlugin;
use crate::plugins::camera_shake::CameraShakePlugin;
use crate::plugins::coin_rewards::CoinRewardsPlugin;
use crate::plugins::hud::HudPlugin;
use crate::plugins::menu::MenuPlugin;
use crate::state::{AppState, ForState, StatesPlugin};
use bevy::audio::VolumeLevel;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use plugins::combat_text::CombatTextPlugin;
use plugins::health::HealthPlugin;
use plugins::status_effect::StatusEffectPlugin;
use weapon::WeaponPlugin;

mod animation;
mod cursor_info;
mod enemy;
mod player;

mod heroes;
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
            AudioManagerPlugin,
            MenuPlugin,
            StatesPlugin,
            SpawnEnemiesPlugin,
            EnemyPlugin,
            PlayerPlugin,
            HudPlugin,
            CoinRewardsPlugin,
            CameraShakePlugin,
            HealthPlugin,
            CombatTextPlugin,
            WeaponPlugin,
            StatusEffectPlugin,
        ))
        .add_systems(
            Update,
            (animation::animate_sprite,).run_if(in_state(AppState::GameRunning)),
        )
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(AppState::GameRunning),
            (on_enter_game_running, spawn_ldtk_level),
        )
        .add_systems(OnExit(AppState::GameRunning), (on_exit_game_running,))
        .run();
}

fn on_enter_game_running(mut commands: Commands, mut volume: ResMut<GlobalVolume>) {
    #[cfg(feature = "dev")]
    {
        volume.volume = VolumeLevel::new(0.)
    }

    //TODO run logic when a new game starts
    commands.insert_resource(LevelSelection::Index(0));
}

fn on_exit_game_running(mut commands: Commands) {
    //TODO run logic when game ends
    commands.insert_resource(LevelSelection::Index(1));
}

fn spawn_ldtk_level(asset_server: Res<AssetServer>, mut commands: Commands) {
    let level_scale = 5.;
    let level_witdh = 512. * level_scale;
    let level_height = 512. * level_scale;

    let mut transform = Transform::from_scale(Vec3::new(level_scale, level_scale, 0.1));
    transform.translation = Vec3::new(-level_witdh / 2., -level_height / 2., -10.);

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("level/level.ldtk"),
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
