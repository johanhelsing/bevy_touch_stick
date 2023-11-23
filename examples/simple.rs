use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::{prelude::*, TouchStickUiKnob, TouchStickUiOutline};

/// Marker type for our touch stick
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
struct MyStick;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            // add an inspector for easily changing settings at runtime
            WorldInspectorPlugin::default(),
            // add the plugin
            TouchStickPlugin::<MyStick>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player {
    max_speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 5.0),
        ..default()
    });

    commands.spawn((
        Player { max_speed: 50. },
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
            ..default()
        },
    ));

    // spawn a touch stick
    commands
        .spawn(TouchStickUiBundle::<MyStick> {
            stick_node: TouchStickUi { id: MyStick },
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(150.),
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                bottom: Val::Vw(15.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TouchStickUiKnob,
                ImageBundle {
                    image: asset_server.load("knob.png").into(),
                    style: Style {
                        width: Val::Px(75.),
                        height: Val::Px(75.),
                        ..default()
                    },
                    background_color: Color::ORANGE.with_a(0.3).into(),
                    ..default()
                },
            ));
            parent.spawn((
                TouchStickUiOutline,
                ImageBundle {
                    image: asset_server.load("outline.png").into(),
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(150.),
                        ..default()
                    },
                    background_color: Color::ORANGE.with_a(0.3).into(),
                    ..default()
                },
            ));
        });
}

fn move_player(
    sticks: Query<&TouchStick<MyStick>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, player) = players.single_mut();
    let stick = sticks.single();
    let move_delta = stick.value * player.max_speed * time.delta_seconds();
    player_transform.translation += move_delta.extend(0.);
}
