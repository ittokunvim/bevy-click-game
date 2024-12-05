use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    GAMETIME_LIMIT,
    PATH_FONT,
    AppState,
    Config,
    GameTimer,
};

const GAMECLEAR_TEXT: &str = "ゲームクリア";
const GAMECLEAR_SIZE: f32 = 32.0;
const TIMER_TEXT: &str = "クリアタイム: ";
const RETRY_TEXT: &str = "リトライ: Key[R]";
const BACKTOTITLE_TEXT: &str = "タイトルに戻る: Key[B]";
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_SIZE: f32 = 20.0;
const TEXT_PADDING: f32 = 50.0;

#[derive(Component)]
struct Gameclear;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    timer: Res<GameTimer>,
) {
    println!("gameclear: setup");
    // gameover
    let top = WINDOW_SIZE.y / 2.0 - GAMECLEAR_SIZE / 2.0 - TEXT_PADDING * 1.5;

    commands.spawn((
        TextBundle::from_section(
            GAMECLEAR_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: GAMECLEAR_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("gameclear"));
    // timer
    let top = WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 - TEXT_PADDING * 0.5;
    let cleartime = GAMETIME_LIMIT - (timer.0.remaining_secs() * 100.0).round() / 100.0;

    commands.spawn((
        TextBundle::from_section(
            format!("{}{}", TIMER_TEXT, cleartime),
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("timer"));
    // retry
    let top = WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING * 0.5;

    commands.spawn((
        TextBundle::from_section(
            RETRY_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("retry"));
    // back to title
    let top = WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING * 1.5;

    commands.spawn((
        TextBundle::from_section(
            BACKTOTITLE_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(top),
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("backtotitle"));
}

fn update(
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |app_state: AppState| {
        println!("gameclear: config setup ingame is true");
        config.setup_ingame = true;
        println!("gameclear: moved state to {:?} from Gameclear", app_state);
        next_state.set(app_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyR => closure(AppState::Ingame),
            KeyCode::KeyB => closure(AppState::Mainmenu),
            _ => {},
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Gameclear>>,
) {
    println!("gameclear: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
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
