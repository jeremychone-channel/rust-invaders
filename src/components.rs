use bevy::time::Timer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Component;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
   fn from(val: (f32, f32)) -> Self {
       SpriteSize(Vec2::new(val.0, val.1))
   }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, true))
    }
}


