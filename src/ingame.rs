use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use rand::distributions::{Distribution, Uniform};

use crate::{
    WINDOW_SIZE,
    AppState,
};

const BALL_COUNT: usize = 30;
const BALL_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Balls
    let mut rng = rand::thread_rng();
    let die_width = Uniform::from(-WINDOW_SIZE.x / 2.0 + BALL_SIZE.x..WINDOW_SIZE.x / 2.0 - BALL_SIZE.x);
    let die_height = Uniform::from(-WINDOW_SIZE.y / 2.0 + BALL_SIZE.y..WINDOW_SIZE.y / 2.0 - BALL_SIZE.y);
    let die_z = Uniform::from(0.0..100.0);
    let die_color = Uniform::from(0.0..1.0);

    for _ in 0..BALL_COUNT {
        let ball_pos_x = die_width.sample(&mut rng);
        let ball_pos_y = die_height.sample(&mut rng);
        let ball_pos_z = die_z.sample(&mut rng);
        let ball_color = Color::srgb(die_color.sample(&mut rng), die_color.sample(&mut rng), die_color.sample(&mut rng));

        commands.spawn(
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(ColorMaterial::from(ball_color)),
                transform: Transform::from_translation(Vec3::new(ball_pos_x, ball_pos_y, ball_pos_z))
                    .with_scale(BALL_SIZE),
                ..default()
            },
        )
        .insert(Name::new("ball"));
    }
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup);
    }
}
