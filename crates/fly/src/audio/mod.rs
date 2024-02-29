use bevy::prelude::*;

pub fn audio_plugin(app: &mut App) {
    app.add_systems(Startup, audio_setup);
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