use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use rand::distributions::{Distribution, Uniform};

use crate::{
    WINDOW_SIZE,
    AppState,
};

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

const BALL_COUNT: usize = 30;
const BALL_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const BALL_SPEED: f32 = 400.0;

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
    let die_velocity = Uniform::from(-0.5..0.5);
    let die_color = Uniform::from(0.0..1.0);

    for _ in 0..BALL_COUNT {
        let ball_pos_x = die_width.sample(&mut rng);
        let ball_pos_y = die_height.sample(&mut rng);
        let ball_pos_z = die_z.sample(&mut rng);
        let ball_velocity_x = die_velocity.sample(&mut rng);
        let ball_velocity_y = die_velocity.sample(&mut rng);
        let ball_color = Color::srgb(die_color.sample(&mut rng), die_color.sample(&mut rng), die_color.sample(&mut rng));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(ColorMaterial::from(ball_color)),
                transform: Transform::from_translation(Vec3::new(ball_pos_x, ball_pos_y, ball_pos_z))
                    .with_scale(BALL_SIZE),
                ..default()
            },
            Velocity(Vec2::new(ball_velocity_x, ball_velocity_y) * BALL_SPEED),
        ))
        .insert(Name::new("ball"));
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time_step: Res<Time<Fixed>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, apply_velocity.run_if(in_state(AppState::Ingame)));
    }
}
