// #![allow(unused)] // silence unused warnings while learning

mod enemy;
mod player;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;

// Entity, Component, System, Resource

// region:    Resources
pub struct Materials {
	player: Handle<ColorMaterial>,
	laser: Handle<ColorMaterial>,
	enemy: Handle<ColorMaterial>,
}
struct WinSize {
	#[allow(unused)]
	w: f32,
	h: f32,
}
struct ActiveEnemies(u32);
// endregion: Resources

// region:    Components
struct Player;
struct PlayerReadyFire(bool);
struct Laser;

struct Enemy;

struct Speed(f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.)
	}
}
// endregion: Components

fn main() {
	App::build()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Rust Invaders!".to_string(),
			width: 598.0,
			height: 676.0,
			..Default::default()
		})
		.insert_resource(ActiveEnemies(0))
		.add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
		.add_plugin(EnemyPlugin)
		.add_startup_system(setup.system())
		.add_system(laser_hit_enemy.system())
		.run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();

	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// create the main resources
	commands.insert_resource(Materials {
		player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
		laser: materials.add(asset_server.load(LASER_SPRITE).into()),
		enemy: materials.add(asset_server.load(ENEMY_SPRITE).into()),
	});
	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height(),
	});

	// position window
	window.set_position(IVec2::new(3870, 4830));
}

fn laser_hit_enemy(
	mut commands: Commands,
	mut laser_query: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
	mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
	mut active_enemies: ResMut<ActiveEnemies>,
) {
	for (laser_entity, laser_tf, laser_sprite, _) in laser_query.iter_mut() {
		for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
			let laser_scale = Vec2::from(laser_tf.scale);
			let enemy_scale = Vec2::from(enemy_tf.scale);
			let collision = collide(
				laser_tf.translation,
				laser_sprite.size * laser_scale,
				enemy_tf.translation,
				enemy_sprite.size * enemy_scale,
			);

			if let Some(_) = collision {
				// remove the enemy
				commands.entity(enemy_entity).despawn();
				active_enemies.0 -= 1;

				// remove the laser
				commands.entity(laser_entity).despawn();
			}
		}
	}
}
