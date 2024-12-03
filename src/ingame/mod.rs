use bevy::prelude::*;

mod balls;
mod pausebutton;
mod scoreboard;
mod timer;

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(balls::BallsPlugin)
            .add_plugins(pausebutton::PausebuttonPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
            .add_plugins(timer::TimerPlugin);
    }
}
