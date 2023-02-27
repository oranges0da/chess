use bevy::prelude::*;

const SIZE: f32 = 640.0; // size of screen

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04))) // background color 
		.add_plugins(DefaultPlugins.set(WindowPlugin {  // window config stuff
			window: WindowDescriptor {
				title: "Chess in Rust!".to_string(),
				width: SIZE,
				height: SIZE,
				..Default::default()
			},
			..Default::default()
		}))
        .run();
}