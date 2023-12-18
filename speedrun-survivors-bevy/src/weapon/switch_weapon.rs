use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    plugins::{assets::GameAssets, menu::GameConfigState},
    state::AppState,
    GameAction,
};

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
    actions: Query<&ActionState<GameAction>>,
) {
    let action = actions.single();

    if action.just_pressed(GameAction::Slot1) {
        switch_weapon_events.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Sword,
        });
    }

    if action.just_pressed(GameAction::Slot2) {
        switch_weapon_events.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Hammer,
        });
    }

    if action.just_pressed(GameAction::Slot3) {
        switch_weapon_events.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Gun,
        });
    }

    if action.just_pressed(GameAction::Slot4) {
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
    game_config: Res<GameConfigState>,
    game_assets: Res<GameAssets>,
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
            &game_config,
            &game_assets,
        );
    }
}
