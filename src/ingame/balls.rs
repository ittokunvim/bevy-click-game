use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use rand::distributions::{Distribution, Uniform};

use crate::{
    WINDOW_SIZE,
    CURSOR_RANGE,
    BALL_COUNT,
    AppState,
    Config,
    BallCount,
};

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
    ball_count: Res<BallCount>,
) {
    if !config.setup_ingame { return };

    println!("balls: setup");
    let mut rng = rand::thread_rng();
    let die_width = Uniform::from(
        -WINDOW_SIZE.x / 2.0 + BALL_SIZE.x..WINDOW_SIZE.x / 2.0 - BALL_SIZE.x
    );
    let die_height = Uniform::from(
        -WINDOW_SIZE.y / 2.0 + BALL_SIZE.y..WINDOW_SIZE.y / 2.0 - BALL_SIZE.y
    );
    let die_z = Uniform::from(0.0..100.0);
    let die_velocity = Uniform::from(-0.5..0.5);

    for _ in 0..**ball_count {
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
                material: materials.add(ColorMaterial::from(random_color())),
                transform: Transform::from_translation(ball_pos).with_scale(BALL_SIZE),
                ..Default::default()
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

fn check_wall_collisions(
    mut query: Query<(&Handle<ColorMaterial>, &mut Velocity, &Transform), With<Ball>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (handle, mut velocity, transform) in query.iter_mut() {
        let size = transform.scale.truncate();
        let left_window_collision =
            WINDOW_SIZE.x / 2.0 < transform.translation.x + size.x / 2.0;
        let right_window_collision =
            -WINDOW_SIZE.x / 2.0 > transform.translation.x - size.x / 2.0;
        let top_window_collision =
            WINDOW_SIZE.y / 2.0 < transform.translation.y + size.y / 2.0;
        let bottom_window_collision =
            -WINDOW_SIZE.y / 2.0 > transform.translation.y - size.y / 2.0;

        if left_window_collision
        || right_window_collision
        || top_window_collision
        || bottom_window_collision {
            let color_material: &mut ColorMaterial = materials.get_mut(handle.id()).unwrap();

            if left_window_collision || right_window_collision {
                velocity.x = -velocity.x;
                color_material.color = random_color();
            }
            if top_window_collision || bottom_window_collision {
                velocity.y = -velocity.y;
                color_material.color = random_color();
            }
        }
    }
}

fn check_ball_collisions(
    mut query: Query<(&Handle<ColorMaterial>, &mut Velocity, &Transform), With<Ball>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time_step: Res<Time<Fixed>>,
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([ball1, ball2]) = combinations.fetch_next() {
        let (handle_1, mut velocity_1, transform_1) = ball1;
        let (handle_2, mut velocity_2, transform_2) = ball2;
        let position_1 = transform_1.translation.truncate();
        let position_2 = transform_2.translation.truncate();
        let direction_1 = velocity_1.xy() * time_step.delta().as_secs_f32();
        let direction_2 = velocity_2.xy() * time_step.delta().as_secs_f32();
        let collision = (
            (position_1.x + direction_1.x * 2.0 - position_2.x - direction_2.x * 2.0).powi(2) +
            (position_1.y + direction_1.y * 2.0 - position_2.y - direction_2.y * 2.0).powi(2)
        ) <= BALL_SIZE.x.powi(2);

        if collision {
            let color_material: &mut ColorMaterial = materials.get_mut(handle_1.id()).unwrap();
            color_material.color = random_color();
            let color_material: &mut ColorMaterial = materials.get_mut(handle_2.id()).unwrap();
            color_material.color = random_color();

            velocity_1.x += (direction_2.x - direction_1.x) / time_step.delta().as_secs_f32();
            velocity_1.y += (direction_2.y - direction_1.y) / time_step.delta().as_secs_f32();
            velocity_2.x += (direction_1.x - direction_2.x) / time_step.delta().as_secs_f32();
            velocity_2.y += (direction_1.y - direction_2.y) / time_step.delta().as_secs_f32();
        }
    }
}

fn mouse_click(
    mut commands: Commands,
    mut ball_count: ResMut<BallCount>,
    mut next_state: ResMut<NextState<AppState>>,
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
            println!("balls: ballCount from {} to {}", **ball_count, **ball_count - 1);
            **ball_count -= 1;
            println!("balls: despawn");
            commands.entity(ball_entity).despawn();
            if **ball_count <= 0 {
                println!("balls: moved state to Gameclear from Ingame");
                next_state.set(AppState::Gameclear);
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Ball>>,
) {
    println!("balls: despawn all");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

fn reset_ball_count(
    mut ball_count: ResMut<BallCount>,
) {
    println!("balls: reset ballCount");
    **ball_count = BALL_COUNT;
}

pub struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, apply_velocity.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, check_wall_collisions.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, check_ball_collisions.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, mouse_click.run_if(in_state(AppState::Ingame)))
            .add_systems(OnEnter(AppState::Gameover), despawn)
            .add_systems(OnExit(AppState::Gameover), reset_ball_count)
            .add_systems(OnEnter(AppState::Gameclear), despawn)
            .add_systems(OnExit(AppState::Gameclear), reset_ball_count)
        ;
    }
}

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let die_color = Uniform::from(0.0..1.0);

    Color::srgb(
        die_color.sample(&mut rng),
        die_color.sample(&mut rng),
        die_color.sample(&mut rng)
    )
}
