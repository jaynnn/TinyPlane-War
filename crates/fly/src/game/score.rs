use bevy::prelude::*;

use crate::event::*;

#[derive(Component)]
pub struct Score(pub u32);

pub fn score_update(
    mut event_enemy_destory: EventReader<EventEnemyDestory>,
    mut query: Query<(&mut Text, &mut Score)>,
) {
    for (mut text, mut score) in query.iter_mut() {
        for add_score in event_enemy_destory.read() {
            println!("{:?}", add_score.0);
            score.0 += add_score.0;
            text.sections[1].value = score.0.to_string();
        }
    }
}