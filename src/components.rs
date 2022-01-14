use bevy::prelude::*;

pub struct GetWindowSize {
	pub h: f32,
	pub w: f32
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
	fn default() -> Self {
		Self(500.0)
	}
}

#[derive(Component)]
pub struct ActorLoader {
	pub ferris: Handle<Image>,
	pub gopher: Handle<Image>
}

#[derive(Component)]
pub struct LaserLoader {
	pub red: Handle<Image>
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerReady(pub bool);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct ActiveEnemies(pub u32);
