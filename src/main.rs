use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const GAMETITLE: &str = "クリックゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.75, 0.75, 0.75);
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_SOUND_BGM: &str = "sounds/bgm.ogg";
const PATH_IMAGE_MAINMENU: &str = "images/mainmenu.png";

const MAINMENU_PADDING: f32 = 50.0;
const GAMETITLE_FONT_SIZE: f32 = 40.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const CLICKSTART_TEXT: &str = "クリックしてスタート";
const CLICKSTART_FONT_SIZE: f32 = 20.0;
const CLICKSTART_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const BOARD_SIZE: Vec2 = Vec2::new(280.0, 210.0);
const BOARD_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_FONT_SIZE / 2.0 - MAINMENU_PADDING),
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
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_FONT_SIZE / 2.0 + MAINMENU_PADDING),
            ..default()
        }),
    )
    .insert(Name::new("clickstart"));
    // Mainmenu Board
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(BOARD_SIZE.x, BOARD_SIZE.y))),
        material: materials.add(BOARD_COLOR),
        ..default()
    })
    .insert(Name::new("board"));
    // Background image
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load(PATH_IMAGE_MAINMENU),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
    )
    .insert(Name::new("image"));
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
