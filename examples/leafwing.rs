use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::prelude::*;
use leafwing_input_manager::prelude::*;

/// Marker type for our touch stick
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
struct LookStick;

/// Marker type for touch stick
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
struct MoveStick;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
    Look,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            // add an inspector for easily changing settings at runtime
            WorldInspectorPlugin::default(),
            // add plugin for each dual axis you want
            TouchStickPlugin::<LookStick>::default(),
            TouchStickPlugin::<MoveStick>::default(),
            // we could even do a third that controls where the player shoots.
            // say we where in a 2d tank game with
            // camera/player movement on seperate sticks already
            // TouchStickPlugin::<ShootStick>::default()

            // add leafwing plugin
            InputManagerPlugin::<Action>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, move_camera))
        .run();
}

#[derive(Component)]
struct Player {
    max_speed: f32,
}

fn setup(mut commands: Commands) {
    // camera too see player / object / joystick
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 5.0),
        ..default()
    });

    // only here soo the camera movement system makes sense
    // just a static object
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::PINK,
            custom_size: Some(Vec2::splat(50.)),
            ..default()
        },
        transform: Transform::from_xyz(20.0, 60.0, 0.0),
        ..default()
    },));

    commands.spawn((
        Player { max_speed: 50. },
        InputManagerBundle::<Action> {
            // Stores "which actions are currently activated"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(DualAxis::right_stick(), Action::Look)
                .build(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
            ..default()
        },
    ));

    // spawn a move stick
    commands.spawn((
        BackgroundColor(Color::BLUE),
        // map this stick as a left gamepad stick (through bevy_input)
        // leafwing will register this as a normal gamepad
        TouchStickGamepadMapping::LEFT_STICK,
        TouchStickUiBundle {
            stick: TouchStick {
                id: MoveStick,
                stick_type: TouchStickType::Fixed,
                ..default()
            },
            // configure the interactable area through bevy_ui
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(150.),
                position_type: PositionType::Absolute,
                left: Val::Percent(15.),
                bottom: Val::Percent(5.),
                ..default()
            },
            ..default()
        },
    ));

    // spawn a look stick
    commands.spawn((
        BackgroundColor(Color::BLUE),
        // map this stick as a right gamepad stick (through bevy_input)
        // leafwing will register this as a normal gamepad
        TouchStickGamepadMapping::RIGHT_STICK,
        TouchStickUiBundle {
            stick: TouchStick {
                id: LookStick,
                stick_type: TouchStickType::Floating,
                ..default()
            },
            // configure the interactable area through bevy_ui
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(150.),
                position_type: PositionType::Absolute,
                right: Val::Percent(15.),
                bottom: Val::Percent(5.),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_player(
    mut players: Query<(&mut Transform, &ActionState<Action>, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, action_state, player) = players.single_mut();

    if action_state.pressed(Action::Move) {
        let axis_value = action_state.clamped_axis_pair(Action::Move).unwrap().xy();

        info!("moving: {axis_value}");

        let move_delta = axis_value * player.max_speed * time.delta_seconds();
        player_transform.translation += move_delta.extend(0.);
    }
}

fn move_camera(
    player: Query<&ActionState<Action>, With<Player>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    let input = player.single();
    let mut camera_transform = camera.single_mut();

    if input.pressed(Action::Look) {
        let axis_value = input
            .clamped_axis_pair(Action::Look)
            .unwrap_or_default()
            .xy();
        camera_transform.translation += axis_value.extend(0.0)
    }
}
