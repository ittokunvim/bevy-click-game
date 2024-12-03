use bevy::prelude::*;

mod ball;
mod pausebutton;
mod scoreboard;
mod timer;

const BALL_COUNT: usize = 30;
const GAMETIME_LIMIT: f32 = 10.0;

#[derive(Resource, Deref, DerefMut, Debug)]
struct Ballcount(usize);

#[derive(Resource)]
struct GameTimer(Timer);

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Ballcount(BALL_COUNT))
            .insert_resource(GameTimer(
                Timer::from_seconds(GAMETIME_LIMIT, TimerMode::Once)
            ))
            .add_plugins(ball::BallPlugin)
            .add_plugins(pausebutton::PausebuttonPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
            .add_plugins(timer::TimerPlugin);
    }
}
