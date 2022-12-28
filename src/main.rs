use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// region:    --- Components

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

enum PlayerOption {
    P1,
    P2,
}

#[derive(Component)]
struct Player {
    option: PlayerOption,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Movable;

// endregion: --- Components

// region:    --- Constants

const PADDLE_HEIGHT: f32 = 200.;
const PADDLE_WIDTH: f32 = 40.;

const BALL_SIZE: f32 = 40.;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const BALL_RESPAWN_DELAY: f64 = 2.;

// endregion: --- Constants

// region:    --- Resources

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
struct Score {
    pub p1: u32,
    pub p2: u32,
}

// endregion: --- Resources

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pong".to_string(),
                width: 800.,
                height: 600.,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_system)
        .add_system(movable_system)
        .add_system(player_keyboard_event_system)
        .run();
}

fn startup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // create WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };

    // spawn background
    let background_shape = shapes::Rectangle {
        extents: Vec2::new(win_size.w, win_size.h),
        origin: RectangleOrigin::Center,
    };

    commands.spawn(GeometryBuilder::build_as(
        &background_shape,
        DrawMode::Fill(FillMode::color(Color::BLACK)),
        Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        }
    ));

    // spawn players
    let paddle_shape = shapes::Rectangle {
        extents: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
        origin: RectangleOrigin::Center,
    };

    let left = -win_size.w / 2. + PADDLE_WIDTH / 2.;
    let right = win_size.w / 2. - PADDLE_WIDTH / 2.;

    commands
        .spawn(GeometryBuilder::build_as(
            &paddle_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform {
                translation: Vec3::new(left, 0., 10.),
                ..Default::default()
            }
        ))
        .insert((Player { option: PlayerOption::P1 }, Movable, Velocity { x: 0., y: 0. }));
    commands
        .spawn(GeometryBuilder::build_as(
            &paddle_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform {
                translation: Vec3::new(right, 0., 10.),
                ..Default::default()
            }
        ))
        .insert((Player { option: PlayerOption::P2 }, Movable, Velocity { x: 0., y: 0. }));

    // insert resources
    commands.insert_resource(win_size);
    commands.insert_resource(Score { p1: 0, p2: 0 });
}

fn movable_system(
    win_size: Res<WinSize>,
    mut query: Query<(&Velocity, &mut Transform, &Movable, Option<&Player>)>,
) {
    let screen_top = win_size.h / 2.;
    let screen_bottom = -win_size.h / 2.;

    for (velocity, mut transform, _, player) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        // keep paddles in bounds
        if player.is_some() {
            if translation.y + PADDLE_HEIGHT / 2. > screen_top {
                translation.y = screen_top - PADDLE_HEIGHT / 2.;
            } else if translation.y - PADDLE_HEIGHT / 2. < screen_bottom {
                translation.y = screen_bottom + PADDLE_HEIGHT / 2.;
            }
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in query.iter_mut() {
        match player.option {
            PlayerOption::P1 => {
                velocity.y = if kb.pressed(KeyCode::S) {
                    -1.
                } else if kb.pressed(KeyCode::W) {
                    1.
                } else {
                    0.
                }
            }
            PlayerOption::P2 => {
                velocity.y = if kb.pressed(KeyCode::Down) {
                    -1.
                } else if kb.pressed(KeyCode::Up) {
                    1.
                } else {
                    0.
                }
            }
        }
    }
}
