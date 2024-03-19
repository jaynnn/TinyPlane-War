use bevy::prelude::*;

use super::Stat;

use crate::game::*;

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
        plane_setup, 
        enemy_setup,
        bullet_setup,
        background_setup,
        collide_setup,
        score_setup,
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
    cmds: Commands,
) {
    
}

fn game_over(

) {

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