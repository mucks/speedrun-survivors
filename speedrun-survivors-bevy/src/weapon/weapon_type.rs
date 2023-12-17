use bevy::prelude::*;

use crate::plugins::{assets::GameAssets, menu::GameConfigState};

use super::{
    flame_thrower::spawn_flame_thrower, gun::spawn_gun, hammer::spawn_hammer, sword::spawn_sword,
};

use strum::EnumIter;

#[derive(Debug, Clone, Copy, Hash, Default, Component, PartialEq, Eq, EnumIter)]
pub enum WeaponType {
    Gun,
    Hammer,
    #[default]
    Sword,
    FlameThrower,
}

impl WeaponType {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game_config: &Res<GameConfigState>,
        game_assets: &Res<GameAssets>,
    ) {
        match self {
            WeaponType::Hammer => spawn_hammer(commands, game_config, game_assets),
            WeaponType::Sword => spawn_sword(commands, game_config, game_assets),
            WeaponType::Gun => spawn_gun(commands, asset_server, game_assets),
            WeaponType::FlameThrower => spawn_flame_thrower(commands, game_config, game_assets),
        };
    }

    pub fn texture_atlas(&self, asset_server: &Res<AssetServer>) -> TextureAtlas {
        let hammer = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/hammer.png"),
            Vec2::new(32., 32.),
            3,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );
        let sword = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/sword.png"),
            Vec2::new(32., 32.),
            8,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );
        let gun = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/gun.png"),
            Vec2::new(27., 21.),
            2,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );
        let flamethrower = TextureAtlas::from_grid(
            asset_server.load("sprites/weapon/flamethrower-sheet.png"),
            Vec2::new(176., 142.),
            3,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );

        match self {
            WeaponType::Hammer => hammer,
            WeaponType::Sword => sword,
            WeaponType::Gun => gun,
            WeaponType::FlameThrower => flamethrower,
        }
    }
}
