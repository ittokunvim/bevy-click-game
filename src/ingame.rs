use bevy::prelude::*;

use crate::AppState;

fn setup() {
    println!("ingame setup");
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup);
    }
}
