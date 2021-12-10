mod setup;
mod plugins {
	pub mod enemy;
	pub mod player;
}

use bevy::{
	prelude::*,
	render::pass::ClearColor
};

use serde::Deserialize;
use ron::de::from_reader;

#[allow(unused)]
use crate::{
	plugins::{
		enemy::EnemyPlugin,
		player::PlayerPlugin
	},
	setup::assets,
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Settings {
	window: Win
}

#[derive(Debug, Deserialize)]
pub struct Win {
	pub title: String,
	pub width: f32,
	pub height: f32,
	pub vsync: bool,
	pub resizable: bool,
	pub cursor_visible: bool
}

fn main() {
	let path =
		format!(
			"{}\\settings\\window.ron",
			env!("CARGO_MANIFEST_DIR")
		);

	let file =
		std::fs::File::open(&path)
			.expect("Failed opening file");

	let config: Settings =
		match from_reader(file) {
			Ok(x) => x,
			Err(e) => {
				println!("Failed to load config: {}", e);

			std::process::exit(1);
		}
	};

    App::build()
        .insert_resource
			(
				WindowDescriptor
					{
						title: config.window.title.into(),
						width: config.window.width,
						height: config.window.height,
						vsync: config.window.vsync,
						resizable: config.window.resizable,
						cursor_visible: config.window.cursor_visible,

						..Default::default()
					}
			)
		.insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(assets.system())
        .add_plugins(DefaultPlugins)
		// .add_plugin(EnemyPlugin)
		.add_plugin(PlayerPlugin)
        .run();
}
