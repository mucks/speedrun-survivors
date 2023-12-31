use crate::enemy::enemy_spawner::SpawnEnemiesPlugin;
use crate::enemy::EnemyPlugin;
use crate::menu::MenuPlugin;
use crate::passives::orca_chopper::OrcaChopperPlugin;
use crate::passives::rug_pull::RugPullPlugin;
use crate::passives::shitcoin::ShitcoinClusterPlugin;
use crate::passives::whale::WhaleDumpPlugin;
use crate::player::PlayerPlugin;
use crate::plugins::assets::AssetsPlugin;
use crate::plugins::camera_shake::CameraShakePlugin;
use crate::plugins::coin_rewards::CoinRewardsPlugin;
use crate::plugins::gameplay_effects::GameplayEffectsPlugin;
use crate::plugins::hud::HudPlugin;
use crate::plugins::pickup::PickupPlugin;
use crate::plugins::sfx_manager::SFXManagerPlugin;
use crate::plugins::vfx_manager::VFXManagerPlugin;
use crate::state::{for_game_states, AppState, StatesPlugin};
use actives::dash::DashPlugin;
use bevy::audio::VolumeLevel;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use plugins::assets::GameAssets;
use plugins::combat_text::CombatTextPlugin;
use plugins::health::HealthPlugin;
use plugins::status_effect::StatusEffectPlugin;
use weapon::WeaponPlugin;

mod actives;
mod animation;
mod data;
mod enemy;
mod menu;
mod passives;
mod player;
mod plugins;
mod state;
mod weapon;

const GAME_NAME: &str = "Speedrun Survivors";

const COLOR_SOL_SURGE: Color = Color::rgb(0.0, 1.0, 0.63921568627);
const COLOR_SOL_OCEAN: Color = Color::rgb(0.01176470588, 0.88235294118, 1.0);
const COLOR_SOL_DINO: Color = Color::rgb(0.86274509804, 0.12156862745, 1.0);

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum GameAction {
    MoveUp,
    MoveLeft,
    MoveDown,
    MoveRight,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Action1,
    Action2,
    Action3,
    Cancel,
    Confirm,
    Pause,
    HackLevelUp,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_NAME.to_string(),
                        // resolution: WindowResolution::new(1024.0, 768.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((LdtkPlugin, InputManagerPlugin::<GameAction>::default()))
        .add_plugins((
            AssetsPlugin,
            SFXManagerPlugin,
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
            DashPlugin,
            StatusEffectPlugin,
        ))
        .add_plugins((
            GameplayEffectsPlugin,
            OrcaChopperPlugin,
            WhaleDumpPlugin,
            ShitcoinClusterPlugin,
            RugPullPlugin,
            PickupPlugin,
            VFXManagerPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_key_bindings))
        .add_systems(
            Update,
            (animation::animate_sprite).run_if(in_state(AppState::GameRunning)), //TODO make an anim plugin to move logic there?
        )
        .add_systems(
            OnEnter(AppState::GameInitializing),
            (on_enter_game_init, spawn_ldtk_map),
        )
        .add_systems(OnExit(AppState::GameOver), on_exit_game_over)
        .insert_resource(LdtkSettings {
            level_background: LevelBackground::Nonexistent, // Fixes an issue with Chrome not rendering the map
            ..default()
        })
        .run();
}

fn on_enter_game_init(mut commands: Commands, mut volume: ResMut<GlobalVolume>) {
    #[cfg(feature = "dev")]
    {
        volume.volume = VolumeLevel::new(0.)
    }

    commands.insert_resource(LevelSelection::Index(0));
}

fn on_exit_game_over(mut commands: Commands) {
    commands.insert_resource(LevelSelection::Index(1));
}

fn spawn_ldtk_map(game_assets: Res<GameAssets>, mut commands: Commands) {
    let (map_id, map_asset) = game_assets.map.clone();

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: map_asset,
            transform: map_id.get_map_transform(),
            ..Default::default()
        },
        for_game_states(),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn((
    //     Camera2dBundle {
    //         camera: Camera {
    //             hdr: true,
    //             ..default()
    //         },
    //         tonemapping: Tonemapping::TonyMcMapface,
    //         ..default()
    //     },
    //     BloomSettings::default(),
    // )); Can use this for better flame thrower https://bevyengine.org/examples/2D%20Rendering/bloom-2d/
}

fn setup_key_bindings(mut commands: Commands) {
    // Keyboard bindings
    let mut input_map = InputMap::<GameAction>::new([
        (KeyCode::W, GameAction::MoveUp),
        (KeyCode::A, GameAction::MoveLeft),
        (KeyCode::S, GameAction::MoveDown),
        (KeyCode::D, GameAction::MoveRight),
        (KeyCode::Key1, GameAction::Slot1),
        (KeyCode::Key2, GameAction::Slot2),
        (KeyCode::Key3, GameAction::Slot3),
        (KeyCode::Key4, GameAction::Slot4),
        (KeyCode::Key5, GameAction::Slot5),
        (KeyCode::Key6, GameAction::Slot6),
        (KeyCode::Space, GameAction::Action3),
        (KeyCode::Return, GameAction::Confirm),
        (KeyCode::Escape, GameAction::Cancel),
        (KeyCode::P, GameAction::Pause),
        (KeyCode::Pause, GameAction::Pause),
        (KeyCode::B, GameAction::HackLevelUp),
    ]);

    // Mouse bindings
    input_map.insert(InputKind::Mouse(MouseButton::Left), GameAction::Action1);
    input_map.insert(InputKind::Mouse(MouseButton::Right), GameAction::Action2);

    // Gamepad bindings
    input_map.insert(GamepadButtonType::DPadUp, GameAction::MoveUp);
    input_map.insert(GamepadButtonType::DPadLeft, GameAction::MoveLeft);
    input_map.insert(GamepadButtonType::DPadDown, GameAction::MoveDown);
    input_map.insert(GamepadButtonType::DPadRight, GameAction::MoveRight);

    // input_map.insert(
    //     SingleAxis::symmetric(GamepadAxisType::LeftStickY, -0.1),
    //     GameAction::MoveUp,
    // );
    // input_map.insert(
    //     SingleAxis::symmetric(GamepadAxisType::LeftStickY, 0.1),
    //     GameAction::MoveDown,
    // );
    // input_map.insert(
    //     SingleAxis::symmetric(GamepadAxisType::LeftStickX, -0.1),
    //     GameAction::MoveLeft,
    // );
    // input_map.insert(
    //     SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.1),
    //     GameAction::MoveRight,
    // );

    input_map.insert(GamepadButtonType::Select, GameAction::Confirm);
    input_map.insert(GamepadButtonType::Start, GameAction::Confirm);
    input_map.insert(GamepadButtonType::Z, GameAction::Cancel);
    input_map.insert(GamepadButtonType::Mode, GameAction::Cancel);

    input_map.insert(GamepadButtonType::West, GameAction::Slot1);
    input_map.insert(GamepadButtonType::North, GameAction::Slot2);
    input_map.insert(GamepadButtonType::East, GameAction::Slot3);
    input_map.insert(GamepadButtonType::South, GameAction::Slot4);

    input_map.insert(GamepadButtonType::LeftTrigger, GameAction::Action1);
    input_map.insert(GamepadButtonType::RightTrigger, GameAction::Action2);

    input_map.insert(GamepadButtonType::RightThumb, GameAction::Action3);

    commands.spawn(InputManagerBundle::<GameAction> {
        action_state: ActionState::default(),
        input_map,
    });
}
