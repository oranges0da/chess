use bevy::prelude::*;

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn board entity
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/board.png"),
        transform: Transform::from_scale(Vec3::new(3.25, 3.25, 1.)),
        ..Default::default()
    });
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_board);
    }
}