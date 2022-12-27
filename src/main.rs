use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// region:    --- Game Constants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const BALL_RESPAWN_DELAY: f64 = 2.;

// endregion: --- Game Constants

// region:    --- Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
#[derive(Default)]
struct Score {
    pub p1: u32,
    pub p2: u32,
}
// endregion: --- Resources

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_players_system(mut commands: Commands) {
    let shape = RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(200.0),
        ..RegularPolygon::default()
    };

    commands.spawn(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        Transform::default(),
    ));
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pong".to_string(),
                width: 800.,
                height: 600.,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup_system)
        .add_plugin(ShapePlugin)
        .add_startup_system(add_players_system)
        .run();
}
