use bevy::{
	prelude::*,
	math::Vec3,
	input::Input
};

use crate::components::*;

// Player plugin for ActorsPlugin
pub struct PlayerActor;

impl Plugin for PlayerActor {
	fn build(
		&self,
		app: &mut App
	) {
		app
			.add_startup_stage(
				"setup_player",
				SystemStage::single(spawn_system)
			)
			.add_system(movement_system)
			.add_system(shooting_system)
			.add_system(laser_system);
	}
}

// Spawn player when game starts
pub fn spawn_system(
	actor: Res<ActorLoader>,
	gws: Res<GetWindowSize>,
	mut cmds: Commands
) {
	let pos_btm =
		-gws.h / 2.0;

	cmds.spawn_bundle (
		SpriteBundle {
			texture: actor.ferris.clone(),
			transform: Transform {
				translation: Vec3::new(0.0, pos_btm + 70.0 / 2.0 + 5.0, 10.0),
				scale: Vec3::new(SCALE, SCALE, 1.1),

				..Default::default()
			},
			..Default::default()
		}
	)
	.insert(Player)
	.insert(PlayerReady(true))
	.insert(Speed::default());
}

// Uses "WASD" for movement
pub fn movement_system(
	input: Res<Input<KeyCode>>,
	mut query: Query<
		(
			&Speed,
			&mut Transform,
		),
		With<Player>
	>
) {
	if let Ok((
		player_speed,
		mut player_trans
	)) = query.get_single_mut() {
			let dir_x =
				if input.pressed(KeyCode::A) {
					-0.7
				} else if input.pressed(KeyCode::D) {
					0.7
				} else {
					0.0
				};
				player_trans.translation.x += dir_x * player_speed.0 * TIME_STEP;

			let dir_y =
				if input.pressed(KeyCode::W) {
					0.7
				} else if input.pressed(KeyCode::S) {
					-0.7
				} else {
					0.0
				};
				player_trans.translation.y += dir_y * player_speed.0 * TIME_STEP;
		}
}

// Spawn laser sprite
pub fn shooting_system(
	input: Res<Input<KeyCode>>,
	laser: Res<LaserLoader>,
	mut cmds: Commands,
	mut query: Query<
		(
			&Transform,
			&mut PlayerReady,
		),
		With<Player>
	>
) {
	if let Ok((
		player_trans,
		mut player_ready
	)) = query.get_single_mut() {
			if player_ready.0 && input.pressed(KeyCode::Space) {
				let x =
					player_trans.translation.x;
				let y =
					player_trans.translation.y;

				cmds.spawn_bundle(
					SpriteBundle {
						sprite: Sprite {
							custom_size: Some(Vec2::new(12.0, 36.0)),

							..Default::default()
						},
						texture: laser.red.clone(),
						transform: Transform {
							translation: Vec3::new(x, y + 50.0, 0.0),

							..Default::default()
						},
						..Default::default()
					}
				)
				.insert(Laser)
				.insert(Speed::default());

				player_ready.0 = false;
			}
			if input.just_released(KeyCode::Space) {
				player_ready.0 = true;
			}
		}
}

// Move laser on screen while shooting
pub fn laser_system(
	gws: Res<GetWindowSize>,
	mut cmds: Commands,
	mut query: Query<
		(
			Entity,
			&Speed,
			&mut Transform,
		),
		With<Laser>
	>
) {
	for (
		red_laser_entity,
		red_laser_speed,
		mut red_laser_trans,
	) in query.iter_mut() {
		let trans =
			&mut red_laser_trans.translation;

			trans.y += red_laser_speed.0 * TIME_STEP;
			if trans.y > gws.h {
				cmds.entity(red_laser_entity).despawn();
			}
	}
}
