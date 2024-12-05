use bevy::prelude::*;

mod mainmenu;
mod ingame;
mod gameover;
mod gameclear;

const GAMETITLE: &str = "クリックゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const CURSOR_RANGE: f32 = 10.0;
const BALL_COUNT: usize = 20;
const GAMETIME_LIMIT: f32 = 25.0;
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_MAINMENU: &str = "images/mainmenu.png";
const PATH_IMAGE_PAUSEBUTTON: &str = "images/pausebutton.png";
const PATH_SOUND_BGM: &str = "sounds/bgm.ogg";

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Mainmenu,
    Ingame,
    Pause,
    Gameover,
    Gameclear,
}

#[derive(Resource, Deref, DerefMut, Debug)]
struct Config {
    setup_ingame: bool,
}

#[derive(Resource, Deref, DerefMut, Debug)]
struct BallCount(usize);

#[derive(Resource)]
struct GameTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .init_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(Config { setup_ingame: true })
        .insert_resource(BallCount(BALL_COUNT))
        .insert_resource(GameTimer(
            Timer::from_seconds(GAMETIME_LIMIT, TimerMode::Once)
        ))
        .add_systems(Startup, setup)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(ingame::IngamePlugin)
        .add_plugins(gameover::GameoverPlugin)
        .add_plugins(gameclear::GameclearPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("main: setup");
    // camera
    commands.spawn(Camera2dBundle::default());
    // bgm
    let bgm_sound = asset_server.load(PATH_SOUND_BGM);

    commands.spawn(
        AudioBundle {
            source: bgm_sound,
            settings: PlaybackSettings::LOOP.with_spatial(true),
        }
    )
    .insert(Name::new("bgm"));
}
