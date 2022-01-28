// #![allow(unused)] // silence unused warnings while learning
use bevy::math::Vec3Swizzles;
use bevy::render::texture::ImageType;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use std::collections::HashSet;
use std::path::Path;

mod enemy;
mod player;

// base dir for the _SPRITE
const SPRITE_DIR: &str = "assets";
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
// SHEET will be using the atlas, so, will default with assets/
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const MAX_ENEMIES: u32 = 4;
const MAX_FORMATION_MEMBERS: u32 = 2;
const PLAYER_RESPAWN_DELAY: f64 = 2.;

// region:    Resources
pub struct SpriteInfos {
	player: (Handle<Image>, Vec2),
	player_laser: (Handle<Image>, Vec2),
	enemy: (Handle<Image>, Vec2),
	enemy_laser: (Handle<Image>, Vec2),
	explosion: Handle<TextureAtlas>,
}
struct WinSize {
	#[allow(unused)]
	w: f32,
	h: f32,
}
struct ActiveEnemies(u32);

struct PlayerState {
	on: bool,
	last_shot: f64,
}
impl Default for PlayerState {
	fn default() -> Self {
		Self {
			on: false,
			last_shot: 0.,
		}
	}
}
impl PlayerState {
	fn shot(&mut self, time: f64) {
		self.on = false;
		self.last_shot = time;
	}
	fn spawned(&mut self) {
		self.on = true;
		self.last_shot = 0.
	}
}
// endregion: Resources

// region:    Components
#[derive(Component)]
struct Laser;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct PlayerReadyFire(bool);
#[derive(Component)]
struct FromPlayer;

#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct FromEnemy;

#[derive(Component)]
struct Explosion;
#[derive(Component)]
struct ExplosionToSpawn(Vec3);

#[derive(Component)]
struct Speed(f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.)
	}
}
// endregion: Components

fn main() {
	App::new()
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
		.add_startup_system(setup)
		.add_system(player_laser_hit_enemy.system())
		.add_system(enemy_laser_hit_player.system())
		.add_system(explosion_to_spawn.system())
		.add_system(animate_explosion.system())
		.run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut images: ResMut<Assets<Image>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();

	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// create the main resources
	let texture_handle = asset_server.load(EXPLOSION_SHEET);
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);

	commands.insert_resource(SpriteInfos {
		player: load_image(&mut images, PLAYER_SPRITE),
		player_laser: load_image(&mut images, PLAYER_LASER_SPRITE),
		enemy: load_image(&mut images, ENEMY_SPRITE),
		enemy_laser: load_image(&mut images, ENEMY_LASER_SPRITE),
		explosion: texture_atlases.add(texture_atlas),
	});

	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height(),
	});

	// position window
	// Commented out - when recording tutorial (place as you see fit)
	// window.set_position(IVec2::new(3870, 4830));
}

// Note - With bevy v0.6, load images directly and synchronously to capture size
//        See https://github.com/bevyengine/bevy/pull/3696
fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
	let path = Path::new(SPRITE_DIR).join(path);
	let bytes = std::fs::read(&path).expect(&format!("Cannot find {}", path.display()));
	let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
	let size = image.texture_descriptor.size;
	let size = Vec2::new(size.width as f32, size.height as f32);
	let image_handle = images.add(image);
	(image_handle, size)
}

fn player_laser_hit_enemy(
	mut commands: Commands,
	sprite_infos: Res<SpriteInfos>,
	laser_query: Query<(Entity, &Transform), (With<Laser>, With<FromPlayer>)>,
	enemy_query: Query<(Entity, &Transform), With<Enemy>>,
	mut active_enemies: ResMut<ActiveEnemies>,
) {
	let mut enemies_blasted: HashSet<Entity> = HashSet::new();

	for (player_laser_entity, player_laser_tf) in laser_query.iter() {
		let player_laser_size = sprite_infos.player_laser.1;
		let player_laser_scale = Vec2::from(player_laser_tf.scale.abs().xy());

		for (enemy_entity, enemy_tf) in enemy_query.iter() {
			let enemy_size = sprite_infos.enemy.1;
			let enemy_scale = Vec2::from(enemy_tf.scale.xy());

			let collision = collide(
				player_laser_tf.translation,
				player_laser_size * player_laser_scale,
				enemy_tf.translation,
				enemy_size * enemy_scale,
			);

			if let Some(_) = collision {
				if enemies_blasted.get(&enemy_entity).is_none() {
					// remove the enemy
					commands.entity(enemy_entity).despawn();
					active_enemies.0 -= 1;

					// spawn explosion to spawn
					commands
						.spawn()
						.insert(ExplosionToSpawn(enemy_tf.translation.clone()));

					enemies_blasted.insert(enemy_entity);
				}

				// remove the laser
				commands.entity(player_laser_entity).despawn();
			}
		}
	}
}

fn enemy_laser_hit_player(
	mut commands: Commands,
	sprite_infos: Res<SpriteInfos>,
	mut player_state: ResMut<PlayerState>,
	time: Res<Time>,
	laser_query: Query<(Entity, &Transform), (With<Laser>, With<FromEnemy>)>,
	player_query: Query<(Entity, &Transform), With<Player>>,
) {
	if let Ok((player_entity, player_tf)) = player_query.get_single() {
		let player_size = sprite_infos.player.1;
		let player_scale = Vec2::from(player_tf.scale.xy());

		// for each enemy laser
		for (enemy_laser_entity, enemy_laser_tf) in laser_query.iter() {
			let enemy_laser_scale = Vec2::from(enemy_laser_tf.scale.abs().xy());
			let enemy_laser_size = sprite_infos.enemy_laser.1;

			let collision = collide(
				enemy_laser_tf.translation,
				enemy_laser_size * enemy_laser_scale,
				player_tf.translation,
				player_size * player_scale,
			);

			// process collision
			if let Some(_) = collision {
				// remove the player
				commands.entity(player_entity).despawn();
				player_state.shot(time.seconds_since_startup());
				// remove the laser
				commands.entity(enemy_laser_entity).despawn();
				// spawn the ExplosionToSpawn entity
				commands
					.spawn()
					.insert(ExplosionToSpawn(player_tf.translation.clone()));
			}
		}
	}
}

fn explosion_to_spawn(
	mut commands: Commands,
	query: Query<(Entity, &ExplosionToSpawn)>,
	materials: Res<SpriteInfos>,
) {
	for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: materials.explosion.clone(),
				transform: Transform {
					translation: explosion_to_spawn.0,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Explosion)
			.insert(Timer::from_seconds(0.05, true));

		commands.entity(explosion_spawn_entity).despawn();
	}
}

fn animate_explosion(
	mut commands: Commands,
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<
		(
			Entity,
			&mut Timer,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
		),
		With<Explosion>,
	>,
) {
	for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index += 1;
			if sprite.index == texture_atlas.textures.len() {
				commands.entity(entity).despawn();
			}
		}
	}
}
