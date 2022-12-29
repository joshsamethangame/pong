use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::{Bounded, Movable, SysLabel, Velocity, WinSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(startup_system)
            .add_system(player_keyboard_event_system.before(SysLabel::Collision));
    }
}

// constants

pub const PLAYER_HEIGHT: f32 = 200.;
pub const PLAYER_WIDTH: f32 = 40.;

#[derive(PartialEq)]
pub enum Players {
    P1,
    P2,
}

// components

#[derive(Component)]
pub struct Player {
    pub identity: Players,
}

impl Player {
    pub fn shape() -> Vec2 {
        Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)
    }
}

// systems

fn startup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize { w: win_w, h: win_h };

    spawn_player(
        &mut commands,
        &win_size,
        Players::P1,
        win_size.screen_left() + PLAYER_WIDTH / 2.
    );
    spawn_player(
        &mut commands,
        &win_size,
        Players::P2,
        win_size.screen_right() - PLAYER_WIDTH / 2.
    );
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in query.iter_mut() {
        match player.identity {
            Players::P1 => {
                velocity.y = if kb.pressed(KeyCode::S) {
                    -1.
                } else if kb.pressed(KeyCode::W) {
                    1.
                } else {
                    0.
                }
            }
            Players::P2 => {
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

// functions

fn spawn_player(
    commands: &mut Commands,
    win_size: &WinSize,
    identity: Players,
    x: f32) {
    let player_shape = shapes::Rectangle {
        extents: Player::shape(),
        origin: RectangleOrigin::Center,
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &player_shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform {
                translation: Vec3::new(x, 0., 10.),
                ..Default::default()
            },
        ))
        .insert(Player { identity })
        .insert(Movable)
        .insert(Velocity { x: 0., y: 0. })
        .insert(Bounded {
            top: Some(win_size.screen_top() - PLAYER_HEIGHT / 2.),
            bottom: Some(win_size.screen_bottom() + PLAYER_HEIGHT / 2.),
            left: None,
            right: None,
        });
}
