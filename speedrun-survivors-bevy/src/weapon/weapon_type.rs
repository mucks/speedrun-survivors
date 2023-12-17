use bevy::prelude::*;

use crate::plugins::menu::GameConfigState;

use super::{
    flame_thrower::spawn_flame_thrower, gun::spawn_gun, hammer::spawn_hammer, sword::spawn_sword,
};

#[derive(Debug, Clone, Copy, Default, Component, PartialEq)]
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
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        game_config: &Res<GameConfigState>,
    ) {
        match self {
            WeaponType::Hammer => {
                spawn_hammer(commands, asset_server, texture_atlases, game_config)
            }
            WeaponType::Sword => spawn_sword(commands, asset_server, texture_atlases, game_config),
            WeaponType::Gun => spawn_gun(commands, asset_server, texture_atlases),
            WeaponType::FlameThrower => {
                spawn_flame_thrower(commands, asset_server, texture_atlases, game_config)
            }
        };
    }
}
