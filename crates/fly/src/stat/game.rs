use bevy::prelude::*;

use super::Stat;

use crate::game::*;
use crate::global::*;
use crate::helper::*;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameStat {
    #[default]
    Gamming,
    Puase,
    GameOver,
}

pub fn game_plugin(app: &mut App) {
    app
    .init_state::<GameStat>()
    .add_systems(OnEnter(Stat::Game), (
        game_setup,
        bullet_setup,
        background_setup,
        collide_setup,
    ))
    .add_systems(Update, (
        plane_update,
        plane_animate,
        enemy_update, 
        bullet_update,
        background_update,
        collide_update,
        score_update,
    ).run_if(in_state(Stat::Game)).run_if(in_state(GameStat::Gamming)))
    .add_systems(OnEnter(GameStat::Gamming), (
        plane_setup,
        enemy_setup,
        score_setup,
    ))
    .add_systems(OnEnter(GameStat::Puase), game_pause_setup)
    .add_systems(Update, game_pause.run_if(in_state(GameStat::Puase)))
    .add_systems(OnExit(GameStat::Puase), game_pause_exit)
    .add_systems(OnEnter(GameStat::GameOver), game_over_setup)
    .add_systems(Update, game_over.run_if(in_state(GameStat::GameOver)))
    .add_systems(OnExit(GameStat::GameOver), game_over_exit)
    .add_systems(OnExit(Stat::Game), game_exit);
}

fn game_pause_setup(

) {

}

fn game_pause(

) {

}

fn game_pause_exit(

) {

}

fn game_over_setup(
    mut cmds: Commands,
    query_camera: Query<&Transform, With<Camera2d>>,
    asset_server: Res<AssetServer>,
) {
    let camera_tansform = query_camera.single();
    cmds.spawn((Text2dBundle {
        text: Text::from_section("PRESS SPACE TO CONTINUE", TextStyle {
            font: asset_server.load("fonts/ARCADECLASSIC.ttf"),
            color: Color::rgba_u8(10, 19, 47, 255),
            font_size: 18.,
            ..default()
        }),
        transform: *camera_tansform,
        ..default()
    }, TextBlink(Timer::from_seconds(0.382, TimerMode::Repeating))));
}

fn game_over(
    mut cmds: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query_enemy: Query<Entity, With<Enemy>>,
    query_bullet: Query<Entity, With<Bullet>>,
    query_score: Query<Entity, With<Score>>,
    mut next_state: ResMut<NextState<GameStat>>,
) {
    if input.just_pressed(KeyCode::Space) {
        despawn_entities_recursive::<Enemy>(&mut cmds, query_enemy);
        despawn_entities_recursive::<Bullet>(&mut cmds, query_bullet);
        despawn_entities_recursive::<Score>(&mut cmds, query_score);
        next_state.set(GameStat::Gamming);
    }

}

fn game_over_exit(

) {

}

fn game_setup(
) {

}

fn game_exit(

) {

}