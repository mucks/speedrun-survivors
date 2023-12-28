use crate::animation::{Animation, Animator};
use crate::state::{for_game_states, AppState};
use bevy::prelude::*;
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};

pub struct VFXManagerPlugin;

impl Plugin for VFXManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, event_reader.run_if(in_state(AppState::GameRunning)))
            .add_event::<PlayVFX>();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(VFXPluginAssets::default(
        &asset_server,
        &mut texture_atlases,
    ));
}

fn event_reader(
    mut commands: Commands,
    assets: Res<VFXPluginAssets>,
    mut rx_vfx: EventReader<PlayVFX>,
) {
    for event in rx_vfx.iter() {
        let data = assets.get_data_for_vfx(&event.vfx);

        commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: data.atlas,
                    transform: Transform {
                        translation: event.location,
                        scale: data.scale,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                for_game_states(),
            ))
            .insert(data.anim);
    }
}

#[derive(Event)]
pub struct PlayVFX {
    /// The visual effect to play
    pub vfx: VFX,
    /// The location at which to spawn the effect
    pub location: Vec3,
}

#[derive(Clone, EnumIter, Eq, Hash, PartialEq)]
pub enum VFX {
    ExplosionXS,
    ExplosionXL,
}

impl VFX {
    fn make_texture_atlas(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        match self {
            VFX::ExplosionXS => texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("sprites/vfx/explosion_xs.png"),
                Vec2::new(48., 48.),
                5,
                5,
                None,
                None,
            )),
            VFX::ExplosionXL => texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load("sprites/vfx/explosion_xl.png"),
                Vec2::new(64., 64.),
                5,
                4,
                None,
                None,
            )),
        }
    }
    fn make_scale(&self) -> Vec3 {
        match self {
            VFX::ExplosionXS => Vec3::splat(1.0),
            VFX::ExplosionXL => Vec3::splat(1.5),
        }
    }

    fn make_animation(&self) -> Animator {
        match self {
            VFX::ExplosionXS => {
                let animation_bank = HashMap::from([(
                    "xplode".to_string(),
                    Animation {
                        start: 1,
                        end: 25,
                        looping: false,
                        cooldown: 0.1,
                    },
                )]);

                Animator {
                    timer: 0.,
                    cooldown: 10.,
                    last_animation: "xplode".to_string(),
                    current_animation: "xplode".to_string(),
                    animation_bank,
                    destroy_on_end: true,
                }
            }
            VFX::ExplosionXL => {
                let animation_bank = HashMap::from([(
                    "xplode".to_string(),
                    Animation {
                        start: 1,
                        end: 20,
                        looping: false,
                        cooldown: 0.1,
                    },
                )]);

                Animator {
                    timer: 0.,
                    cooldown: 10.,
                    last_animation: "xplode".to_string(),
                    current_animation: "xplode".to_string(),
                    animation_bank,
                    destroy_on_end: true,
                }
            }
        }
    }
}

#[derive(Clone)]
struct VFXData {
    atlas: Handle<TextureAtlas>,
    scale: Vec3,
    anim: Animator,
}

impl VFXData {
    pub fn new(sprite_sheet: Handle<TextureAtlas>, scale: Vec3, anim: Animator) -> Self {
        Self {
            atlas: sprite_sheet,
            scale,
            anim,
        }
    }
}

#[derive(Resource)]
struct VFXPluginAssets {
    effects: HashMap<VFX, VFXData>,
}

impl VFXPluginAssets {
    fn get_data_for_vfx(&self, vfx: &VFX) -> VFXData {
        self.effects[vfx].clone()
    }

    fn default(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let mut effects = HashMap::new();

        for vfx in VFX::iter() {
            effects.insert(
                vfx.clone(),
                VFXData::new(
                    vfx.make_texture_atlas(asset_server, texture_atlases),
                    vfx.make_scale(),
                    vfx.make_animation(),
                ),
            );
        }

        Self { effects }
    }
}
