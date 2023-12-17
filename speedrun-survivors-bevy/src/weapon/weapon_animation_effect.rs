use bevy::prelude::*;
#[derive(Debug, Component, PartialEq, Eq, Hash, Clone, Copy, strum::EnumIter)]
pub enum WeaponAnimationEffect {
    SwordSwing,
    FlameThrowerFlame,
    HammerStomp,
}

impl WeaponAnimationEffect {
    pub fn texture_atlas(&self, asset_server: &Res<AssetServer>) -> TextureAtlas {
        let hammer_stomp = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/hammer-effect.png"),
            Vec2::new(32., 32.),
            10,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );

        let sword_swing = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/sword-effect.png"),
            Vec2::new(32., 32.),
            4,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );

        let flame_thrower_flame = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/flame_effect.png"),
            Vec2::new(48., 48.),
            6,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );

        match self {
            Self::HammerStomp => hammer_stomp,
            Self::FlameThrowerFlame => flame_thrower_flame,
            Self::SwordSwing => sword_swing,
        }
    }
}
