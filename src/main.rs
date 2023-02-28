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
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    // spawn camera entity
    commands.spawn(Camera2dBundle::default());

    // slice sprite sheet using texture_handler to get just board
    let texture_handle = asset_server.load("../assets/chess_assets.png"); // 400x248
    let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(160.0, 126.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // spawn chess entity
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        ..default()
    });
}