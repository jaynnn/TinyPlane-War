use bevy::prelude::*;

use super::Stat;

use crate::game::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(Stat::Game), game_setup)
        .add_systems(Update, (
            plane_update, 
            enemy_update, 
            bullet_update,
            background_update,
            collide_update,
            score_update,
        ).run_if(in_state(Stat::Game)))
        .add_systems(OnExit(Stat::Game), game_exit);
}

fn game_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 0., 0.));
    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 640., 0.));
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40., 60.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -100., 1.0),
            texture: asset_server.load("texture/plane.png"),
            ..default()
        },
        Plane
    ));

    let font = asset_server.load::<Font>("fonts/Arimo-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 6.18*4.,
        color: Color::rgba_linear(0., 0.05, 0.35, 0.9),
    };
    cmds.spawn((TextBundle::from_sections([
        TextSection::new("Score: ", text_style.clone()),
        TextSection::new("0", text_style.clone())
        ]).with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default()
        }),
        Score(0),
    ));

    cmds.spawn(AudioBundle {
        source: asset_server.load("sounds/back/normal.mp3"),
        settings: PlaybackSettings::LOOP
    });
}

fn game_exit(

) {

}