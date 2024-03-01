use bevy::{ecs::query, prelude::*};
use crate::audio::*;
use crate::event::*;

#[derive(Component)]
pub struct MenuPlane(pub f32);

pub fn main_menu_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut event_audio: EventWriter<EventAudioChange>
) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(360., 640.)),
                ..default()
            },
            texture: asset_server.load("background/menu_back.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
    
    let tranform = Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::PI / 2.)).with_translation(Vec3::new(-148., 240., 1.));
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            texture: asset_server.load("texture/plane.png"),
            transform: tranform,
            ..default()
        },
        MenuPlane(1.)
    ));

    event_audio.send(EventAudioChange { 
        audio_type: AudioType::BackGround,
        path: "sounds/back/menu_back.mp3".to_string(), 
        settings: PlaybackSettings::LOOP
    });


}

pub fn main_menu_update(
    mut query_menu_plane: Query<(&mut Transform, &mut MenuPlane)>,
    time: Res<Time>,
) {
    let (mut plane, mut dir) = query_menu_plane.get_single_mut().unwrap();
    plane.translation.x += dir.0 * 128. * time.delta_seconds();
    if plane.translation.x > 148. && dir.0 > 0. {
        plane.rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.);
        dir.0 = -dir.0;
    }
    if plane.translation.x < -148. && dir.0 < 0. {
        plane.rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.);
        dir.0 = -dir.0;
    }
}

pub fn main_menu_button_update(
    mut query: Query<(&Interaction),
        (Changed<Interaction>, With<Button>)>
) {

}

pub fn main_menu_exit(

) {

}
