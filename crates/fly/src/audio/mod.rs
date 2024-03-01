use std::default;

use bevy::prelude::*;

use crate::event::*;
use crate::global::*;

#[derive(Component, Default, Debug)]
pub enum AudioType {
    #[default]
    BackGround,
    SoundEffects,
    SplatSound,
}

#[derive(Component)]
struct AudioBackGround;

pub fn audio_plugin(app: &mut App) {
    app
    .insert_resource(Events::<EventAudioChange>::default())
    .add_systems(Startup, audio_setup)
    .add_systems(PostUpdate, audio_update);
}

fn audio_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    cmds.spawn((
        AudioBundle {
            source: asset_server.load("sounds/back/normal.mp3"),
            settings: PlaybackSettings::LOOP,
        },
        AudioBackGround
    ));
}

fn audio_update(
    mut event: EventReader<EventAudioChange>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    query_background_audio: Query<Entity, With<AudioBackGround>>,
) {
    for e in event.read() {
        match e.audio_type {
            AudioType::BackGround => {
                let back_audio = query_background_audio.get_single().unwrap();
                cmds.entity(back_audio).insert(
                    AudioBundle {
                        source: asset_server.load(e.path.clone()),
                        settings: e.settings,
                    }
                );
            },
            AudioType::SoundEffects => {
                // Number of sounds effect played simultaneously
                let mut count = 0;
                cmds.spawn((
                    AudioBundle {
                        source: asset_server.load(e.path.clone()),
                        settings: e.settings,
                    },
                ));
                count += 1;
                if count >= AUDIO_PLAY_COUNT_IN_SAME_TIME {
                    break;
                }
            },
            AudioType::SplatSound => {},
        }
    }
}