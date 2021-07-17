use crate::{
	ActiveEnemies, Enemy, FromEnemy, Laser, Materials, Speed, WinSize, MAX_ENEMIES, SCALE,
};
use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut bevy::prelude::AppBuilder) {
		app
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1.0))
					.with_system(enemy_spawn.system()),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.9))
					.with_system(enemy_fire.system()),
			);
	}
}

fn enemy_spawn(
	mut commands: Commands,
	mut active_enemies: ResMut<ActiveEnemies>,
	win_size: Res<WinSize>,
	materials: Res<Materials>,
) {
	if active_enemies.0 < MAX_ENEMIES {
		// compute the random position
		let mut rng = thread_rng();
		let w_span = win_size.w / 2. - 100.;
		let h_span = win_size.h / 2. - 100.;
		let x = rng.gen_range(-w_span..w_span) as f32;
		let y = rng.gen_range(-h_span..h_span) as f32;

		// spawn enemy
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.enemy.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.0),
					scale: Vec3::new(SCALE, SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Enemy);

		active_enemies.0 += 1;
	}
}

fn enemy_fire(
	mut commands: Commands,
	materials: Res<Materials>,
	enemy_query: Query<&Transform, With<Enemy>>,
) {
	// for each enemy shoot laser
	for &tf in enemy_query.iter() {
		let x = tf.translation.x;
		let y = tf.translation.y;
		// spawn enemy laser sprite
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.enemy_laser.clone(),
				transform: Transform {
					translation: Vec3::new(x, y - 15., 0.),
					scale: Vec3::new(SCALE, -SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Laser)
			.insert(FromEnemy)
			.insert(Speed::default());
	}
}
