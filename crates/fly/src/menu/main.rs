use bevy::{ecs::query, prelude::*};

pub fn main_menu_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
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
}

pub fn main_menu_update(

) {

}

pub fn main_menu_button_update(
    mut query: Query<(&Interaction),
        (Changed<Interaction>, With<Button>)>
) {

}

pub fn main_menu_exit(

) {

}
