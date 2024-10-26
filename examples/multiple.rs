use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::{prelude::*, TouchStickUiKnob, TouchStickUiOutline};
use std::f32::consts::PI;

// ID for joysticks
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
enum Stick {
    #[default]
    Left,
    Right,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            TouchStickPlugin::<Stick>::default(),
        ))
        .add_systems(Startup, create_scene)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player {
    pub max_speed: f32,
}

fn create_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 5.0),
        ..default()
    });

    commands
        .spawn((
            Player { max_speed: 50. },
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(30., 50.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // pointy "nose" for player
            parent.spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(15., 0., 0.),
                    rotation: Quat::from_rotation_z(PI / 4.),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(50. / f32::sqrt(2.))),
                    ..default()
                },
                ..default()
            });
        });

    // Note: you don't have to spawn these parented to an interface node this
    // just allows you too hide the controls easier. For instance, if you pause
    // the game, you can just hide your game controls via the root node's
    // `Style` component.
    commands
        .spawn((
            Name::new("TouchControlsRoot"),
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("LeftTouchStick"),
                    TouchStickUiBundle {
                        stick: TouchStick {
                            id: Stick::Left,
                            stick_type: TouchStickType::Fixed,
                            ..default()
                        },
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(150.),
                            position_type: PositionType::Absolute,
                            left: Val::Px(35.),
                            bottom: Val::Percent(15.),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TouchStickUiKnob,
                        ImageBundle {
                            image: asset_server.load("knob.png").into(),
                            style: Style {
                                // REQUIRED style attributes
                                position_type: PositionType::Absolute, // knob must be positioned absolutely
                                margin: UiRect::all(Val::Auto), // margin must be set too Val::Auto or the knob wont be centered correctly
                                width: Val::Px(75.),
                                height: Val::Px(75.),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                    parent.spawn((
                        TouchStickUiOutline,
                        ImageBundle {
                            image: asset_server.load("outline.png").into(),
                            style: Style {
                                // REQUIRED style attributes
                                position_type: PositionType::Absolute,
                                margin: UiRect::all(Val::Auto),
                                width: Val::Px(150.),
                                height: Val::Px(150.),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    Name::new("RightTouchStick"),
                    TouchStickUiBundle {
                        stick: TouchStick {
                            id: Stick::Right,
                            stick_type: TouchStickType::Fixed,
                            ..default()
                        },
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(150.),
                            position_type: PositionType::Absolute,
                            right: Val::Px(35.),
                            bottom: Val::Percent(15.),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TouchStickUiKnob,
                        ImageBundle {
                            image: asset_server.load("knob.png").into(),
                            style: Style {
                                // REQUIRED style attributes
                                position_type: PositionType::Absolute, // knob must be positioned absolutely
                                margin: UiRect::all(Val::Auto), // margin must be set too Val::Auto or the knob wont be centered correctly
                                width: Val::Px(75.),
                                height: Val::Px(75.),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                    parent.spawn((
                        TouchStickUiOutline,
                        ImageBundle {
                            image: asset_server.load("outline.png").into(),
                            style: Style {
                                // REQUIRED style attributes
                                position_type: PositionType::Absolute,
                                margin: UiRect::all(Val::Auto),
                                width: Val::Px(150.),
                                height: Val::Px(150.),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn move_player(
    sticks: Query<&TouchStick<Stick>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, player) = players.single_mut();

    for stick in &sticks {
        let dt = time.delta_seconds();

        match stick.id {
            Stick::Left => {
                let delta = stick.value * player.max_speed * dt;
                player_transform.translation.x += delta.x;
                player_transform.translation.y += delta.y;
            }
            Stick::Right => {
                if stick.value != Vec2::ZERO {
                    let dir = Vec2::angle_between(Vec2::X, stick.value.normalize());
                    player_transform.rotation = Quat::from_rotation_z(dir);
                }
            }
        }
    }
}
