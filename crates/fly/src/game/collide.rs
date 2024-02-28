use bevy::prelude::*;

use crate::event::*;
use crate::global::*;
use crate::game::*;

pub fn collide_update(
    mut cmds: Commands,
    query1: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
    query2: Query<(Entity, &Transform, &Sprite), With<Plane>>,
    query3: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    mut event_enemy_destory: EventWriter<EventEnemyDestory>,
) {
    for (be, bt, bs) in query1.iter() {
        for (pe, pt, ps) in query3.iter() {
            if rec_is_overlap(
                (bt.translation.x, bt.translation.y, bs.custom_size.unwrap().x / 2., bs.custom_size.unwrap().y / 2.),
                (pt.translation.x, pt.translation.y, ps.custom_size.unwrap().x / 2., ps.custom_size.unwrap().y / 2.)
            ) {
                cmds.entity(be).despawn();
                cmds.entity(pe).despawn();
                event_enemy_destory.send(EventEnemyDestory(1));
            }
        }
    }

    for (be, bt, bs) in query3.iter() {
        for (pe, pt, ps) in query2.iter() {
            if rec_is_overlap(
                (bt.translation.x, bt.translation.y, bs.custom_size.unwrap().x / 2., bs.custom_size.unwrap().y / 2.),
                (pt.translation.x, pt.translation.y, ps.custom_size.unwrap().x / 2., ps.custom_size.unwrap().y / 2.)
            ) {
                cmds.entity(be).despawn();
                cmds.entity(pe).despawn();
            }
        }
    }
}