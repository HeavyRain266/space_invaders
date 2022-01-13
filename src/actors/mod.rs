pub mod enemy;
pub mod player;

use self::{
	enemy::EnemyActor,
	player::PlayerActor,
};

use bevy::{
	prelude::*,
	app::PluginGroupBuilder
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
