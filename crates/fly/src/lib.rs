use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::log::LogPlugin;
use bevy::utils::tracing::Level;

mod game;
mod event;
mod global;
mod menu;
mod stat;
mod audio;
mod helper;

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
            ).set(LogPlugin {
                level: Level::TRACE,
                filter: "".to_string(),
                ..default()
            }))
            .add_plugins((
                stat::game_plugin, 
                stat::menu_plugin,
                audio::audio_plugin,
            ))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                helper::text_blink,
            ));
     }
}

fn setup(
    mut cmds: Commands,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection = OrthographicProjection {    
        far: 1000.,
        near: -1000.,
        scaling_mode: ScalingMode::FixedVertical(788.),
        viewport_origin: Vec2::new(0.7, 0.5),
        ..default()
    };
    cmds.spawn(camera);
}