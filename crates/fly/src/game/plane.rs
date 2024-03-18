use std::time::Duration;

use bevy::prelude::*;
use bevy::core::FrameCount;

use crate::global::*;
use crate::game::*;

#[derive(Component)]
struct PlaneAnimateIndies {
    st: usize,
    ed: usize,
}

#[derive(Component)]
pub struct Plane;

#[derive(Component)]
pub struct PlaneAnimate {
    left: PlaneAnimateIndies,
    right: PlaneAnimateIndies
}

pub fn plane_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_atlas = asset_server.load("images/planeanimate.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(258.), 2, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let place_animate = PlaneAnimate {left: PlaneAnimateIndies {st: 1, ed: 2}, right :  PlaneAnimateIndies {st: 3, ed: 4}};
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40., 60.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -100., 1.0),
            texture: texture_atlas,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        place_animate,
        Plane
    ));
}

pub fn plane_update(
    mut query: Query<&mut Transform, With<Plane>>,
    query2: Query<&Sprite, With<Plane>>,
    input: Res<ButtonInput<KeyCode>>,
    query3: Query<&Sprite, With<BackGround>>,
    mut cmds: Commands,
    fcount: Res<FrameCount>,
    asset_server: Res<AssetServer>,
) {
    let mut plane_transform = query.get_single_mut().unwrap();
    let plane_sprite = query2.get_single().unwrap();
    let background_sprite = query3.iter().next().unwrap();
    let width = background_sprite.custom_size.unwrap().x;
    let height = background_sprite.custom_size.unwrap().y;
    let left_edge = -(width / 2.0) + BACKGROUND_EDGE_SIZE;
    let right_edge = width / 2.0 - BACKGROUND_EDGE_SIZE;
    let top_edge = height / 2.0;
    let bottom_edge = -height / 2.0;
    let plane_width = plane_sprite.custom_size.unwrap().x/2.0;
    let plane_height = plane_sprite.custom_size.unwrap().y/2.0;
    for key in input.get_pressed() {
        match *key {
            KeyCode::KeyA => {
                if plane_transform.translation.x - plane_width - PLANE_MOVE_TRANSLATION > left_edge {
                    plane_transform.translation.x -= PLANE_MOVE_TRANSLATION;
                }
            }
            KeyCode::KeyD => {
                if plane_transform.translation.x + plane_width + PLANE_MOVE_TRANSLATION < right_edge {
                    plane_transform.translation.x += PLANE_MOVE_TRANSLATION;
                }
            }
            KeyCode::KeyW => {
                if plane_transform.translation.y + plane_height + PLANE_MOVE_TRANSLATION < top_edge {
                    plane_transform.translation.y += PLANE_MOVE_TRANSLATION;
                }
            }
            KeyCode::KeyS => {
                if plane_transform.translation.y - plane_height - PLANE_MOVE_TRANSLATION > bottom_edge {
                    plane_transform.translation.y -= PLANE_MOVE_TRANSLATION;
                }
            }
            _ => {}
        }
    }

    if fcount.0 % 15 == 0 {
        cmds.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(5.)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    plane_transform.translation.x, 
                    plane_transform.translation.y + plane_height/2. + 2., 
                    plane_transform.translation.z
                ),
                texture: asset_server.load("images/bullet.png"),
                ..default()
            },
            Bullet {
                velocity: 100.,
                acceleration: 150.,
                timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
            }
        ));
    }
}

pub fn plane_animate(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlaneAnimate, &mut TextureAtlas), With<Plane>>,
) {
    let (indices, mut atlas) = query.single_mut();
    for key in input.get_just_released() {
        match *key {
            KeyCode::KeyA | KeyCode::KeyD => {
                if atlas.index > 0 {
                    atlas.index = 0;
                }
            },
            _ => {}
        }
    }
    
    for key in input.get_pressed() {
        match *key {
            KeyCode::KeyA => {
                if atlas.index < indices.left.ed {
                    atlas.index += 1;
                    println!("{}", atlas.index);
                }
            },
            
            KeyCode::KeyD => {
                if atlas.index < indices.right.st {
                    atlas.index = indices.right.st - 1;
                }
                if atlas.index < indices.right.ed {
                    atlas.index += 1;
                    println!("{}", atlas.index);
                }
            },
            _ => {}
        }
    }
}