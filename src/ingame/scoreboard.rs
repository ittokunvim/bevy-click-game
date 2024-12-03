use bevy::prelude::*;

use crate::{
    PATH_FONT,
    AppState,
    Config,
};
use crate::ingame::{
    Ballcount,
    GameTimer,
};

const BALLCOUNT_TEXT: &str = "ボールのこり: ";
const TIMER_TEXT: &str = " | タイム: ";
const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Component)]
struct ScoreboardUi;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("scoreboard: setup");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                BALLCOUNT_TEXT, 
                TextStyle {
                    font: asset_server.load(PATH_FONT),
                    font_size: TEXT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font: asset_server.load(PATH_FONT),
                    font_size: TEXT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::new(
                TIMER_TEXT, 
                TextStyle {
                    font: asset_server.load(PATH_FONT),
                    font_size: TEXT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font: asset_server.load(PATH_FONT),
                    font_size: TEXT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: TEXT_PADDING,
            left: TEXT_PADDING,
            ..Default::default()
        }),
        ScoreboardUi,
    ));
}

fn update(
    mut query: Query<&mut Text, With<ScoreboardUi>>,
    ballcount: Res<Ballcount>,
    timer: Res<GameTimer>,
) {
    let mut text = query.single_mut();
    // write ballcount and timer
    text.sections[1].value = ballcount.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)));
    }
}
