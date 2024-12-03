use bevy::prelude::*;

mod ball;
mod pausebutton;

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ball::BallPlugin)
            .add_plugins(pausebutton::PausebuttonPlugin);
    }
}
