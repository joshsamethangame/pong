use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Component)]
struct Ball;

#[derive(Resource)]
enum LastScored {
    P1,
    P2,
}
impl Default for LastScored {
    fn default() -> Self {
        Self::P1
    }
}
