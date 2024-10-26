use bevy::{color::palettes::css as colors, prelude::*};
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
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 5.0),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Player"),
        Player { max_speed: 50. },
        SpriteBundle {
            sprite: Sprite {
                color: colors::ORANGE.into(),
                custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
            ..default()
        },
    ));

    // size of outline for touchstick
    let radius = 250.0;
    // knob size for touchstick
    let knob_size = 75.0;
    let knob_handle: Handle<Image> = asset_server.load("knob.png");
    let outline_handle: Handle<Image> = asset_server.load("outline.png");
    // define 2 position elements and then leave the other 2 at Val::Auto,
    let position = UiRect { left: Val::Px(0.0), right: Val::Auto, top: Val::Auto, bottom: Val::Px(0.0) };

    // TODO: extract below to reuseable function?
    // spawn a touch stick
    commands
        .spawn((
            Name::new("TouchStick"),
            TouchStickUiBundle::<MyStick> {
                stick: TouchStick::<MyStick> {
                    stick_type: TouchStickType::Fixed,
                    radius,
                    ..default()
                },
                style: Style {
                    width: Val::Px(radius),
                    height: Val::Px(radius),
                    top: position.top,
                    bottom: position.bottom,
                    left: position.left,
                    right: position.right,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Knob"),
                TouchStickUiKnob,
                ImageBundle {
                    // background_color: BackgroundColor(colors::ORANGE.into()),
                    image: knob_handle.into(),
                    style: Style {
                        // REQUIRED style attributes
                        position_type: PositionType::Absolute, // knob must be positioned absolutely
                        margin: UiRect::all(Val::Auto), // margin must be set too Val::Auto or the knob wont be centered correctly
                        width: Val::Px(knob_size),
                        height: Val::Px(knob_size),
                        ..default()
                    },
                    ..default()
                },
            ));
            parent.spawn((
                Name::new("Outline"),
                TouchStickUiOutline,
                ImageBundle {
                    // background_color: BackgroundColor(colors::PURPLE.into()),
                    image: outline_handle.into(),
                    style: Style {
                        // REQUIRED style attributes
                        margin: UiRect::all(Val::Auto),
                        position_type: PositionType::Absolute,
                        width: Val::Px(radius),
                        height: Val::Px(radius),
                        ..default()
                    },
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
