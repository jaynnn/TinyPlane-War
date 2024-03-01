use bevy::prelude::*;


#[derive(Event, Default, Debug)]
pub struct EventEnemyDestory(pub u32);

#[derive(Event, Default, Debug)]
pub struct EventAudioChange {
    path: String,
    settings: PlaybackSettings,
}