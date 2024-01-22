use bevy::{prelude::*, utils::info, core::FrameCount};
use rand::{self, Rng};

pub struct Fly {}
impl Default for Fly {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Component)]
pub struct MyCamera;

#[derive(Component)]
pub struct BackGround;

#[derive(Component)]
pub struct Plane;

#[derive(Component)]
pub struct Bullet {
    pub velocity: f32,
    pub acceleration: f32,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Hp(pub u32);

impl Plugin for Fly {
     fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::WHITE))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fly".to_string(),
                        resolution: [360., 640.].into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                plane_update, 
                enemy_update, 
                bullet_update,
                background_update,
                collide_update,
            ));
     }
}

fn spawn_background(cmds: &mut Commands, asset_server: &Res<AssetServer>, transform: Transform) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(360., 640.)),
                ..default()
            },
            texture: asset_server.load("flyback-1.png"),
            transform: transform,
            ..default()
        },
        BackGround
    ));
}

fn setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    cmds.spawn(Camera2dBundle::default());

    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 0., 0.));
    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 640., 0.));
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40., 60.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -100., 1.0),
            texture: asset_server.load("plane.png"),
            ..default()
        },
        Plane
    ));
}

const PLANE_MOVE_TRANSLATION: f32 = 6.18;
const BACKGROUND_EDGE_SIZE: f32 = 10.0;
fn plane_update(
    mut query: Query<&mut Transform, With<Plane>>,
    query2: Query<&Sprite, With<Plane>>,
    input: Res<Input<KeyCode>>,
    query3: Query<&Sprite, With<BackGround>>,
    mut cmds: Commands,
    fcount: Res<FrameCount>,
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
            KeyCode::A => {
                if plane_transform.translation.x - plane_width - PLANE_MOVE_TRANSLATION > left_edge {
                    plane_transform.translation.x -= PLANE_MOVE_TRANSLATION;
                }
            }
            KeyCode::D => {
                if plane_transform.translation.x + plane_width + PLANE_MOVE_TRANSLATION < right_edge {
                    plane_transform.translation.x += PLANE_MOVE_TRANSLATION;
                }
            }
            KeyCode::W => {
                if plane_transform.translation.y + plane_height + PLANE_MOVE_TRANSLATION < top_edge {
                    plane_transform.translation.y += PLANE_MOVE_TRANSLATION;
                }
            }
            KeyCode::S => {
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
                    color: Color::BLUE,
                    ..default()
                },
                transform: Transform::from_xyz(
                    plane_transform.translation.x, 
                    plane_transform.translation.y + plane_height/2. + 2., 
                    plane_transform.translation.z
                ),
                ..default()
            },
            Bullet {
                velocity: 100.,
                acceleration: 150.
            }
        ));
    }
}

fn enemy_update(
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
                texture: asset_server.load("enemy.png"),
                ..default()
            },
            Enemy,
        ));
    }
    for mut t in query2.iter_mut() {
        t.translation.y -= time.delta_seconds() * 100.;
    }
}

fn bullet_update(
    mut query: Query<(&mut Transform, &mut Bullet)>,
    time: Res<Time>,
) {
    for (mut t, mut b) in query.iter_mut() {
        let delta_sec = time.delta_seconds();
        b.velocity += b.acceleration * delta_sec;
        t.translation.y += b.velocity * delta_sec;
    }
}

fn background_update(
    mut cmds: Commands,
    query: Query<Entity, With<BackGround>>,
    mut query2: Query<(&mut Transform, &Sprite), With<BackGround>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for e in query.iter() {
        let (mut t, s) = query2.get_mut(e).unwrap();
        let move_inter = time.delta_seconds() * 10.;
        info((t.translation, move_inter));
        t.translation.y -= move_inter;
        let height = s.custom_size.unwrap().y;
        if t.translation.y < -height {
            cmds.entity(e).despawn();
            spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., height, 0.));
        }
    }
}

fn rec_is_overlap(r1: (f32, f32, f32, f32), r2: (f32, f32, f32, f32)) -> bool {
    r1.0 - r1.3 < r2.0 + r2.3 &&
    r1.0 + r1.3 > r2.0 - r2.3 &&
    r1.1 - r1.3 < r2.1 + r2.3 &&
    r1.1 + r1.3 > r2.1 - r2.3
}

fn collide_update(
    mut cmds: Commands,
    query1: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
    query2: Query<(Entity, &Transform, &Sprite), With<Plane>>,
    query3: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
) {
    for (be, bt, bs) in query1.iter() {
        for (pe, pt, ps) in query3.iter() {
            if rec_is_overlap(
                (bt.translation.x, bt.translation.y, bs.custom_size.unwrap().x / 2., bs.custom_size.unwrap().y / 2.),
                (pt.translation.x, pt.translation.y, ps.custom_size.unwrap().x / 2., ps.custom_size.unwrap().y / 2.)
            ) {
                cmds.entity(be).despawn();
                cmds.entity(pe).despawn();
            }
        }
    }

    for (be, bt, bs) in query3.iter() {
        for (pe, pt, ps) in query2.iter() {
            if rec_is_overlap(
                (bt.translation.x, bt.translation.y, bs.custom_size.unwrap().x / 2., bs.custom_size.unwrap().y / 2.),
                (pt.translation.x, pt.translation.y, ps.custom_size.unwrap().x / 2., ps.custom_size.unwrap().y / 2.)
            ) {
                cmds.entity(be).despawn();
                cmds.entity(pe).despawn();
            }
        }
    }
}