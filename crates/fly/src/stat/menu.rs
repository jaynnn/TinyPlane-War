use bevy::prelude::*;

use super::Stat;

use crate::menu::*;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum MenuStat {
    #[default]
    Main,
    Settings,
    Credits,
    ReleaseNotes,
}


pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuStat>()
        .add_plugins(settings_plugin)
        .add_systems(OnEnter(Stat::Menu), menu_setup)
        .add_systems(OnEnter(MenuStat::Main), main_menu_setup.run_if(in_state(Stat::Menu)))
        .add_systems(Update, (main_menu_update, main_menu_button_hovered, main_menu_button_click).run_if(in_state(MenuStat::Main)).run_if(in_state(Stat::Menu)))
        .add_systems(OnExit(MenuStat::Main), main_menu_exit)
        .add_systems(OnEnter(MenuStat::Settings), settings_menu_setup)
        .add_systems(Update, settings_menu_button_update.run_if(in_state(MenuStat::Settings)))
        .add_systems(OnExit(MenuStat::Settings), settings_menu_exit)
        .add_systems(OnEnter(MenuStat::Credits), credits_menu_setup)
        .add_systems(OnExit(MenuStat::Credits), credits_graphic_menu_exit)
        .add_systems(OnEnter(MenuStat::ReleaseNotes), release_notes_menu_setup)
        .add_systems(OnExit(MenuStat::ReleaseNotes), release_notes_graphic_menu_exit);
}