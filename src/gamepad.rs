use std::marker::PhantomData;

use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadConnection, GamepadConnectionEvent, GamepadEvent,
        GamepadInfo,
    },
    prelude::*,
};

use crate::{StickIdType, TouchStick};

/// Plugin that makes [`TouchStick`]s pretend to be regular bevy gamepads
///
/// Add [`GamepadAxisMapping`] to a [`TouchStick`] to make it show up as a bevy gamepad.
pub(crate) struct GamepadMappingPlugin<S: StickIdType> {
    _marker: PhantomData<S>,
}

impl<S: StickIdType> Default for GamepadMappingPlugin<S> {
    fn default() -> Self {
        Self { _marker: default() }
    }
}

impl<S: StickIdType> Plugin for GamepadMappingPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (connect_gamepad::<S>, send_axis_events::<S>).chain(),
        );
    }
}

/// HACK: chosen at random, we're betting on no collisions with gilrs gamepads
/// needs to be below `u32::MAX` to work on 32bit platforms.
const TOUCH_GAMEPAD_ID: usize = 3407632091;

const TOUCH_GAMEPAD: Gamepad = Gamepad {
    id: TOUCH_GAMEPAD_ID,
};

/// Mapping of a [`TouchStick`] to bevy gamepad axes.
///
/// Adding this component to a [`TouchStick`] will create an emulated gamepad through `bevy_input`.
#[derive(Component, Reflect, Clone, Copy, Debug, Eq, PartialEq)]
pub struct TouchStickGamepadMapping(pub GamepadAxisType, pub GamepadAxisType);

impl TouchStickGamepadMapping {
    /// Defines default left stick mapping
    pub const LEFT_STICK: Self =
        TouchStickGamepadMapping(GamepadAxisType::LeftStickX, GamepadAxisType::LeftStickY);
    /// Defines default right stick mapping
    pub const RIGHT_STICK: Self =
        TouchStickGamepadMapping(GamepadAxisType::RightStickX, GamepadAxisType::RightStickY);
}

/// The gamepad is connected when the first [`TouchStick`] is added.
fn connect_gamepad<S: StickIdType>(
    mut gamepad_events: EventWriter<GamepadEvent>,
    sticks: Query<(), (With<TouchStick<S>>, With<TouchStickGamepadMapping>)>,
    mut was_connected: Local<bool>,
) {
    let connected = !sticks.is_empty();

    if *was_connected != connected {
        *was_connected = connected;

        let connection = if connected {
            GamepadConnection::Connected(GamepadInfo {
                name: "bevy_touch_stick".into(),
            })
        } else {
            GamepadConnection::Disconnected
        };

        gamepad_events.send(GamepadEvent::Connection(GamepadConnectionEvent {
            gamepad: TOUCH_GAMEPAD,
            connection,
        }));
    }
}

/// Reads values from touch sticks and sends as bevy input events
fn send_axis_events<S: StickIdType>(
    mut events: EventWriter<GamepadEvent>,
    sticks: Query<(&TouchStick<S>, &TouchStickGamepadMapping)>,
) {
    for (stick, axis_mapping) in &sticks {
        let gamepad = TOUCH_GAMEPAD;
        let TouchStickGamepadMapping(x_type, y_type) = axis_mapping;
        let Vec2 { x, y } = stick.value;
        trace!("sending axis event {x}, {y}");
        // TODO: bevy does this, maybe we should as well?
        // let axis = GamepadAxis::new(gamepad, axis_type);
        // let old_value = stick.value;
        // let axis_settings = gamepad_settings.get_axis_settings(axis);
        // // Only send events that pass the user-defined change threshold
        // if let Some(filtered_value) = axis_settings.filter(raw_value, old_value) {
        // events.send(GamepadAxisChangedEvent::new(gamepad, axis_type, filtered_value).into());
        // }

        events.send(GamepadAxisChangedEvent::new(gamepad, *x_type, x).into());
        events.send(GamepadAxisChangedEvent::new(gamepad, *y_type, y).into());
    }
}
