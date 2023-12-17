use crate::state::{AppState, ForState};
use bevy::prelude::*;

// TODO use this for spatial audio
//  (all sounds currently originate on the player anyway -> LATER)
//  https://bevyengine.org/examples/Audio/spatial-audio-2d/
//  https://github.com/bevyengine/bevy/blob/main/examples/audio/spatial_audio_2d.rs
//  Maybe for spawning bosses to indicate where they come from or so
const AUDIO_SCALE: f32 = 1. / 100.0;

pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
            .add_systems(Update, (on_update).run_if(in_state(AppState::GameRunning)))
            .add_event::<PlaySFX>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioPluginAssets {
        music_combat: asset_server.load("audio/neon_gaming_dopestuff.ogg").into(),
        sfx_sword_hit: asset_server.load("audio/sfx/sword_hit.ogg").into(),
        sfx_sword_miss: asset_server.load("audio/sfx/sword_miss.ogg").into(),
        sfx_gun_shot: asset_server.load("audio/sfx/gun_shot.ogg").into(),
    });
    commands.insert_resource(AudioPluginState::default());
}

fn on_enter_game_running(
    mut commands: Commands,
    mut state: ResMut<AudioPluginState>,
    assets: Res<AudioPluginAssets>,
) {
    // Play music when the level starts
    commands.spawn((
        AudioBundle {
            source: assets.music_combat.clone(),
            settings: PlaybackSettings::LOOP,
            ..default()
        },
        ForState {
            states: vec![AppState::GameRunning],
        },
    ));
}

fn on_exit_game_running(mut data: ResMut<AudioPluginState>) {}

fn on_update(
    mut commands: Commands,
    assets: Res<AudioPluginAssets>,
    mut sfx_events: EventReader<PlaySFX>,
) {
    for event in sfx_events.iter() {
        commands.spawn((
            AudioBundle {
                source: assets.get_asset_by_sfx(&event.sfx),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ));
    }
}

#[derive(Event)]
pub struct PlaySFX {
    /// The sound effect to play
    pub sfx: SFX,
    /// Use for spatial audio (NOT IMPLEMENTED)
    pub location: Option<Vec2>,
}

pub enum SFX {
    AttackSwordMiss,
    AttackSwordHit,
    AttackHammerHit,
    AttackHammerMiss,
    AttackGun,
    Pain,
    TODO,
}

#[derive(Resource)]
struct AudioPluginState {}

impl Default for AudioPluginState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Resource)]
struct AudioPluginAssets {
    music_combat: Handle<AudioSource>,
    sfx_sword_hit: Handle<AudioSource>,
    sfx_sword_miss: Handle<AudioSource>,
    sfx_gun_shot: Handle<AudioSource>,
}

impl AudioPluginAssets {
    fn get_asset_by_sfx(&self, sfx: &SFX) -> Handle<AudioSource> {
        match sfx {
            SFX::AttackSwordMiss => self.sfx_sword_miss.clone(),
            SFX::AttackSwordHit => self.sfx_sword_hit.clone(),
            SFX::AttackHammerHit => self.sfx_sword_hit.clone(),
            SFX::AttackHammerMiss => self.sfx_sword_miss.clone(),
            SFX::AttackGun => self.sfx_gun_shot.clone(),
            SFX::Pain => self.sfx_sword_miss.clone(),
            SFX::TODO => self.sfx_sword_miss.clone(),
        }
    }
}
