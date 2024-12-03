use bevy::prelude::*;

use crate::{
    AppState,
    GameTimer,
};

fn update(
    mut timer: ResMut<GameTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("timer: moved state to Gameover from Ingame");
        next_state.set(AppState::Gameover);
    }
}

fn reset(
    mut timer: ResMut<GameTimer>,
) {
    println!("timer: reset");
    timer.0.reset();
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Gameover), reset)
            .add_systems(OnExit(AppState::Gameclear), reset)
        ;
    }
}
