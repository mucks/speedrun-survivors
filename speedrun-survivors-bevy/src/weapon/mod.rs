use bevy::prelude::*;

use self::{switch_weapon::SwitchWeaponEvent, weapon_type::WeaponType};

pub mod flame_thrower;
pub mod gun;
pub mod hammer;
pub mod switch_weapon;
pub mod sword;
pub mod weapon_animation_effect;
pub mod weapon_type;

fn spawn_initial_weapon(mut tx_switch: EventWriter<SwitchWeaponEvent>) {
    tx_switch.send(SwitchWeaponEvent {
        weapon_type: WeaponType::default(),
    });
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
