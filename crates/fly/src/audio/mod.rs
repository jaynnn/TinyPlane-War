use bevy::prelude::*;

use crate::event::*;

#[derive(Component)]
enum AudioType {
    BackGround,
    SoundEffects,
    SplatSound,
}

pub fn audio_plugin(app: &mut App) {
    app
    .insert_resource(Events::<EventAudioChange>::default())
    .add_systems(Startup, audio_setup)
    .add_systems(Update, audio_update);
}

fn audio_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    cmds.spawn(AudioBundle {
        source: asset_server.load("sounds/back/normal.mp3"),
        settings: PlaybackSettings::LOOP
    });
}

fn audio_update(
    mut event: EventReader<EventAudioChange>,
) {

}