pub mod enemy;
pub mod player;

use bevy::{
	prelude::*,
	app::PluginGroupBuilder
};

use self::{
	enemy::EnemyActor,
	player::PlayerActor,
};

pub struct ActorsBundle;

impl PluginGroup for ActorsBundle {
	fn build(
		&mut self,
		group: &mut PluginGroupBuilder
	) {
		group
			.add(EnemyActor)
			.add(PlayerActor);
	}
}
