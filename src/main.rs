use bevy::prelude::*;

const GAMETITLE: &str = "クリックゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.75, 0.75, 0.75);
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_SOUND_BGM: &str = "sounds/bgm.ogg";

const GAMETITLE_FONT_SIZE: f32 = 40.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const CLICKSTART_TEXT: &str = "クリックしてスタート";
const CLICKSTART_FONT_SIZE: f32 = 20.0;
const CLICKSTART_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const CLICKSTART_PADDING: Val = Val::Px(16.0);

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
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Game title
    commands.spawn(
        TextBundle::from_section(
            GAMETITLE,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: GAMETITLE_FONT_SIZE,
                color: GAMETITLE_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_FONT_SIZE / 2.0),
            ..default()
        }),
    )
    .insert(Name::new("gametitle"));
    // Click Start
    commands.spawn(
        TextBundle::from_section(
            CLICKSTART_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: CLICKSTART_FONT_SIZE,
                color: CLICKSTART_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: CLICKSTART_PADDING,
            bottom: CLICKSTART_PADDING,
            ..default()
        }),
    )
    .insert(Name::new("clickstart"));
    // bgm
    let bgm_sound = asset_server.load(PATH_SOUND_BGM);

    commands.spawn(
        AudioBundle {
            source: bgm_sound,
            settings: PlaybackSettings::LOOP.with_spatial(true),
        }
    );
 }
