use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::{GameTextures, Laser, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE, PlayerState, SPRITE_SCALE, WinSize};
use crate::components::{FromPlayer, Movable, Player, SpriteSize, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system( player_spawn_system),
            )
            .add_system(player_keyboard_event_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.2))// This was added so that player can hold space and it will shot laser every 0.2 sec!
                    .with_system(player_fire_system),
            );
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let now = time.seconds_since_startup();
    let last_shot = player_state.last_shot;

    if !player_state.on && (last_shot == -1.0 || now > last_shot + PLAYER_RESPAWN_DELAY) {
        let bottom = -win_size.h / 2.0;
        commands.spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE, 10.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Laser)
            .insert(Player)
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Movable { auto_despawn: false })
            .insert(Velocity {x: 0.0, y: 0.0});

        player_state.spawned();
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_texture: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2.0 * SPRITE_SCALE -5.0;

            let mut spawn_laser = |x_offset: f32| {
                commands.spawn_bundle(SpriteBundle {
                    texture: game_texture.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y + 15.0, 0.0),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity {x: 0.0, y: 1.0});
            };
            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -0.5
        } else if kb.pressed(KeyCode::Right) {
            0.5
        } else {
            0.0
        };
        velocity.y = if kb.pressed(KeyCode::Up) {
            0.5
        } else if kb.pressed(KeyCode::Down) {
            -0.5
        } else {
            0.0
        };
    }
}
