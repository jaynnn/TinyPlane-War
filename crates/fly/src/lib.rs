use bevy::prelude::*;

mod game;
mod event;
mod global;
mod menu;
mod stat;
mod audio;

pub use event::*;

pub struct Fly;

impl Plugin for Fly {
     fn build(&self, app: &mut App) {
        app
            .init_state::<stat::Stat>()
            .insert_resource(ClearColor(Color::WHITE))
            .insert_resource(Events::<EventEnemyDestory>::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fly".to_string(),
                        resolution: [1080., 1440.].into(),
                        resizable: false,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ))
            .add_plugins((
                stat::game_plugin, 
                stat::menu_plugin,
                audio::audio_plugin,
            ))
            .add_systems(Startup, setup);
     }
}

fn setup(
    mut cmds: Commands,
) {
    cmds.spawn(Camera2dBundle::default());
}
