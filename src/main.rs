use bevy::prelude::*;

mod mainmenu;

const GAMETITLE: &str = "クリックゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.75, 0.75, 0.75);

const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_MAINMENU: &str = "images/mainmenu.png";
const PATH_SOUND_BGM: &str = "sounds/bgm.ogg";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..default()
                }),
                ..default()
            })
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_plugins(mainmenu::MainmenuPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
