use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_prototype_lyon::prelude::*;
use crate::{Movable, movement_system, Score, Velocity, WinSize};
use crate::player::{Player, PLAYER_HEIGHT, PLAYER_WIDTH, Players};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_startup_system(startup_system)
            .add_system(ball_track_player_system)
            .add_system(
                ball_wall_collision_system
                    .before(movement_system)
            )
            .add_system(
                start_ball_system
                    .before(score_system)
            )
            .add_system(
                score_system
                    .before(ball_player_collision_system)
            )
            .add_system(ball_player_collision_system
                .before(ball_wall_collision_system)
            );
    }
}

// constants

const BALL_SIZE: f32 = 40.;
const BALL_RESPAWN_DELAY: u64 = 2;

// components

#[derive(Component)]
struct Ball;

impl Ball {
    fn shape() -> Vec2 {
        Vec2::new(BALL_SIZE, BALL_SIZE)
    }
}

#[derive(Component)]
pub struct TrackingPlayer {
    player: Players,
}

#[derive(Component)]
pub struct BallStartTimer {
    timer: Timer,
}

// systems

fn startup_system(
    mut commands: Commands
) {
    let ball_shape = shapes::Rectangle {
        extents: Ball::shape(),
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
        .insert(Ball)
        .insert(TrackingPlayer { player: Players::P1 })
        .insert(Movable)
        .insert(Velocity { x: 0., y: 0. });
    commands
        .spawn(BallStartTimer {
            timer: Timer::new(Duration::from_secs(BALL_RESPAWN_DELAY), TimerMode::Once)
        });
}

fn ball_track_player_system(
    mut ball_query: Query<(&TrackingPlayer, &mut Transform)>,
    player_query: Query<(&Player, &Transform), Without<TrackingPlayer>>,
) {
    if let Ok((ball, mut ball_transform)) = ball_query.get_single_mut() {
        let ball_translation = &mut ball_transform.translation;
        for (player, player_transform) in player_query.iter() {
            let player_translation = player_transform.translation;
            if ball.player == player.identity {
                ball_translation.y = player_translation.y;
                ball_translation.x = 0.;
            }
        }
    }
}

fn ball_wall_collision_system(
    win_size: Res<WinSize>,
    mut ball_query: Query<(&Ball, &mut Transform, &mut Velocity)>,
) {
    let (_, mut transform, mut velocity) = ball_query.single_mut();
    let translation = &mut transform.translation;
    if translation.y + BALL_SIZE / 2. > win_size.screen_top()
        || translation.y - BALL_SIZE / 2. < win_size.screen_bottom() {
        velocity.y = -velocity.y;
    }
}

pub fn start_ball_system(
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
                    Players::P1 => velocity.x = -1.,
                    Players::P2 => velocity.x = 1.,
                }
                velocity.y = 0.;
                commands.entity(ball_entity)
                    .remove::<TrackingPlayer>();
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
    let (entity, _, transform) = query.single_mut();
    let translation = transform.translation;
    if translation.x - BALL_SIZE / 2. < win_size.screen_left() {
        score.p2 += 1;
        commands.entity(entity)
            .insert(TrackingPlayer { player: Players::P2 });
        commands.spawn(BallStartTimer {
            timer: Timer::new(Duration::from_secs(BALL_RESPAWN_DELAY), TimerMode::Once)
        });
    } else if translation.x + BALL_SIZE / 2. > win_size.screen_right() {
        score.p1 += 1;
        commands.entity(entity)
            .insert(TrackingPlayer { player: Players::P1 });
        commands.spawn(BallStartTimer {
            timer: Timer::new(Duration::from_secs(BALL_RESPAWN_DELAY), TimerMode::Once)
        });
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
                Ball::shape(),
                player_transform.translation,
                Player::shape(),
            );

            if collision.is_some() {
                ball_velocity.x *= -1.;
                ball_velocity.y = (ball_transform.translation.y - player_transform.translation.y) / ((BALL_SIZE + PLAYER_HEIGHT) / 2.);

                ball_transform.translation.x = ball_transform.translation.x.max(win_size.screen_left() + PLAYER_WIDTH + BALL_SIZE / 2.);
                ball_transform.translation.x = ball_transform.translation.x.min(win_size.screen_right() - PLAYER_WIDTH - BALL_SIZE / 2.);
            }
        }
    }
}
