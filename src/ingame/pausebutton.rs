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
struct Pausebutton {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return };

    println!("pausebutton: setup");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(IMAGE_SIZE), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = Pausebutton { first: 0, last: 1 };
    let pos = Vec3::new(
        WINDOW_SIZE.x / 2.0 - SIZE,
        -WINDOW_SIZE.y / 2.0 + SIZE,
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
    mut query: Query<(&Transform, &Pausebutton, &mut TextureAtlas), With<Pausebutton>>,
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
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
            println!("pausebutton: toggled");
            atlas.index = prop.last;
            println!("pausebutton: moved state to Pause from Ingame");
            next_state.set(AppState::Pause);
        } else {
            println!("pausebutton: change config.setup_ingame to false");
            config.setup_ingame = false;
            println!("pausebutton: toggled");
            atlas.index = prop.first;
            println!("pausebutton: moved state to Ingame from Pause");
            next_state.set(AppState::Ingame);
        }
    }
}

pub struct PausebuttonPlugin;

impl Plugin for PausebuttonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, update.run_if(in_state(AppState::Pause)));
    }
}
