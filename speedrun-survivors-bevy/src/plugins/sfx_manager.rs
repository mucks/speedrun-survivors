use crate::state::{for_game_states, AppState, ForState};
use bevy::prelude::*;

// TODO use this for spatial audio
//  (all sounds currently originate on the player anyway -> LATER)
//  https://bevyengine.org/examples/Audio/spatial-audio-2d/
//  https://github.com/bevyengine/bevy/blob/main/examples/audio/spatial_audio_2d.rs
//  Maybe for spawning bosses to indicate where they come from or so
// const AUDIO_SCALE: f32 = 1. / 100.0;

pub struct SFXManagerPlugin;

impl Plugin for SFXManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::GameInitializing), on_enter_game_init)
            .add_systems(OnEnter(AppState::GameOver), on_enter_game_over)
            .add_systems(Update, event_reader.run_if(in_state(AppState::GameRunning)))
            .add_event::<PlaySFX>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SFXPluginAssets {
        music_combat: asset_server
            .load("audio/music/neon_gaming_dopestuff.ogg")
            .into(),
        sfx_sword_hit: asset_server.load("audio/sfx/sword_hit.ogg").into(),
        sfx_sword_miss: asset_server.load("audio/sfx/sword_miss.ogg").into(),
        sfx_gun_shot: asset_server.load("audio/sfx/gun_shot.ogg").into(),
        sfx_game_over: asset_server.load("audio/sfx/game_over.ogg").into(),
    });
}

fn on_enter_game_init(mut commands: Commands, assets: Res<SFXPluginAssets>) {
    // Play music when the maps starts
    commands.spawn((
        AudioBundle {
            source: assets.music_combat.clone(),
            settings: PlaybackSettings::LOOP,
            ..default()
        },
        for_game_states(),
    ));
}

fn on_enter_game_over(mut commands: Commands, assets: Res<SFXPluginAssets>) {
    // Play the game over sound effect
    commands.spawn((
        AudioBundle {
            source: assets.sfx_game_over.clone(),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        },
        ForState {
            states: vec![AppState::GameOver],
        },
    ));
}

fn event_reader(
    mut commands: Commands,
    assets: Res<SFXPluginAssets>,
    mut rx_sfx: EventReader<PlaySFX>,
) {
    for event in rx_sfx.iter() {
        commands.spawn((
            AudioBundle {
                source: assets.get_asset_by_sfx(&event.sfx),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            },
            for_game_states(), //TODO not sure if this should play in level up menu
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
struct SFXPluginAssets {
    music_combat: Handle<AudioSource>,
    sfx_sword_hit: Handle<AudioSource>,
    sfx_sword_miss: Handle<AudioSource>,
    sfx_gun_shot: Handle<AudioSource>,
    sfx_game_over: Handle<AudioSource>,
}

impl SFXPluginAssets {
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
