use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Rectangle;
use crate::WinSize;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_spawn_system);
    }
}

#[derive(Component)]
struct Player;

fn player_spawn_system(
    mut commands: Commands,
    win_size: Res<WinSize>
) {
    let left = -win_size.w / 2.;
    let paddle = Rectangle {
        extents: Vec2::new(20., 40.),
        origin: RectangleOrigin::Center,
    };

    commands.spawn(GeometryBuilder::build_as(
        &paddle,
        DrawMode::Fill(FillMode::color(Color::WHITE)),
        Transform {
            translation: Vec3::new(left, 0., 10.),
            ..Default::default()
        }
    ))
        .insert(Player);
}
