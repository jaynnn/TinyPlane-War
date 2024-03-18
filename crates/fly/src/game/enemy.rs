use bevy::prelude::*;
use bevy::core::FrameCount;
use rand::{self, Rng};

use crate::game::*;
use crate::global::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Event, Default, Debug)]
struct EventEnemyDestory(u32);

pub fn enemy_setup() {}

pub fn enemy_update(
    mut cmds: Commands,
    query: Query<&Sprite, With<BackGround>>,
    mut query2: Query<&mut Transform, With<Enemy>>,
    frame_count: Res<FrameCount>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    if frame_count.0 % 20 == 0 {
        let background_sprite = query.iter().next().unwrap();
        let width = background_sprite.custom_size.unwrap().x;
        let height = background_sprite.custom_size.unwrap().y;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-width/2. + BACKGROUND_EDGE_SIZE .. width/2. - BACKGROUND_EDGE_SIZE);
        let y = rng.gen_range(height/2. .. height/2. + 50.);
        cmds.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(30.)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 1.),
                texture: asset_server.load("images/enemy2.png"),
                ..default()
            },
            Enemy,
        ));
    }
    for mut t in query2.iter_mut() {
        t.translation.y -= time.delta_seconds() * 100.;

    }
}