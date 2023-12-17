use bevy::prelude::*;

use crate::{plugins::menu::GameConfigState, state::AppState};

use super::weapon_type::WeaponType;

pub struct SwitchWeaponPlugin;

impl Plugin for SwitchWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwitchWeaponEvent>().add_systems(
            Update,
            (on_switch_weapon, switch_weapon_controls).run_if(in_state(AppState::GameRunning)),
        );
    }
}

#[derive(Event)]
pub struct SwitchWeaponEvent {
    pub weapon_type: WeaponType,
}

fn switch_weapon_controls(
    mut switch_weapon_events: EventWriter<SwitchWeaponEvent>,
    scan_code_input: Res<Input<ScanCode>>,
) {
    if scan_code_input.just_pressed(ScanCode(2)) {
        switch_weapon_events.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Sword,
        });
    }

    if scan_code_input.just_pressed(ScanCode(3)) {
        switch_weapon_events.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Hammer,
        });
    }

    if scan_code_input.just_pressed(ScanCode(4)) {
        switch_weapon_events.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Gun,
        });
    }

    if scan_code_input.just_pressed(ScanCode(5)) {
        switch_weapon_events.send(SwitchWeaponEvent {
            weapon_type: WeaponType::FlameThrower,
        });
    }
}

fn on_switch_weapon(
    mut commands: Commands,
    mut switch_weapon_event_reader: EventReader<SwitchWeaponEvent>,
    mut weapon_query: Query<(&mut Transform, Entity), With<WeaponType>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_config: Res<GameConfigState>,
) {
    for switch_weapon_event in switch_weapon_event_reader.iter() {
        println!("Switching weapon to {:?}", switch_weapon_event.weapon_type);

        // delete all weapons
        for (_transform, entity) in weapon_query.iter_mut() {
            println!("Deleting weapon {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }

        // spawn new weapon
        switch_weapon_event.weapon_type.spawn(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            &game_config,
        );
    }
}
