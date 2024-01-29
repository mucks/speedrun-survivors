use bevy::prelude::*;
#[derive(Debug, Component, PartialEq, Eq, Hash, Clone, Copy, strum::EnumIter)]
pub enum WeaponAnimationEffect {
    FlameThrowerFlame,
}

impl WeaponAnimationEffect {
    pub fn texture_atlas(&self, asset_server: &Res<AssetServer>) -> TextureAtlas {
        let flame_thrower_flame = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/flame_effect.png"),
            Vec2::new(48., 48.),
            6,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );

        match self {
            Self::FlameThrowerFlame => flame_thrower_flame,
        }
    }
}
