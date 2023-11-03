use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
            WorldInspectorPlugin::new(),
            TouchStickPlugin::<Stick>::default(),
        ))
        .add_systems(Startup, create_scene)
        .add_systems(Update, (move_player, update_stick_colors))
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
    commands.spawn((
        TouchStickBundle::new(TouchStickNode {
            border_image: asset_server.load("outline.png"),
            knob_image: asset_server.load("knob.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
            id: Stick::Left,
            behavior: TouchStickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE.with_a(0.2)))
        .set_style(Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Px(35.),
            bottom: Val::Percent(15.),
            ..default()
        }),
        BackgroundColor(Color::ORANGE_RED.with_a(0.2)),
    ));

    // Spawn Virtual Joystick on Right
    commands.spawn((
        TouchStickBundle::new(TouchStickNode {
            border_image: asset_server.load("outline.png"),
            knob_image: asset_server.load("knob.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
            id: Stick::Right,
            behavior: TouchStickType::Fixed,
            ..default()
        })
        .set_color(TintColor(Color::WHITE.with_a(0.2)))
        .set_style(Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            right: Val::Px(35.),
            bottom: Val::Percent(15.),
            ..default()
        }),
        BackgroundColor(Color::ORANGE_RED.with_a(0.2)),
    ));
}

fn update_stick_colors(
    mut stick_events: EventReader<TouchStickEvent<Stick>>,
    mut sticks: Query<(&mut TintColor, &TouchStickNode<Stick>)>,
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
    // todo: don't use events
    mut stick_events: EventReader<TouchStickEvent<Stick>>,
    mut players: Query<(&mut Transform, &Player)>,
    time_step: Res<FixedTime>,
) {
    let (mut player_transform, player) = players.single_mut();

    for event in stick_events.iter() {
        let Vec2 { x, y } = event.value();

        // todo: maybe it's more interesting to set player direction per stick instead?
        match event.id() {
            Stick::Left => {
                player_transform.translation.x +=
                    x * player.max_speed * time_step.period.as_secs_f32();
            }
            Stick::Right => {
                player_transform.translation.y +=
                    y * player.max_speed * time_step.period.as_secs_f32();
            }
        }
    }
}
