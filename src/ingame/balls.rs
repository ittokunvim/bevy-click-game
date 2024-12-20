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
    PATH_SOUND_DESPAWN,
    AppState,
    Config,
    BallCount,
};

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

#[derive(Resource, Deref)]
struct DespawnSound(Handle<AudioSource>);

#[derive(Event, Default)]
struct DespawnEvent;

const SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const VELOCITY: f32 = 2.0;
const SPEED: f32 = 120.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    ball_count: Res<BallCount>,
) {
    if !config.setup_ingame { return };

    println!("balls: setup");
    // despawn sound
    let despawn_sound = asset_server.load(PATH_SOUND_DESPAWN);
    commands.insert_resource(DespawnSound(despawn_sound));
    // balls
    let mut rng = rand::thread_rng();
    let ball_positions = set_ball_positions();
    let die_velocity = Uniform::from(-VELOCITY..VELOCITY);

    if ball_positions.len() < **ball_count { error!("ball_positions is not long enough.") }

    for i in 0..**ball_count {
        let ball_pos = ball_positions[i];
        let velocity_pos = Vec2::new(
            die_velocity.sample(&mut rng),
            die_velocity.sample(&mut rng),
        );

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(ColorMaterial::from(random_color())),
                transform: Transform::from_translation(ball_pos).with_scale(SIZE),
                ..Default::default()
            },
            Ball,
            Velocity(velocity_pos * SPEED),
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
            color_material.color = random_color();

            if left_window_collision || right_window_collision { velocity.x = -velocity.x }
            if top_window_collision || bottom_window_collision { velocity.y = -velocity.y }
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
        ) <= SIZE.x.powi(2);

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
    mut despawn_events: EventWriter<DespawnEvent>,
    mut ball_count: ResMut<BallCount>,
    mut next_state: ResMut<NextState<AppState>>,
    mouse_events: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ball_query: Query<(Entity, &Transform), With<Ball>>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    cursor_pos = Vec2::new(
        cursor_pos.x - window.width() / 2.0, 
        -cursor_pos.y + window.height() / 2.0,
    );

    for (ball_entity, ball_transform) in ball_query.iter() {
        let ball_pos = ball_transform.translation.truncate();
        let distance = cursor_pos.distance(ball_pos);

        if distance < SIZE.x - CURSOR_RANGE {
            println!("balls: despawn ball from {} to {}", **ball_count, **ball_count - 1);
            despawn_events.send_default();
            **ball_count -= 1;
            commands.entity(ball_entity).despawn();
            if **ball_count <= 0 {
                println!("balls: moved state to Gameclear from Ingame");
                next_state.set(AppState::Gameclear);
            }
        }
    }
}

fn play_despawn_sound(
    mut commands: Commands,
    mut events: EventReader<DespawnEvent>,
    sound: Res<DespawnSound>,
) {
    if events.is_empty() { return }
    events.clear();
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
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
            .add_event::<DespawnEvent>()
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                apply_velocity,
                check_wall_collisions,
                check_ball_collisions,
                mouse_click,
                play_despawn_sound,
            ).run_if(in_state(AppState::Ingame)))
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

fn set_ball_positions() -> Vec<Vec3> {
    vec![
        Vec3::new(SIZE.x *  0.0, SIZE.y *  0.0,  0.0),
        Vec3::new(SIZE.x *  0.0, SIZE.y *  1.0,  1.0),
        Vec3::new(SIZE.x *  1.0, SIZE.y *  1.0,  2.0),
        Vec3::new(SIZE.x *  1.0, SIZE.y *  0.0,  3.0),
        Vec3::new(SIZE.x *  1.0, SIZE.y * -1.0,  4.0),
        Vec3::new(SIZE.x *  0.0, SIZE.y * -1.0,  5.0),
        Vec3::new(SIZE.x * -1.0, SIZE.y * -1.0,  6.0),
        Vec3::new(SIZE.x * -1.0, SIZE.y *  0.0,  7.0),
        Vec3::new(SIZE.x * -1.0, SIZE.y *  1.0,  8.0),
        Vec3::new(SIZE.x * -1.0, SIZE.y *  2.0,  8.0),
        Vec3::new(SIZE.x *  0.0, SIZE.y *  2.0,  9.0),
        Vec3::new(SIZE.x *  1.0, SIZE.y *  2.0, 10.0),
        Vec3::new(SIZE.x *  2.0, SIZE.y *  2.0, 11.0),
        Vec3::new(SIZE.x *  2.0, SIZE.y *  1.0, 12.0),
        Vec3::new(SIZE.x *  2.0, SIZE.y *  0.0, 13.0),
        Vec3::new(SIZE.x *  2.0, SIZE.y * -1.0, 14.0),
        Vec3::new(SIZE.x *  2.0, SIZE.y * -2.0, 15.0),
        Vec3::new(SIZE.x *  1.0, SIZE.y * -2.0, 16.0),
        Vec3::new(SIZE.x *  0.0, SIZE.y * -2.0, 17.0),
        Vec3::new(SIZE.x * -1.0, SIZE.y * -2.0, 18.0),
        Vec3::new(SIZE.x * -2.0, SIZE.y * -2.0, 19.0),
        Vec3::new(SIZE.x * -2.0, SIZE.y * -1.0, 20.0),
    ]
}
