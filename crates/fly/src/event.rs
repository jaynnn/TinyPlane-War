use bevy::prelude::*;

use crate::audio::*;

#[derive(Event, Default, Debug)]
pub struct EventEnemyDestory(pub u32);

#[derive(Event, Default, Debug)]
pub struct EventAudioChange {
    pub audio_type: AudioType,
    pub path: String,
    pub settings: PlaybackSettings,
}
