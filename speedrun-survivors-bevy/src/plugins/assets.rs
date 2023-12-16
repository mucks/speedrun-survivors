use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub weapon_axe: UiImage,
    pub buff_1: UiImage,
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
        weapon_axe: asset_server.load("ui/weapon_axe.png").into(),
        buff_1: asset_server.load("ui/buff_1.png").into(),
    });
}
