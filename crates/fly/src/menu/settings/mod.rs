use bevy::prelude::*;

mod generic;
mod graphic;
mod input;
mod sound;

pub use generic::*;
pub use graphic::*;
pub use input::*;
pub use sound::*;

#[derive(Default, States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum SettingStat {
    #[default]
    SettingGeneric,
    SettingSound,
    SettingInput,
    SettingGraphic,
}

pub fn settings_plugin(app: &mut App) {
    app
    .add_systems(OnEnter(SettingStat::SettingGeneric), settings_generic_menu_setup)
    .add_systems(OnExit(SettingStat::SettingGeneric), settings_generic_menu_exit)
    .add_systems(OnEnter(SettingStat::SettingSound), settings_sound_menu_setup)
    .add_systems(OnExit(SettingStat::SettingSound), settings_sound_menu_exit)
    .add_systems(OnEnter(SettingStat::SettingInput), settings_inut_menu_setup)
    .add_systems(OnExit(SettingStat::SettingInput), settings_input_menu_exit)
    .add_systems(OnEnter(SettingStat::SettingGraphic), settings_graphic_menu_setup)
    .add_systems(OnExit(SettingStat::SettingGraphic), settings_graphic_menu_exit);
}

pub fn settings_menu_setup() {

}

pub fn settings_menu_button_update() {

}

pub fn settings_menu_exit() {
    
}