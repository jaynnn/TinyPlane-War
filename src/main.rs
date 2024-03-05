use bevy::prelude::*;
use fly::Fly;

fn main() {
    App::new()
        .add_plugins(Fly)
        .run();
}
