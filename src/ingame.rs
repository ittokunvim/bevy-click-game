use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use rand::distributions::{Distribution, Uniform};

use crate::{
    WINDOW_SIZE,
    AppState,
    Config,
};

#[derive(Default, Component, Debug)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

const BALL_COUNT: usize = 30;
const BALL_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const BALL_SPEED: f32 = 400.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: &mut Config,
) {
    if config.startup { return };
    config.startup = true;

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
            Ball,
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

fn check_for_collisions(mut query: Query<(&mut Velocity, &Transform), With<Ball>>) {
    for (mut velocity, transform) in query.iter_mut() {
        let size = transform.scale.truncate();
        let left_window_collision =
            WINDOW_SIZE.x / 2.0 < transform.translation.x + size.x / 2.0;
        let right_window_collision =
            -WINDOW_SIZE.x / 2.0 > transform.translation.x - size.x / 2.0;
        let top_window_collision =
            WINDOW_SIZE.y / 2.0 < transform.translation.y + size.y / 2.0;
        let bottom_window_collision =
            -WINDOW_SIZE.y / 2.0 > transform.translation.y - size.y / 2.0;

        if left_window_collision || right_window_collision {
            velocity.x = -velocity.x;
        }
        if top_window_collision || bottom_window_collision {
            velocity.y = -velocity.y;
        }
    }
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        let mut config = Config {
            startup: false,
        };

        app
            .add_systems(
                OnEnter(AppState::Ingame),
                move |
                commands: Commands,
                meshes: ResMut<Assets<Mesh>>,
                materials: ResMut<Assets<ColorMaterial>>
                | {
                    setup(commands, meshes, materials, &mut config);
                }
            )
            .add_systems(Update, apply_velocity.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, check_for_collisions.run_if(in_state(AppState::Ingame)));
    }
}
