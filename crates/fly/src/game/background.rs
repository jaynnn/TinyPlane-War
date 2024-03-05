use bevy::prelude::*;

#[derive(Component)]
pub struct BackGround;

pub fn background_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 0., 0.));
    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 640., 0.));
}

pub fn spawn_background(cmds: &mut Commands, asset_server: &Res<AssetServer>, transform: Transform) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(360., 640.)),
                ..default()
            },
            texture: asset_server.load("images/background/flyback-1.png"),
            transform: transform,
            ..default()
        },
        BackGround
    ));
}

pub fn background_update(
    mut cmds: Commands,
    query: Query<Entity, With<BackGround>>,
    mut query2: Query<(&mut Transform, &Sprite), With<BackGround>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for e in query.iter() {
        let (mut t, s) = query2.get_mut(e).unwrap();
        let move_inter = time.delta_seconds() * 10.;
        // info((t.translation, move_inter));
        t.translation.y -= move_inter;
        let height = s.custom_size.unwrap().y;
        if t.translation.y < -height {
            cmds.entity(e).despawn();
            spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., height, 0.));
        }
    }
}
