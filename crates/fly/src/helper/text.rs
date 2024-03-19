use bevy::prelude::*;

#[derive(Component)]
pub struct TextBlink(pub Timer);

pub fn text_blink(
    mut query_text_blink: Query<(&mut Visibility, &mut TextBlink)>,
    time: Res<Time>,
) {
    for (mut text_blink_visible, mut text_blink) in query_text_blink.iter_mut() {
        text_blink.0.tick(time.delta());
        if text_blink.0.finished() {
            if *text_blink_visible == Visibility::Visible {
                *text_blink_visible = Visibility::Hidden
            } else {
                *text_blink_visible = Visibility::Visible
            }
        }
    }
}