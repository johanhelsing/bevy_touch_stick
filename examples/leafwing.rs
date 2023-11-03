use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::{
    prelude::*, TintColor, TouchStickEvent, TouchStickEventType, TouchStickGamepadMapping,
};
use leafwing_input_manager::prelude::*;

/// Marker type for our touch stick
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
struct MyStick;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            // add an inspector for easily changing settings at runtime
            WorldInspectorPlugin::default(),
            // add the plugin
            TouchStickPlugin::<MyStick>::default(),
            // add leafwing plugin
            InputManagerPlugin::<Action>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_stick_color, move_player))
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
        InputManagerBundle::<Action> {
            // Stores "which actions are currently activated"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
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

    // spawn a touch stick
    commands.spawn((
        // map this stick as a left gamepad stick (through bevy_input)
        // leafwing will register this as a normal gamepad
        TouchStickGamepadMapping::LEFT_STICK,
        TouchStickBundle::new(TouchStickNode::<MyStick> {
            border_image: asset_server.load("outline.png"),
            knob_image: asset_server.load("knob.png"),
            knob_size: Vec2::splat(80.),
            dead_zone: 0.,
            ..default()
        })
        // set the tint color for the knob texture
        .set_color(TintColor(Color::WHITE.with_a(0.2)))
        // configure the interactable area through bevy_ui
        .set_style(Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.),
            bottom: Val::Percent(15.),
            ..default()
        }),
        // make it easy to see the area in which the stick can be interacted with
        BackgroundColor(Color::WHITE.with_a(0.05)),
    ));
}

fn update_stick_color(
    mut stick_events: EventReader<TouchStickEvent<MyStick>>,
    mut sticks: Query<(&mut TintColor, &TouchStickNode<MyStick>)>,
) {
    for event in stick_events.iter() {
        let tint_color = match event.get_type() {
            TouchStickEventType::Press | TouchStickEventType::Drag => TintColor(Color::WHITE),
            TouchStickEventType::Release => TintColor(Color::WHITE.with_a(0.2)),
        };

        for (mut color, node) in &mut sticks {
            if node.id == event.id() {
                *color = tint_color;
            }
        }
    }
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