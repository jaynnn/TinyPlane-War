use bevy::prelude::*;

use super::Stat;

use crate::menu::*;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum MenuStat {
    #[default]
    Main,
    Settings,
    SettingGeneric,
    SettingSound,
    SettingInput,
    SettingGraphic,
    Credits,
    ReleaseNotes,
}

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuStat>()
        .add_systems(OnEnter(Stat::Menu), menu_setup)
        .add_systems(OnEnter(MenuStat::Main), main_menu_setup)
        .add_systems(OnExit(MenuStat::Main), main_menu_exit)
        .add_systems(OnEnter(MenuStat::Settings), settings_menu_setup)
        .add_systems(OnExit(MenuStat::Settings), settings_menu_exit)
        .add_systems(OnEnter(MenuStat::SettingGeneric), settings_generic_menu_setup)
        .add_systems(OnExit(MenuStat::SettingGeneric), settings_generic_menu_exit)
        .add_systems(OnEnter(MenuStat::SettingSound), settings_sound_menu_setup)
        .add_systems(OnExit(MenuStat::SettingSound), settings_sound_menu_exit)
        .add_systems(OnEnter(MenuStat::SettingInput), settings_inut_menu_setup)
        .add_systems(OnExit(MenuStat::SettingInput), settings_input_menu_exit)
        .add_systems(OnEnter(MenuStat::SettingGraphic), settings_graphic_menu_setup)
        .add_systems(OnExit(MenuStat::SettingGraphic), settings_graphic_menu_exit)
        .add_systems(OnEnter(MenuStat::Credits), credits_menu_setup)
        .add_systems(OnExit(MenuStat::Credits), credits_graphic_menu_exit)
        .add_systems(OnEnter(MenuStat::ReleaseNotes), release_notes_menu_setup)
        .add_systems(OnExit(MenuStat::ReleaseNotes), release_notes_graphic_menu_exit);
} 