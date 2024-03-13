use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use fly::Fly;

fn main() {
    App::new()
        .add_plugins(Fly)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
