use bevy::{
	prelude::*,
	math::Vec3,
	core::FixedTimestep,
	sprite::collide_aabb::collide,
};
use rand::*;

use crate::components::*;

// Enemy Plugin for ActorsPlugin
pub struct EnemyActor;

impl Plugin for EnemyActor {
	fn build(
		&self,
		app: &mut App
	) {
		app
			.insert_resource(ActiveEnemies(0))
			.add_system(despawn_system)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1.0))
					.with_system(spawn_system)
			);
	}
}


// Spawns enemy and respawns on death
pub fn spawn_system(
	actor: Res<ActorLoader>,
	gws: Res<GetWindowSize>,
	mut cmds: Commands,
	mut active_enemy: ResMut<ActiveEnemies>,
) {
	if active_enemy.0 < 1 {
		let mut rng =
			thread_rng();

		let w_span =
			gws.w / 2.0 - 100.0;

		let h_span =
			gws.h / 2.0 - 100.0;

		let x =
			rng.gen_range(-w_span..w_span) as f32;

		let y =
			rng.gen_range(-h_span..h_span) as f32;

		cmds.spawn_bundle(
			SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(128.0, 128.0)),

					..Default::default()
				},
				texture: actor.gopher.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.0),
					scale: Vec3::new(SCALE, SCALE, 1.0),

					..Default::default()
				},
				..Default::default()
			}
		).insert(Enemy);

		active_enemy.0 += 1;
	}
}

// Despawn enemy on collision with player's laser
pub fn despawn_system(
	mut cmds: Commands,
	mut red_laser_query: Query<
		(
			Entity,
			&Transform,
			&Sprite,
		),
		With<Laser>,
	>,
	mut enemy_query: Query<
		(
			Entity,
			&Transform,
			&Sprite,
		),
		With<Enemy>,
	>,
	mut active_enemy: ResMut<ActiveEnemies>
) {
	for (
		red_laser_entity,
		red_laser_trans,
		red_laser_sprite
	) in red_laser_query.iter_mut() {
		for (
			enemy_entity,
			enemy_trans,
			enemy_sprite
		) in enemy_query.iter_mut() {
			let red_laser_scale =
				Vec3::truncate(red_laser_trans.scale);

			let enemy_scale =
				Vec3::truncate(enemy_trans.scale);

			let on_collision =
				collide(
					red_laser_trans.translation,
					red_laser_sprite.custom_size.unwrap() * red_laser_scale,
					enemy_trans.translation,
					enemy_sprite.custom_size.unwrap() * enemy_scale,
				);

			if let Some(_) = on_collision {
				cmds
					.entity(enemy_entity)
					.despawn();

				active_enemy.0 -= 1;

				cmds
					.entity(red_laser_entity)
					.despawn();
			}
		}
	}
}
