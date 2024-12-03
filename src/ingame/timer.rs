use bevy::prelude::*;

use crate::AppState;
use crate::ingame::GameTimer;

fn update(
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("timer: moved state to Gameover from Ingame");
        next_state.set(AppState::Gameover);
    }
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)));
    }
}
