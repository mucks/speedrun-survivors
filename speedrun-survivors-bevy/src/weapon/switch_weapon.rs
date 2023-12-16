use bevy::prelude::*;

use crate::weapon::{gun::spawn_gun, hammer::spawn_hammer, sword::spawn_sword};

use super::weapon_type::WeaponType;

pub struct SwitchWeaponPlugin;

impl Plugin for SwitchWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwitchWeaponEvent>()
            .add_systems(Update, on_switch_weapon);
    }
}

#[derive(Event)]
pub struct SwitchWeaponEvent {
    pub weapon_type: WeaponType,
}

pub fn on_switch_weapon(
    mut commands: Commands,
    mut switch_weapon_event_reader: EventReader<SwitchWeaponEvent>,
    mut weapon_query: Query<(&mut Transform, Entity), With<WeaponType>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for switch_weapon_event in switch_weapon_event_reader.iter() {
        println!("Switching weapon to {:?}", switch_weapon_event.weapon_type);

        // delete all weapons
        for (_transform, entity) in weapon_query.iter_mut() {
            println!("Deleting weapon {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }

        // spawn new weapon
        switch_weapon_event
            .weapon_type
            .spawn(&mut commands, &asset_server, &mut texture_atlases);
    }
}
