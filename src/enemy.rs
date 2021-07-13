use crate::{ActiveEnemies, Enemy, MAX_ENEMIES, Materials, SCALE, WinSize};
use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut bevy::prelude::AppBuilder) {
		app.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(1.0))
				.with_system(enemy_spawn.system()),
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
