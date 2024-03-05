use bevy::ui::widget::UiImageSize;
use bevy::{ecs::query, prelude::*};
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
            background_color: BackgroundColor(Color::srgba_u8(255, 240, 231, 255)),
            transform: Transform::from_xyz(0., 0., 0.1),
            ..default()
        },
        MainMenuScreen,
    )).with_children(|builder| {
        let font = asset_server.load::<Font>("fonts/ARCADECLASSIC.TTF");
        let btn_texture = asset_server.load("images/btn_hover_left.png");
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
        btn_rect_liner(builder, font, btn_texture, button_style, btn_texts_actions, 10);
    });
}

// Create buttons on a continuous grid, length == btn_texts.len()
// 连续的网格上创建按钮, 长度为 btn_texts.len()
fn btn_rect_liner(builder: &mut ChildBuilder, font: Handle<Font>, btn_texture: Handle<Image>, btn_style: Style, btn_texts: Vec<(&str, MenuButtonAction)>, mut st: i16) {
    for (s, menu_action) in btn_texts {
        btn_rect(builder, font.clone(), btn_style.clone(), btn_texture.clone(),s, menu_action, st, st+1);
        st += 1;
    }
}

fn btn_rect(builder: &mut ChildBuilder, font: Handle<Font>, btn_style: Style, btn_texture: Handle<Image>, btn_text: &str, menu_action: MenuButtonAction, start: i16, end: i16) {
    builder.spawn(NodeBundle {
        style: Style {
            justify_self: JustifySelf::Center,
            grid_row: GridPlacement::start_end(start, end),
            grid_column: GridPlacement::start_end(5, 9),
            ..default()
        },
        background_color: BackgroundColor(Color::srgba_u8(255, 240, 231, 255)),
        ..default()
    })
    .with_children(|builder| {
        builder.spawn((ButtonBundle {
            style: btn_style,
            image: UiImage::default().with_color(Color::srgba_u8(255, 240, 231, 255)),
            ..default()
        }, menu_action))
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_rows: vec![GridTrack::fr(1.), GridTrack::fr(5.), GridTrack::fr(1.)],
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|builder| {
                builder.spawn(ImageBundle {
                    style: Style {
                        grid_column: GridPlacement::start_end(1, 2),
                        width: Val::Percent(30.),
                        height: Val::Percent(30.),
                        ..default()
                    },
                    image: UiImage {
                        texture: btn_texture.clone(),
                        ..default()
                    },
                    // visibility: Visibility::Hidden,
                    ..default()
                });
                builder.spawn(ImageBundle {
                    style: Style {
                        grid_column: GridPlacement::start_end(3, 4),
                        width: Val::Percent(30.),
                        height: Val::Percent(30.),
                        ..default()
                    },
                    image: UiImage {
                        flip_x: true,
                        texture: btn_texture,
                        ..default()
                    },
                    // visibility: Visibility::Hidden,
                    ..default()
                });
            })
            .with_children(|builder| {
                builder.spawn(TextBundle {
                    style: Style {
                        grid_column: GridPlacement::start_end(2, 3),
                        ..default()
                    },
                    text: Text::from_section(btn_text, TextStyle {
                        font: font,
                        font_size: 18.,
                        color: Color::BLACK,
                    }),
                    ..default()
                });
            });
        });
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
    println!("hhhhhhhhhhhhhhhhhhh");
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
