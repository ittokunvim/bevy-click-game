use bevy::prelude::*;

use crate::AppState;

fn setup() {
    println!("gameclear: setup");
}

fn update() {}

fn despawn() {
    println!("gameclear: despawn");
}

pub struct GameclearPlugin;

impl Plugin for GameclearPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameclear), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameclear)))
            .add_systems(OnExit(AppState::Gameclear), despawn)
        ;
    }
}
