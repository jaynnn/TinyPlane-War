use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub velocity: f32,
    pub acceleration: f32,
    pub timer: Timer,
}

pub fn bullet_setup() {}

pub fn bullet_update(
    mut query: Query<(&mut Transform, Entity, &mut Bullet)>,
    time: Res<Time>,
    mut cmds: Commands,
) {
    for (mut t, e, mut b) in query.iter_mut() {
        b.timer.tick(time.delta());
        if b.timer.finished() {
            cmds.entity(e).despawn();
        }
        let delta_sec = time.delta_seconds();
        b.velocity += b.acceleration * delta_sec;
        t.translation.y += b.velocity * delta_sec;
    }
}