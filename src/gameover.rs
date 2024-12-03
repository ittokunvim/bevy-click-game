use bevy::prelude::*;

use crate::AppState;

fn setup() {
    println!("gameover: setup");
}

fn update() {}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)));
    }
}
