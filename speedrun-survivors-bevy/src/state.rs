use bevy::prelude::*;

/// Component to tag entities as required in certain states only
#[derive(Component, Debug)]
pub struct ForState<T> {
    pub states: Vec<T>,
}
pub fn for_game_states() -> ForState<AppState> {
    ForState {
        states: vec![
            AppState::GameInitializing,
            AppState::GameRunning,
            AppState::GameLevelUp,
            AppState::GamePaused,
        ],
    }
}

/// Possible Game States
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[cfg_attr(not(feature = "dev"), default)]
    SplashScreen,
    GameMenuMain,
    #[cfg_attr(feature = "dev", default)]
    GameInitializing,
    GameRunning,
    GameLevelUp,
    GamePaused,
    GameOver,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        for state in AppState::variants() {
            app.add_systems(OnEnter(state), state_enter_unspawn::<AppState>);
        }
    }
}

/// Un-spawn entities not required in the new state
fn state_enter_unspawn<T: States>(
    mut commands: Commands,
    state: ResMut<State<T>>,
    query: Query<(Entity, &ForState<T>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.get()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
