use std::{
	path::Path,
	process::exit
};

use crate::components::*;

use bevy::{
	prelude::*,
	window::WindowMode
};

// Load all assets used in the game
pub fn asset_loader(
	server: Res<AssetServer>,
	mut cmds: Commands,
	mut windows: ResMut<Windows>,
) {
	let win =
		windows
			.get_primary_mut()
			.unwrap();

	// Enable hot reloading
	server
		.watch_for_changes()
		.unwrap();

	cmds.spawn_bundle(OrthographicCameraBundle::new_2d());

	// Load Actors
	cmds.insert_resource(
		ActorLoader {
			ferris: server.load(Path::new("actors/ferris.png")),
			gopher: server.load(Path::new("actors/gopher.png"))
		}
	);

	// Load lasers
	// TODO: Turn laser(s) from images to rectangles
	cmds.insert_resource(
		LaserLoader {
			red: server.load(Path::new("lasers/red.png"))
		}
	);

	// Get window size
	cmds.insert_resource(
		GetWindowSize {
			h: win.height(),
			w: win.width()
		}
	);
}

// Press "Delete" to exit game
pub fn exit_geme(
	input: Res<Input<KeyCode>>
) {
	if input.pressed(KeyCode::Delete) {
		println!("Goodbye!");

		exit(0);
	}
}

// Press "F" to turn on fullscreen mode
// Press "Escape" to turn it off
pub fn toggle_fullscreen(
	input: Res<Input<KeyCode>>,
	mut windows: ResMut<Windows>
) {
	let win =
		windows
			.get_primary_mut()
			.unwrap();

	if input.just_pressed(KeyCode::F) {
		win.set_mode(WindowMode::Fullscreen)
	} else if input.just_pressed(KeyCode::Escape) {
		win.set_mode(WindowMode::Windowed)
	}
}
