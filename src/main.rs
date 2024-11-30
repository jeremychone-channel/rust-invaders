#![allow(unused)] // silence unused warnings while exploring (to comment out)

use bevy::math::bounding::IntersectsVolume;
use bevy::math::{bounding::Aabb2d, Vec3Swizzles};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use components::{
	Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, Movable,
	Player, SpriteSize, Velocity,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use std::collections::HashSet;

mod components;
mod enemy;
mod player;

// region:    --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants

// region:    --- Game Constants

const BASE_SPEED: f32 = 500.;

const PLAYER_RESPAWN_DELAY: f64 = 2.;
const ENEMY_MAX: u32 = 2;
const FORMATION_MEMBERS_MAX: u32 = 2;

// endregion: --- Game Constants

// region:    --- Resources
#[derive(Resource)]
pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

#[derive(Resource)]
struct GameTextures {
	player: Handle<Image>,
	player_laser: Handle<Image>,
	enemy: Handle<Image>,
	enemy_laser: Handle<Image>,
	explosion_layout: Handle<TextureAtlasLayout>,
	explosion_texture: Handle<Image>,
}

#[derive(Resource)]
struct EnemyCount(u32);

#[derive(Resource)]
struct PlayerState {
	on: bool,       // alive
	last_shot: f64, // -1 if not shot
}
impl Default for PlayerState {
	fn default() -> Self {
		Self {
			on: false,
			last_shot: -1.,
		}
	}
}

impl PlayerState {
	pub fn shot(&mut self, time: f64) {
		self.on = false;
		self.last_shot = time;
	}
	pub fn spawned(&mut self) {
		self.on = true;
		self.last_shot = -1.;
	}
}
// endregion: --- Resources

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Rust Invaders!".into(),
				resolution: (598., 676.).into(),
				// position window (for tutorial)
				// position: WindowPosition::At(IVec2::new(2780, 4900)),
				..Default::default()
			}),
			..Default::default()
		}))
		.add_plugins(PlayerPlugin)
		.add_plugins(EnemyPlugin)
		.add_systems(Startup, setup_system)
		.add_systems(Update, movable_system)
		.add_systems(Update, player_laser_hit_enemy_system)
		.add_systems(Update, enemy_laser_hit_player_system)
		.add_systems(Update, explosion_to_spawn_system)
		.add_systems(Update, explosion_animation_system)
		.run();
}

fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
	query: Query<&Window, With<PrimaryWindow>>,
) {
	// camera
	commands.spawn(Camera2d);

	// capture window size
	let Ok(primary) = query.get_single() else {
		return;
	};
	let (win_w, win_h) = (primary.width(), primary.height());

	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);

	// create explosion texture atlas
	let texture_handle = asset_server.load(EXPLOSION_SHEET);
	let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 4, None, None);
	let explosion_layout = texture_atlases.add(texture_atlas);

	// add GameTextures resource
	let game_textures = GameTextures {
		player: asset_server.load(PLAYER_SPRITE),
		player_laser: asset_server.load(PLAYER_LASER_SPRITE),
		enemy: asset_server.load(ENEMY_SPRITE),
		enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
		explosion_layout,
		explosion_texture: texture_handle,
	};
	commands.insert_resource(game_textures);
	commands.insert_resource(EnemyCount(0));
}

fn movable_system(
	mut commands: Commands,
	time: Res<Time>,
	win_size: Res<WinSize>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
	let delta = time.delta_secs();

	for (entity, velocity, mut transform, movable) in &mut query {
		let translation = &mut transform.translation;
		translation.x += velocity.x * delta * BASE_SPEED;
		translation.y += velocity.y * delta * BASE_SPEED;

		if movable.auto_despawn {
			// despawn when out of screen
			const MARGIN: f32 = 200.;
			if translation.y > win_size.h / 2. + MARGIN
				|| translation.y < -win_size.h / 2. - MARGIN
				|| translation.x > win_size.w / 2. + MARGIN
				|| translation.x < -win_size.w / 2. - MARGIN
			{
				commands.entity(entity).despawn();
			}
		}
	}
}

#[allow(clippy::type_complexity)] // for the Query types.
fn player_laser_hit_enemy_system(
	mut commands: Commands,
	mut enemy_count: ResMut<EnemyCount>,
	laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
	enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
	let mut despawned_entities: HashSet<Entity> = HashSet::new();

	// iterate through the lasers
	for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
		if despawned_entities.contains(&laser_entity) {
			continue;
		}

		let laser_scale = laser_tf.scale.xy();

		// iterate through the enemies
		for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
			if despawned_entities.contains(&enemy_entity)
				|| despawned_entities.contains(&laser_entity)
			{
				continue;
			}

			let enemy_scale = enemy_tf.scale.xy();

			// determine if collision
			let collision = Aabb2d::new(
				laser_tf.translation.truncate(),
				(laser_size.0 * laser_scale) / 2.,
			)
			.intersects(&Aabb2d::new(
				enemy_tf.translation.truncate(),
				(enemy_size.0 * enemy_scale) / 2.,
			));

			// perform collision
			if collision {
				// remove the enemy
				commands.entity(enemy_entity).despawn();
				despawned_entities.insert(enemy_entity);
				enemy_count.0 -= 1;

				// remove the laser
				commands.entity(laser_entity).despawn();
				despawned_entities.insert(laser_entity);

				// spawn the explosionToSpawn
				commands.spawn(ExplosionToSpawn(enemy_tf.translation));
			}
		}
	}
}

#[allow(clippy::type_complexity)] // for the Query types.
fn enemy_laser_hit_player_system(
	mut commands: Commands,
	mut player_state: ResMut<PlayerState>,
	time: Res<Time>,
	laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
	player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
	if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
		let player_scale = player_tf.scale.xy();

		for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
			let laser_scale = laser_tf.scale.xy();

			// determine if collision
			let collision = Aabb2d::new(
				laser_tf.translation.truncate(),
				(laser_size.0 * laser_scale) / 2.,
			)
			.intersects(&Aabb2d::new(
				player_tf.translation.truncate(),
				(player_size.0 * player_scale) / 2.,
			));

			// perform the collision
			if collision {
				// remove the player
				commands.entity(player_entity).despawn();
				player_state.shot(time.elapsed_secs_f64());

				// remove the laser
				commands.entity(laser_entity).despawn();

				// spawn the explosionToSpawn
				commands.spawn(ExplosionToSpawn(player_tf.translation));

				break;
			}
		}
	}
}

fn explosion_to_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	query: Query<(Entity, &ExplosionToSpawn)>,
) {
	for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
		// spawn the explosion sprite
		commands
			.spawn((
				Sprite {
					image: game_textures.explosion_texture.clone(),
					texture_atlas: Some(TextureAtlas {
						layout: game_textures.explosion_layout.clone(),
						index: 0,
					}),
					..Default::default()
				},
				Transform::from_translation(explosion_to_spawn.0),
			))
			.insert(Explosion)
			.insert(ExplosionTimer::default());

		// despawn the explosionToSpawn
		commands.entity(explosion_spawn_entity).despawn();
	}
}

fn explosion_animation_system(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut ExplosionTimer, &mut Sprite), With<Explosion>>,
) {
	for (entity, mut timer, mut sprite) in &mut query {
		timer.0.tick(time.delta());
		if timer.0.finished() {
			if let Some(texture) = sprite.texture_atlas.as_mut() {
				texture.index += 1;
				if texture.index >= EXPLOSION_LEN {
					commands.entity(entity).despawn();
				}
			}
		}
	}
}
