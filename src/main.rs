// #![allow(unused)] // silence unused warnings while learning

mod enemy;
mod player;

use bevy::prelude::*;
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
