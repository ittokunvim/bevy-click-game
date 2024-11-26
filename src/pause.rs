use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    WINDOW_SIZE,
    CURSOR_RANGE,
    PATH_IMAGE_PAUSEBUTTON,
    AppState,
    Config,
};

const IMAGE_SIZE: u32 = 64;
const SIZE: f32 = 32.0;

#[derive(Default, Component, Debug)]
struct PauseButton {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: &mut Config,
) {
    if config.startup { return };
    config.startup = true;

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(IMAGE_SIZE), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = PauseButton { first: 0, last: 1 };
    let pos = Vec3::new(
        WINDOW_SIZE.x / 2.0 - SIZE,
        WINDOW_SIZE.y / 2.0 - SIZE,
        10.0
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SIZE)),
                ..default()
            },
            texture: asset_server.load(PATH_IMAGE_PAUSEBUTTON),
            transform: Transform::from_translation(pos),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
    ))
    .insert(Name::new("pausebutton"));
}

fn update(
    mouse_event: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &PauseButton, &mut TextureAtlas), With<PauseButton>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if !mouse_event.just_pressed(MouseButton::Left) { return; }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    let Ok((transform, prop, mut atlas)) = query.get_single_mut() else { return; };
    let pausebutton_pos = transform.translation.truncate();
    // get cursor position
    cursor_pos = Vec2::new(
        cursor_pos.x - WINDOW_SIZE.x / 2.0,
        -cursor_pos.y + WINDOW_SIZE.y / 2.0
    );

    let distance = cursor_pos.distance(pausebutton_pos);

    if distance < SIZE - CURSOR_RANGE {
        if atlas.index == prop.first {
            atlas.index = prop.last;
            app_state.set(AppState::Pause);
        } else {
            atlas.index = prop.first;
            app_state.set(AppState::Ingame);
        }
    }
}

pub struct PauseButtonPlugin;

impl Plugin for PauseButtonPlugin {
    fn build(&self, app: &mut App) {
        let mut config = Config {
            startup: false,
        };

        app.add_systems(
            OnEnter(AppState::Ingame),
            move |
            cmd: Commands,
            asset_server: Res<AssetServer>,
            texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
            | {
                setup(cmd, asset_server, texture_atlas_layouts, &mut config);
            }
        )
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, update.run_if(in_state(AppState::Pause)));
    }
}
