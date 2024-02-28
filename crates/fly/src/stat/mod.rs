mod game;
mod menu;

pub use game::*;
pub use menu::*;

use bevy::prelude::*;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum Stat {
    Menu,
    #[default]
    Game,
}