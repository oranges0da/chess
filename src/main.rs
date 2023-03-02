use bevy::prelude::*;

mod board;
use board::*;

pub const SIZE: f32 = 420.0; // size of screen (lol)
pub const BOARD_SPRITE: &str = "../assets/board.png";
pub const PIECES_SPRITE: &str = "../assets/pieces.png";

fn main() {
    App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {  // window config stuff
			window: WindowDescriptor {
				title: "Chess in Rust!".to_string(),
				width: SIZE,
				height: SIZE,
				..Default::default()
			},
			..Default::default()
		}))
        .add_startup_system(setup)
		.add_plugin(BoardPlugin)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    // spawn camera entity
    commands.spawn(Camera2dBundle::default());
}