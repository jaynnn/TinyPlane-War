use bevy::prelude::*;

use super::Stat;

use crate::game::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(Stat::Game), (
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
        ).run_if(in_state(Stat::Game)))
        .add_systems(OnExit(Stat::Game), game_exit);
}

fn game_setup(
) {

}

fn game_exit(

) {

}