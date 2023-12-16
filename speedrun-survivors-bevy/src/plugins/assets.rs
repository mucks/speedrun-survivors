use bevy::prelude::*;

use crate::weapon::weapon_type::WeaponType;

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub buff_1: UiImage,
    pub weapons: Vec<(WeaponType, UiImage)>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        font: asset_server.load("ui/expanse.otf"),
        buff_1: asset_server.load("ui/buff_1.png").into(),
        weapons: vec![
            (
                WeaponType::Hammer,
                asset_server.load("ui/weapon/hammer-icon.png").into(),
            ),
            (
                WeaponType::Sword,
                asset_server.load("ui/weapon/sword-icon.png").into(),
            ),
            (
                WeaponType::Gun,
                asset_server.load("ui/weapon/gun-icon.png").into(),
            ),
        ],
    });
}
