use bevy::prelude::*;

use crate::state::{AppState, ForState};

pub struct CoinRewardsPlugin;

impl Plugin for CoinRewardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
            .add_systems(Update, (on_update).run_if(in_state(AppState::GameRunning)))
            .add_event::<CoinAccumulated>()
            .insert_resource(CoinAccumulator {
                total_coin: 0,
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                coin_rate: 1,
            });
    }
}

fn on_enter_game_running(mut coin_accumulator: ResMut<CoinAccumulator>) {
    coin_accumulator.total_coin = 0;
    coin_accumulator.timer.reset();
}
fn on_exit_game_running(mut commands: Commands) {}

fn on_update(
    mut event_reader: EventReader<CoinAccumulated>,
    mut coin_accumulator: ResMut<CoinAccumulator>,
    time: Res<Time>,
) {
    let mut coins_gained: u64 = 0;
    for ev in event_reader.iter() {
        coins_gained += ev.coin;
    }

    // Issue coins based on timer
    coin_accumulator.timer.tick(time.delta());
    if coin_accumulator.timer.finished() {
        coins_gained += coin_accumulator.coin_rate * coin_accumulator.timer.times_finished_this_tick() as u64;
    }

    // Update the total
    coin_accumulator.total_coin += coins_gained;
}

#[derive(Event)]
pub struct CoinAccumulated {
    pub coin: u64,
}

#[derive(Resource)]
pub struct CoinAccumulator {
    pub total_coin: u64,
    timer: Timer,
    coin_rate: u64,
}