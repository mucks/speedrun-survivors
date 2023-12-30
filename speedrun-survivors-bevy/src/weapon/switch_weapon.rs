use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::menu::MenuGameConfig;
use crate::plugins::gameplay_effects::{GameplayEffectPluginState, GameplayTag};
use crate::{plugins::assets::GameAssets, state::AppState, GameAction};

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
    mut tx_switch: EventWriter<SwitchWeaponEvent>,
    actions: Query<&ActionState<GameAction>>,
    mut gameplay_state: ResMut<GameplayEffectPluginState>,
) {
    let action = actions.single();

    if action.just_pressed(GameAction::Slot1)
        && gameplay_state.player_tags.add_tag(GameplayTag::Attack, 0.5)
    {
        tx_switch.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Sword,
        });
    }

    if action.just_pressed(GameAction::Slot2)
        && gameplay_state.player_tags.add_tag(GameplayTag::Attack, 0.5)
    {
        tx_switch.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Hammer,
        });
    }

    if action.just_pressed(GameAction::Slot3)
        && gameplay_state.player_tags.add_tag(GameplayTag::Attack, 0.5)
    {
        tx_switch.send(SwitchWeaponEvent {
            weapon_type: WeaponType::Gun,
        });
    }

    if action.just_pressed(GameAction::Slot4)
        && gameplay_state.player_tags.add_tag(GameplayTag::Attack, 0.5)
    {
        tx_switch.send(SwitchWeaponEvent {
            weapon_type: WeaponType::FlameThrower,
        });
    }
}

fn on_switch_weapon(
    mut commands: Commands,
    mut rx_switch: EventReader<SwitchWeaponEvent>,
    mut weapon_query: Query<(&mut Transform, Entity), With<WeaponType>>,
    game_config: Res<MenuGameConfig>,
    game_assets: Res<GameAssets>,
) {
    for switch_weapon_event in rx_switch.iter() {
        println!("Switching weapon to {:?}", switch_weapon_event.weapon_type);

        // delete all weapons
        for (_transform, entity) in weapon_query.iter_mut() {
            println!("Deleting weapon {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }

        // spawn new weapon
        switch_weapon_event
            .weapon_type
            .spawn(&mut commands, &game_config, &game_assets);
    }
}
