use bevy::prelude::*;

use super::{gun::spawn_gun, hammer::spawn_hammer, sword::spawn_sword};

#[derive(Debug, Clone, Copy, Component, PartialEq)]
pub enum WeaponType {
    Gun,
    Hammer,
    Sword,
}

impl WeaponType {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        match self {
            WeaponType::Hammer => spawn_hammer(commands, asset_server, texture_atlases),
            WeaponType::Sword => spawn_sword(commands, asset_server, texture_atlases),
            WeaponType::Gun => spawn_gun(commands, asset_server, texture_atlases),
        };
    }
}
