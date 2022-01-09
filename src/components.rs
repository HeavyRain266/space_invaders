use bevy::prelude::*;

// Shared types
pub const SCALE: f32 = 0.8;
pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct GetWindowSize {
	pub h: f32,
	pub w: f32
}

// Shared Components
#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
	fn default() -> Self {
		Self(500.0)
	}
}

// Actor loader
#[derive(Component)]
pub struct ActorLoader {
	pub ferris: Handle<Image>,
	pub gopher: Handle<Image>
}

// Laser loader
#[derive(Component)]
pub struct LaserLoader {
	pub red: Handle<Image>
}

// Player components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerReady(pub bool);

// Enemy components
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct ActiveEnemies(pub u32);
