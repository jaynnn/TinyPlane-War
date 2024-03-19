use bevy::prelude::*;
use crate::audio::*;
use crate::event::*;

#[derive(Component)]
pub struct MenuPlane(pub f32);

#[derive(Component, Clone, Copy)]
pub enum MenuButtonAction {
    NewGame,
    Continue,
    Settings,
    Credits,
    ReleaseNotes,
    Quit,
}

#[derive(Component)]
pub struct MenuButtonSide;

#[derive(Component)]
struct MainMenuScreen;

pub fn main_menu_setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut event_audio: EventWriter<EventAudioChange>
) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(360., 640.)),
                ..default()
            },
            texture: asset_server.load("images/background/menu_back2.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
    
    let tranform = Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::PI / 2.)).with_translation(Vec3::new(-148., 240., 1.));
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            texture: asset_server.load("images/plane.png"),
            transform: tranform,
            ..default()
        },
        MenuPlane(1.)
    ));

    event_audio.send(EventAudioChange { 
        audio_type: AudioType::BackGround,
        path: "sounds/back/menu_back.mp3".to_string(), 
        settings: PlaybackSettings::LOOP
    });
    
    cmds.spawn((
        NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: RepeatedGridTrack::fr(12, 1.),
                grid_template_rows: RepeatedGridTrack::fr(24, 1.),
                position_type: PositionType::Relative,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba_u8(255, 240, 231, 255)),
            transform: Transform::from_xyz(0., 0., 0.1),
            ..default()
        },
        MainMenuScreen,
    )).with_children(|builder| {
        let font = asset_server.load::<Font>("fonts/ARCADECLASSIC.TTF");
        let button_style = Style {
            display: Display::Grid,
            grid_template_rows: vec![GridTrack::fr(1.), GridTrack::fr(5.), GridTrack::fr(1.)],
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let btn_texts_actions = vec![
            ("New Game",  MenuButtonAction::NewGame),
            ("Continue",  MenuButtonAction::Continue),
            ("Options",  MenuButtonAction::Settings),
            ("Release Notes", MenuButtonAction::Credits),
            ("Credits",  MenuButtonAction::ReleaseNotes),
            ("Quit", MenuButtonAction::Quit),
        ];
    });
}

pub fn main_menu_update(
    mut query_menu_plane: Query<(&mut Transform, &mut MenuPlane)>,
    time: Res<Time>,
) {
    let (mut plane, mut dir) = query_menu_plane.get_single_mut().unwrap();
    plane.translation.x += dir.0 * 128. * time.delta_seconds();
    if plane.translation.x > 148. && dir.0 > 0. {
        plane.rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.);
        dir.0 = -dir.0;
    }
    if plane.translation.x < -148. && dir.0 < 0. {
        plane.rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.);
        dir.0 = -dir.0;
    }
}

pub fn main_menu_button_hovered(
    mut query: Query<(&Interaction, &MenuButtonAction), Changed<Interaction>>,
) {
    for (interaction, btn_action) in query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
            Interaction::Pressed => {

            }
        }
    }
}

pub fn main_menu_button_click(

) {

}

pub fn main_menu_exit(

) {

}
