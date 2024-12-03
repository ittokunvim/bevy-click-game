use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use rand::distributions::{Distribution, Uniform};

use crate::{
    WINDOW_SIZE,
    CURSOR_RANGE,
    AppState,
    Config,
};

#[derive(Component)]
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
    config: Res<Config>,
) {
    if !config.setup_ingame { return };

    println!("ball: setup");
    let mut rng = rand::thread_rng();
    let die_color = Uniform::from(0.0..1.0);
    let die_width = Uniform::from(
        -WINDOW_SIZE.x / 2.0 + BALL_SIZE.x..WINDOW_SIZE.x / 2.0 - BALL_SIZE.x
    );
    let die_height = Uniform::from(
        -WINDOW_SIZE.y / 2.0 + BALL_SIZE.y..WINDOW_SIZE.y / 2.0 - BALL_SIZE.y
    );
    let die_z = Uniform::from(0.0..100.0);
    let die_velocity = Uniform::from(-0.5..0.5);

    for _ in 0..BALL_COUNT {
        let ball_color = Color::srgb(
            die_color.sample(&mut rng),
            die_color.sample(&mut rng),
            die_color.sample(&mut rng)
        );
        let ball_pos = Vec3::new(
            die_width.sample(&mut rng),
            die_height.sample(&mut rng),
            die_z.sample(&mut rng),
        );
        let velocity_pos = Vec2::new(
            die_velocity.sample(&mut rng),
            die_velocity.sample(&mut rng),
        );

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(ColorMaterial::from(ball_color)),
                transform: Transform::from_translation(ball_pos).with_scale(BALL_SIZE),
                ..default()
            },
            Ball,
            Velocity(velocity_pos * BALL_SPEED),
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

fn mouse_click(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_event: Res<ButtonInput<MouseButton>>,
    balls_query: Query<(Entity, &Transform), With<Ball>>,
) {
    if !mouse_event.just_pressed(MouseButton::Left) { return }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    cursor_pos = Vec2::new(
        cursor_pos.x - window.width() / 2.0, 
        -cursor_pos.y + window.height() / 2.0,
    );

    for (ball_entity, ball_transform) in balls_query.iter() {
        let ball_pos = ball_transform.translation.truncate();
        let distance = cursor_pos.distance(ball_pos);

        if distance < BALL_SIZE.x - CURSOR_RANGE {
            println!("ball: despawned");
            commands.entity(ball_entity).despawn();
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, apply_velocity.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, check_for_collisions.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, mouse_click.run_if(in_state(AppState::Ingame)));
    }
}
