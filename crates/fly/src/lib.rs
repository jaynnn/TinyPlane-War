use bevy::prelude::*;

mod attitude;
mod background;
mod collide;
mod enemy;
mod plane;
mod score;
mod event;
mod global;
mod bullet;

pub use attitude::*;
pub use background::*;
pub use collide::*;
pub use enemy::*;
pub use plane::*;
pub use score::*;
pub use event::*;
pub use global::*;
pub use bullet::*;

pub struct Fly {}
impl Default for Fly {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for Fly {
     fn build(&self, app: &mut App) {
        app
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
            .add_systems(Startup, setup)
            .add_systems(Update, (
                plane_update, 
                enemy_update, 
                bullet_update,
                background_update,
                collide_update,
                score_update,
            ));
     }
}

fn setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    cmds.spawn(Camera2dBundle::default());

    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 0., 0.));
    spawn_background(&mut cmds, &asset_server, Transform::from_xyz(0., 640., 0.));
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40., 60.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -100., 1.0),
            texture: asset_server.load("texture/plane.png"),
            ..default()
        },
        Plane
    ));

    let font = asset_server.load::<Font>("fonts/Arimo-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 6.18*4.,
        color: Color::rgba_linear(0., 0.05, 0.35, 0.9),
    };
    cmds.spawn((TextBundle::from_sections([
        TextSection::new("Score: ", text_style.clone()),
        TextSection::new("0", text_style.clone())
        ]).with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default()
        }),
        Score(0),
    ));

    cmds.spawn(AudioBundle {
        source: asset_server.load("sounds/back/normal.mp3"),
        settings: PlaybackSettings::LOOP
    });
}
