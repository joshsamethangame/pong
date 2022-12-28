use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_prototype_lyon::prelude::*;

// region:    --- Components

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(PartialEq)]
enum PlayerOption {
    P1,
    P2,
}

#[derive(Component)]
struct Player {
    option: PlayerOption,
}

impl Player {
    fn size() -> Vec2 {
        Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)
    }
}

#[derive(Component)]
struct Ball;

impl Ball {
    fn size() -> Vec2 {
        Vec2::new(BALL_SIZE, BALL_SIZE)
    }
}

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct TrackingPlayer {
    player: PlayerOption,
}

#[derive(Component)]
struct BallStartTimer {
    timer: Timer,
}

#[derive(Component)]
struct ScoreText;

// endregion: --- Components

// region:    --- Constants

const PADDLE_HEIGHT: f32 = 200.;
const PADDLE_WIDTH: f32 = 40.;

const BALL_SIZE: f32 = 40.;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const BALL_RESPAWN_DELAY: u64 = 2;

// endregion: --- Constants

// region:    --- Resources

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

impl WinSize {
    fn screen_top(&self) -> f32 {
        self.h / 2.
    }

    fn screen_bottom(&self) -> f32 {
        -self.h / 2.
    }

    fn screen_left(&self) -> f32 {
        -self.w / 2.
    }

    fn screen_right(&self) -> f32 {
        self.w / 2.
    }
}

#[derive(Resource)]
struct Score {
    pub p1: u32,
    pub p2: u32,
}

// endregion: --- Resources

// region:    --- Labels

#[derive(SystemLabel)]
enum SysLabel {
    Collision,
}

// endregion: --- Labels

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
        .add_system(
            movable_system
                .after(SysLabel::Collision)
        )
        .add_system(player_keyboard_event_system)
        .add_system(ball_track_player_system)
        .add_system(
            ball_wall_collision_system
                .label(SysLabel::Collision)
        )
        .add_system(start_ball_system)
        .add_system(score_system)
        .add_system(
            ball_player_collision_system
                .label(SysLabel::Collision)
        )
        .add_system(text_update_system)
        .run();
}

fn startup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
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
        },
    ));

    // spawn players
    let paddle_shape = shapes::Rectangle {
        extents: Player::size(),
        origin: RectangleOrigin::Center,
    };

    let left = win_size.screen_left() + PADDLE_WIDTH / 2.;
    let right = win_size.screen_right() - PADDLE_WIDTH / 2.;

    commands
        .spawn(GeometryBuilder::build_as(
            &paddle_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform {
                translation: Vec3::new(left, 0., 10.),
                ..Default::default()
            },
        ))
        .insert((Player { option: PlayerOption::P1 }, Movable, Velocity { x: 0., y: 0. }));
    commands
        .spawn(GeometryBuilder::build_as(
            &paddle_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform {
                translation: Vec3::new(right, 0., 10.),
                ..Default::default()
            },
        ))
        .insert((Player { option: PlayerOption::P2 }, Movable, Velocity { x: 0., y: 0. }));

    // spawn ball
    let ball_shape = shapes::Rectangle {
        extents: Ball::size(),
        origin: RectangleOrigin::Center,
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &ball_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform {
                translation: Vec3::new(0., 0., 10.),
                ..Default::default()
            },
        ))
        .insert((Ball, TrackingPlayer { player: PlayerOption::P1 }, Velocity { x: 0., y: 0. }));
    commands
        .spawn(BallStartTimer {
            timer: Timer::new(Duration::from_secs(BALL_RESPAWN_DELAY), TimerMode::Once)
        });

    // spawn score text
    commands
        .spawn((
            TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font: Default::default(),
                }),
                TextSection::new(
                    "   ",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        font: Default::default(),
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font: Default::default(),
                }),
            ]),
            ScoreText,
        ));

    // insert resources
    commands.insert_resource(win_size);
    commands.insert_resource(Score { p1: 0, p2: 0 });
}

fn movable_system(
    win_size: Res<WinSize>,
    mut query: Query<(&Velocity, &mut Transform, &Movable, Option<&Player>)>,
) {
    for (velocity, mut transform, _, player) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        // keep paddles in bounds
        if player.is_some() {
            if translation.y + PADDLE_HEIGHT / 2. > win_size.screen_top() {
                translation.y = win_size.screen_top() - PADDLE_HEIGHT / 2.;
            } else if translation.y - PADDLE_HEIGHT / 2. < win_size.screen_bottom() {
                translation.y = win_size.screen_bottom() + PADDLE_HEIGHT / 2.;
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

fn ball_track_player_system(
    mut ball_query: Query<(&TrackingPlayer, &mut Transform)>,
    player_query: Query<(&Player, &Transform), Without<TrackingPlayer>>,
) {
    if let Ok((ball, mut ball_transform)) = ball_query.get_single_mut() {
        let ball_translation = &mut ball_transform.translation;
        for (player, player_transform) in player_query.iter() {
            if ball.player == player.option {
                ball_translation.y = player_transform.translation.y;
                ball_translation.x = 0.;
            }
        }
    }
}

fn ball_wall_collision_system(
    win_size: Res<WinSize>,
    mut ball_query: Query<(&Ball, &mut Transform, &mut Velocity)>,
) {
    if let Ok((_, mut transform, mut velocity)) = ball_query.get_single_mut() {
        let translation = &mut transform.translation;
        if translation.y + BALL_SIZE / 2. > win_size.screen_top()
            || translation.y - BALL_SIZE / 2. < win_size.screen_bottom() {
            velocity.y = -velocity.y;
        }
    }
}

fn start_ball_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<(Entity, &mut BallStartTimer)>,
    mut ball_query: Query<(Entity, &TrackingPlayer, &mut Velocity)>,
) {
    if let Ok((timer_entity, mut timer)) = timer_query.get_single_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.just_finished() {
            if let Ok((ball_entity, ball, mut velocity)) = ball_query.get_single_mut() {
                match ball.player {
                    PlayerOption::P1 => velocity.x = -1.,
                    PlayerOption::P2 => velocity.x = 1.,
                }
                velocity.y = 0.;
                commands.entity(ball_entity)
                    .remove::<TrackingPlayer>()
                    .insert(Movable);
            }
            commands.entity(timer_entity)
                .despawn();
        }
    }
}

fn score_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut score: ResMut<Score>,
    mut query: Query<(Entity, &Ball, &Transform)>,
) {
    if let Ok((entity, _, transform)) = query.get_single_mut() {
        let translation = transform.translation;
        if translation.x - BALL_SIZE / 2. < win_size.screen_left() {
            score.p2 += 1;
            commands.entity(entity)
                .remove::<Movable>()
                .insert(TrackingPlayer { player: PlayerOption::P2 });
            commands.spawn(BallStartTimer {
                timer: Timer::new(Duration::from_secs(BALL_RESPAWN_DELAY), TimerMode::Once)
            });
        } else if translation.x + BALL_SIZE / 2. > win_size.screen_right() {
            score.p1 += 1;
            commands.entity(entity)
                .remove::<Movable>()
                .insert(TrackingPlayer { player: PlayerOption::P1 });
            commands.spawn(BallStartTimer {
                timer: Timer::new(Duration::from_secs(BALL_RESPAWN_DELAY), TimerMode::Once)
            });
        }
    }
}

fn ball_player_collision_system(
    win_size: Res<WinSize>,
    mut ball_query: Query<(&Ball, &mut Transform, &mut Velocity)>,
    player_query: Query<(&Player, &Transform), Without<Ball>>,
) {
    if let Ok((_, mut ball_transform, mut ball_velocity)) = ball_query.get_single_mut() {
        for (_, player_transform) in player_query.iter() {
            let collision = collide(
                ball_transform.translation,
                Ball::size(),
                player_transform.translation,
                Player::size(),
            );

            if collision.is_some() {
                ball_velocity.x *= -1.;
                ball_velocity.y = (ball_transform.translation.y - player_transform.translation.y) / ((BALL_SIZE + PADDLE_HEIGHT) / 2.);

                ball_transform.translation.x = ball_transform.translation.x.max(win_size.screen_left() + PADDLE_WIDTH + BALL_SIZE / 2.);
                ball_transform.translation.x = ball_transform.translation.x.min(win_size.screen_right() - PADDLE_WIDTH - BALL_SIZE / 2.);
            }
        }
    }
}

fn text_update_system(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", score.p1);
        text.sections[2].value = format!("{}", score.p2);
    }
}
