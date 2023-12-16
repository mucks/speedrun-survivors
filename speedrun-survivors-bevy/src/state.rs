use bevy::prelude::*;

/// Component to tag entities as required in certain states only
#[derive(Component, Debug)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

/// Possible Game States
#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default] // TODO - for testing we probably want to switch this to GameRunning :)
    SplashScreen,
    GameCreate,
    GameRunning,
    GameOver,
}
impl AppState {
    pub const GAME_STATES: &[AppState; 3] = &[
        AppState::GameCreate,
        AppState::GameRunning,
        AppState::GameOver,
    ];
    pub fn is_game_state(&self) -> bool {
        AppState::GAME_STATES.contains(self)
    }
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