use bevy::prelude::*;

use crate::plugins::menu::GameConfigState;

use self::weapon_type::WeaponType;

pub mod flame_thrower;
pub mod gun;
pub mod hammer;
pub mod switch_weapon;
pub mod sword;
pub mod weapon_animation_effect;
pub mod weapon_type;

fn spawn_initial_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_config: Res<GameConfigState>,
) {
    WeaponType::default().spawn(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &game_config,
    );
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hammer::HammerPlugin,
            sword::SwordPlugin,
            switch_weapon::SwitchWeaponPlugin,
            gun::GunPlugin,
            flame_thrower::FlameThrowerPlugin,
        ))
        .add_systems(Startup, spawn_initial_weapon);
    }
}
