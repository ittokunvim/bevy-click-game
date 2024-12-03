use bevy::prelude::*;

mod ball;
mod pausebutton;
mod scoreboard;

const BALL_COUNT: usize = 30;

#[derive(Resource, Deref, DerefMut, Debug)]
struct Ballcount(usize);

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Ballcount(BALL_COUNT))
            .add_plugins(ball::BallPlugin)
            .add_plugins(pausebutton::PausebuttonPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin);
    }
}
