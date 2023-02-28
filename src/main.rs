use bevy::prelude::*;

const SIZE: f32 = 420.0; // size of screen (lol)

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
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    // spawn camera entity
    commands.spawn(Camera2dBundle::default());

    // load board and spawn necessary sprite bundle for it
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/board.png"),
        transform: Transform::from_scale(Vec3::new(3.25, 3.25, 1.)),
        ..default()
    });
}