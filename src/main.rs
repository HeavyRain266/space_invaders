mod asset_loader;
mod actors;
mod components;

use bevy::{
	prelude::*,
	render::options::{
		Backends,
		WgpuOptions
	},
};

use crate::{
	asset_loader::*,
	actors::*
};

fn main() {
	let mut app = App::new();

	// Setup native Graphics API for each platform
	let platform_api =
		if cfg!(target_os = "windows") {
			Backends::DX12
		} else if cfg!(target_os = "macos") {
			Backends::METAL
		} else if cfg!(target_os = "linux") {
			Backends::VULKAN
		} else {
			panic!("Unsupported platform!");
		};

	// Resources
	app
		.insert_resource(
			WgpuOptions {
    			backends: Some(platform_api),

				..Default::default()
			}
		)
		.insert_resource(
			WindowDescriptor {
				title: "Space Invaders!".into(),
				vsync: true,
				width: 1000.0,
				height: 600.0,

				..Default::default()
			}
		)
		.insert_resource(ClearColor(Color::BLACK));

	// Systems
	app
		.add_system(exit_geme)
		.add_system(toggle_fullscreen)
		.add_startup_system(asset_loader);

	// Plugins
	app
		.add_plugins(ActorsBundle)
		.add_plugins(DefaultPlugins);

	// Run game
	app.run();
}
