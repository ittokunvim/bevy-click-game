use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    GAMETITLE,
    WINDOW_SIZE,
    PATH_FONT,
    PATH_IMAGE_MAINMENU,
    AppState,
};

const GAMETITLE_FONT_SIZE: f32 = 32.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const CLICKSTART_TEXT: &str = "クリックしてスタート";
const CLICKSTART_FONT_SIZE: f32 = 20.0;
const CLICKSTART_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const TEXT_PADDING: f32 = 50.0;
const BOARD_SIZE: Vec2 = Vec2::new(280.0, 210.0);
const BOARD_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

#[derive(Default, Component, Debug)]
struct Mainmenu;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("mainmenu: setup gametitle");
    commands.spawn((
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
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_FONT_SIZE / 2.0 - TEXT_PADDING),
            ..default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("gametitle"));
    println!("mainmenu: setup clickstart");
    commands.spawn((
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
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_FONT_SIZE / 2.0 + TEXT_PADDING),
            ..default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("clickstart"));
    println!("mainmenu: setup mainmenud");
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(BOARD_SIZE.x, BOARD_SIZE.y))),
            material: materials.add(BOARD_COLOR),
            ..default()
        },
        Mainmenu,
    ))
    .insert(Name::new("board"));
    println!("mainmenu: setup image");
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PATH_IMAGE_MAINMENU),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
        Mainmenu,
    ))
    .insert(Name::new("image"));
}

fn update(
    mouse_event: Res<ButtonInput<MouseButton>>,
    mainmenu_query: Query<Entity, With<Mainmenu>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if mouse_event.just_pressed(MouseButton::Left) {
        println!("mainmenu: mouse clicked");
        println!("mainmenu: despawned entities");
        for mainmenu_entity in mainmenu_query.iter() {
            commands.entity(mainmenu_entity).despawn();
        }
        println!("mainmenu: moved mainmenu -> ingame");
        app_state.set(AppState::Ingame);
    }
}

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Mainmenu), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Mainmenu)));
    }
}
