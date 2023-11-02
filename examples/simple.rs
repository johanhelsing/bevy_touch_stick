use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::{
    prelude::*, TintColor, TouchStickEvent, TouchStickEventType, TouchStickInteractionArea,
};

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
        // required marker component
        TouchStickInteractionArea,
        TouchStickBundle::new(TouchStickNode::<MyStick> {
            border_image: asset_server.load("outline.png"),
            knob_image: asset_server.load("knob.png"),
            knob_size: Vec2::splat(80.),
            dead_zone: 0.,
            behavior: TouchStickType::Floating,
            ..default()
        })
        .set_color(TintColor(Color::WHITE.with_a(0.2)))
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
            TouchStickEventType::Up => TintColor(Color::WHITE.with_a(0.2)),
        };

        for (mut color, node) in &mut sticks {
            if node.id == event.id() {
                *color = tint_color;
            }
        }
    }
}

fn move_player(
    // todo: this should use a resource/component instead of events
    mut stick_events: EventReader<TouchStickEvent<MyStick>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, player) = players.single_mut();

    for event in stick_events.iter() {
        let move_delta = event.value() * player.max_speed * time.delta_seconds();
        player_transform.translation += move_delta.extend(0.);
    }
}
