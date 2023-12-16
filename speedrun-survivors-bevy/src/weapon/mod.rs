use bevy::prelude::*;

use crate::state::AppState;

use self::weapon_type::WeaponType;

pub mod gun;
pub mod hammer;
pub mod switch_weapon;
pub mod sword;
pub mod weapon_type;

const INITIAL_WEAPON_TYPE: WeaponType = WeaponType::Sword;

fn spawn_initial_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    INITIAL_WEAPON_TYPE.spawn(&mut commands, &asset_server, &mut texture_atlases);
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hammer::HammerPlugin,
            sword::SwordPlugin,
            switch_weapon::SwitchWeaponPlugin,
        ))
        .add_systems(Startup, spawn_initial_weapon);
    }
}
