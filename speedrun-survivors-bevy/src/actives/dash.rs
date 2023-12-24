use crate::plugins::gameplay_effects::{
    GameplayEffect, GameplayEffectPluginState, GameplayStat, GameplayTag,
};
use crate::GameAction;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::state::AppState;

pub struct DashPlugin;

impl Plugin for DashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update.run_if(in_state(AppState::GameRunning)));
    }
}

fn on_update(
    actions: Query<&ActionState<GameAction>>,
    mut gameplay_state: ResMut<GameplayEffectPluginState>,
) {
    let action = actions.single();

    if action.pressed(GameAction::Action3)
        && gameplay_state.player_tags.add_tag(GameplayTag::Dash, 2.5)
    {
        gameplay_state.player_effects.apply_temporary(
            vec![GameplayEffect::new_mul(GameplayStat::MovementSpeed, 3.0)],
            0.7,
        );
    }
}
