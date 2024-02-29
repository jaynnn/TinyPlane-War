use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub velocity: f32,
    pub acceleration: f32,
}

pub fn bullet_setup() {}

pub fn bullet_update(
    mut query: Query<(&mut Transform, &mut Bullet)>,
    time: Res<Time>,
) {
    for (mut t, mut b) in query.iter_mut() {
        let delta_sec = time.delta_seconds();
        b.velocity += b.acceleration * delta_sec;
        t.translation.y += b.velocity * delta_sec;
    }
}