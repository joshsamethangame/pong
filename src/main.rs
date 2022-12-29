use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::ball::BallPlugin;
use crate::player::PlayerPlugin;

mod player;
mod ball;

// region:    --- Components

#[derive(Component)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Bounded {
    top: Option<f32>,
    bottom: Option<f32>,
    left: Option<f32>,
    right: Option<f32>,
}

// endregion: --- Components

// region:    --- Constants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const FONT_NAME: &str = "MajorMonoDisplay-Regular.ttf";
const TEXT_SIZE: f32 = 60.;

// endregion: --- Constants

// region:    --- Resources

#[derive(Resource)]
struct WinSize {
    w: f32,
    h: f32,
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
    p1: u32,
    p2: u32,
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
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(main_startup_system)
        .add_system(movement_system)
        .add_system(text_update_system)
        .run();
}

fn main_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

    // spawn score text
    commands
        .spawn((
            TextBundle::from_section(
                "0 0",
                TextStyle {
                    font: asset_server.load(FONT_NAME),
                    font_size: TEXT_SIZE,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(win_size.w / 2. - TEXT_SIZE),
                    ..default()
                },
                ..default()
            }),
            ScoreText,
        ));

    // insert resources
    commands.insert_resource(win_size);
    commands.insert_resource(Score { p1: 0, p2: 0 });
}

fn movement_system(
    mut query: Query<(&Velocity, &mut Transform, &Movable, Option<&Bounded>)>,
) {
    for (velocity, mut transform, _, bounded) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if let Some(bounded) = bounded {
            if let Some(top) = bounded.top {
                translation.y = translation.y.min(top);
            }
            if let Some(bottom) = bounded.bottom {
                translation.y = translation.y.max(bottom);
            }
            if let Some(left) = bounded.left {
                translation.x = translation.x.max(left);
            }
            if let Some(right) = bounded.right {
                translation.x = translation.x.min(right);
            }
        }
    }
}

fn text_update_system(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("{} {}", score.p1, score.p2);
    }
}
