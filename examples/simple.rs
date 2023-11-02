use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::{
    prelude::*, TintColor, VirtualJoystickEvent, VirtualJoystickEventType,
    VirtualJoystickInteractionArea,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(VirtualJoystickPlugin::<String>::default())
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
        Player { max_speed: 50. },
    ));

    // Spawn a stick at horizontal center
    commands.spawn((
        VirtualJoystickInteractionArea,
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("outline.png"),
            knob_image: asset_server.load("knob.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
            id: "UniqueJoystick".to_string(),
            behaviour: VirtualJoystickType::Floating,
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
        // Make it easy to see the area in which the stick can be interacted with
        BackgroundColor(Color::ORANGE_RED.with_a(0.3)),
    ));
}

fn update_stick_color(
    mut stick_events: EventReader<VirtualJoystickEvent<String>>,
    mut sticks: Query<(&mut TintColor, &VirtualJoystickNode<String>)>,
) {
    for event in stick_events.iter() {
        let tint_color = match event.get_type() {
            VirtualJoystickEventType::Press | VirtualJoystickEventType::Drag => {
                TintColor(Color::WHITE)
            }
            VirtualJoystickEventType::Up => TintColor(Color::WHITE.with_a(0.2)),
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
    mut stick_events: EventReader<VirtualJoystickEvent<String>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, player) = players.single_mut();

    for event in stick_events.iter() {
        let move_delta = event.value() * player.max_speed * time.delta_seconds();
        player_transform.translation += move_delta.extend(0.);
    }
}
