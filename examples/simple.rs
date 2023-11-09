use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_touch_stick::prelude::*;

/// Marker type for our touch stick
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
struct MyStick;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            // add an inspector for easily changing settings at runtime
            // WorldInspectorPlugin::default(),
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

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<CircleMaterial>>,
    asset_server: Res<AssetServer>,
) {
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

    // sticks are entities for now (could perhaps be resources)
    // commands.spawn(TouchStick::<MyStick>::default());

    // spawn a touch stick
    commands.spawn(TouchStickUiBundle::<MyStick> {
        // todo: provide default material
        stick_node: TouchStickUi {
            knob_image: asset_server.load("knob.png"),
            border_image: asset_server.load("outline.png"),
            knob_radius: 40.,
            outline_radius: 80.,
            ..default()
        },
        // material: materials.add(CircleMaterial {
        //     color: Vec4::new(1., 0., 0., 1.),
        // }),
        style: Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.),
            bottom: Val::Percent(15.),
            ..default()
        },
        ..default()
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
