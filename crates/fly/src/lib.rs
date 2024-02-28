use bevy::prelude::*;

mod game;
mod event;
mod global;
mod menu;
mod stat;

pub use event::*;

pub struct Fly {}
impl Default for Fly {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for Fly {
     fn build(&self, app: &mut App) {
        app
            .init_state::<stat::Stat>()
            .insert_resource(ClearColor(Color::WHITE))
            .insert_resource(Events::<EventEnemyDestory>::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fly".to_string(),
                        resolution: [360., 640.].into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ))
            .add_plugins((stat::game_plugin, stat::menu_plugin))
            .add_systems(Startup, setup);
     }
}

fn setup(
    mut cmds: Commands,
) {
    cmds.spawn(Camera2dBundle::default());
}
