use bevy::prelude::*;

use crate::event::*;

#[derive(Component)]
pub struct Score(pub u32);

pub fn score_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load::<Font>("fonts/ARCADECLASSIC.TTF");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 6.18*4.,
        color: Color::linear_rgba(0., 0.05, 0.35, 0.9),
    };
    cmds.spawn((TextBundle::from_sections([
        TextSection::new("Score:", text_style.clone()),
        TextSection::new(" 0", text_style.clone())
        ]).with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default()
        }),
        Score(0),
    ));
}

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