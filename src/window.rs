// FIXME: This is not loaded for now,
// for some reason tends to crash DirectX...

use {
	bevy::{
		prelude::Res,
		window::WindowId,
		winit::WinitWindows
	},
	winit::window::Icon
};

const ICON_PATH: &str = r#"assets\window\icon.png"#;

/*
	Dirty hack from Beavy cheat-book
	allow us to access Winit window directly
	in order to setup window against Bevy's WindowDescriptor
*/
pub fn handle_icon(
	windows: Res<WinitWindows>
) {
	 let primary =
	 	windows.get_window(WindowId::primary()).unwrap();

 	let (icon_rgba, icon_width, icon_height) =
	 	{
		 	let image =
			 	image::open(ICON_PATH)
				 .expect("Failed to open icon path")
				 .into_rgba8();

		 	let (width, height) =
			 	image.dimensions();

		 	let rgba =
			 	image.into_raw();

		 	(rgba, width, height)
	 };

 	let icon =
	 	Icon::from_rgba
		 	(
			 	icon_rgba,
			 	icon_width,
			 	icon_height
		 	).unwrap();

 	primary.set_window_icon(Some(icon));
}
