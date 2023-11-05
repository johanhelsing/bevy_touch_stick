use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::*;

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
            // WorldInspectorPlugin::new(),
            TouchStickPlugin::<Stick>::default(),
        ))
        .add_systems(Startup, create_scene)
        .add_systems(Update, move_player)
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
    commands.spawn(TouchStickBundle {
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
    });

    // Spawn Virtual Joystick on Right
    commands.spawn(TouchStickBundle {
        stick: TouchStick::new(Stick::Right),
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
    // todo: don't use events
    mut stick_events: EventReader<TouchStickEvent<Stick>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, player) = players.single_mut();

    for event in stick_events.read() {
        let Vec2 { x, y } = event.value();

        let dt = time.delta_seconds();

        // todo: maybe it's more interesting to set player direction per stick instead?
        match event.id() {
            Stick::Left => {
                player_transform.translation.x += x * player.max_speed * dt;
            }
            Stick::Right => {
                player_transform.translation.y += y * player.max_speed * dt;
            }
        }
    }
}
