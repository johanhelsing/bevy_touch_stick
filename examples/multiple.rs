use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::prelude::*;

// ID for left joystick
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
struct LeftStick;

// ID for right joystick
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
struct RightStick;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // WorldInspectorPlugin::new(),
            TouchStickPlugin::<LeftStick>::default(),
            TouchStickPlugin::<RightStick>::default(),
        ))
        .add_systems(Startup, create_scene)
        .add_systems(Update, (move_player, move_camera))
        .run();
}

#[derive(Component)]
// Player with velocity
struct Player {
    pub max_speed: f32,
}

fn create_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 5.0),
        ..default()
    });

    commands.spawn((
        Player { max_speed: 50. },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            texture: asset_server.load("knob.png"),
            sprite: Sprite {
                color: Color::PURPLE,
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            ..default()
        },
    ));

    // Spawn Virtual Joystick on left
    commands.spawn(TouchStickUiBundle {
        stick: LeftStick.into(),
        style: Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Px(35.),
            bottom: Val::Percent(15.),
            ..default()
        },
        ..default()
    });

    // Spawn Virtual Joystick on Right
    commands.spawn(TouchStickUiBundle {
        stick: RightStick.into(),
        style: Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            right: Val::Px(35.),
            bottom: Val::Percent(15.),
            ..default()
        },
        ..default()
    });
}

fn move_player(
    left_stick: Query<&TouchStick<LeftStick>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, player) = players.single_mut();
    let stick = left_stick.single();

    let Vec2 { x, y } = stick.value;

    let dt = time.delta_seconds();

    // todo: maybe it's more interesting to set player direction per stick instead?
    player_transform.translation.x += x * player.max_speed * dt;
    player_transform.translation.y += y * player.max_speed * dt;
}

fn move_camera(
    right_stick: Query<&TouchStick<RightStick>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera.single_mut();
    let axis_value = right_stick.single().value;

    camera_transform.translation += axis_value.extend(0.0) * time.delta_seconds()
}
