use bevy::{prelude::*, reflect::TypePath, ui::UiSystem};
use std::{hash::Hash, marker::PhantomData};
use ui::TouchStickUiPlugin;

mod behavior;
#[cfg(feature = "gamepad_mapping")]
mod gamepad;
mod input;
#[cfg(feature = "material2d")]
mod renderer;
mod ui;

pub mod prelude {
    #[cfg(feature = "gamepad_mapping")]
    pub use crate::TouchStickGamepadMapping;
    pub use crate::{
        TouchStick, TouchStickPlugin, TouchStickType, TouchStickUi, TouchStickUiBundle,
    };
    // todo: remove this?
    pub use crate::ui::CircleMaterial;
}

#[cfg(feature = "gamepad_mapping")]
use crate::gamepad::GamepadMappingPlugin;
#[cfg(feature = "gamepad_mapping")]
pub use crate::gamepad::TouchStickGamepadMapping;
use crate::input::{
    send_drag_events_from_mouse, send_drag_events_from_touch, update_sticks_from_drag_events,
    DragEvent,
};
pub use crate::{
    behavior::TouchStickType,
    ui::{
        CircleMaterial, TouchStickInteractionArea, TouchStickUi, TouchStickUiBundle,
        TouchStickUiKnob, TouchStickUiOutline,
    },
};

/// pure data, independent of bevy_ui
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct TouchStick<S: StickIdType> {
    pub id: S,
    pub drag_id: Option<u64>,
    pub dead_zone: f32,
    /// todo: only used for dynamic mode
    pub base_position: Vec2,
    /// The screen position where the drag was started
    pub drag_start: Vec2,
    /// The screen position where the drag is currently at
    pub drag_position: Vec2,
    /// Value with maximum magnitude 1
    pub value: Vec2,
    /// In input space (y-down)
    pub interactable_zone: Rect,
    /// Define the behavior of joystick
    pub stick_type: TouchStickType,
}

impl<S: StickIdType> From<S> for TouchStick<S> {
    fn from(id: S) -> Self {
        Self::new(id)
    }
}

impl<S: StickIdType> TouchStick<S> {
    pub fn new(id: S) -> Self {
        Self {
            id: id.into(),
            // drag_id: None,
            // dead_zone: 0.,
            // base_position: default(),
            // start_position: default(),
            // current_position: default(),
            // value: default(),
            // interactable_zone: Rect {
            //     min: Vec2::MIN,
            //     max: Vec2::MAX,
            // },
            // stick_type: default(),
            ..default()
        }
    }
}

pub struct TouchStickPlugin<S> {
    _marker: PhantomData<S>,
}

impl<S> Default for TouchStickPlugin<S> {
    fn default() -> Self {
        Self { _marker: default() }
    }
}

impl<S: StickIdType> Plugin for TouchStickPlugin<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<TouchStickInteractionArea>()
            .register_type::<TouchStickUi<S>>()
            .register_type::<TouchStick<S>>()
            .register_type::<TouchStickType>()
            .register_type::<TouchStickEventType>()
            .add_event::<TouchStickEvent<S>>()
            .add_event::<DragEvent>()
            .add_plugins(TouchStickUiPlugin::<S>::default())
            .add_systems(
                PreUpdate,
                (
                    // todo: resolve ambiguity
                    send_drag_events_from_touch.before(update_sticks_from_drag_events::<S>),
                    send_drag_events_from_mouse.before(update_sticks_from_drag_events::<S>),
                ),
            )
            .add_systems(PreUpdate, update_sticks_from_drag_events::<S>)
            .add_systems(
                PostUpdate,
                map_input_zones_from_ui_nodes::<S>.before(UiSystem::Layout),
            );

        #[cfg(feature = "gamepad_mapping")]
        app.add_plugins(GamepadMappingPlugin::<S>::default());
    }
}

pub trait StickIdType:
    Hash + Sync + Send + Clone + Default + Reflect + FromReflect + TypePath + 'static
{
}

impl<S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + TypePath + 'static>
    StickIdType for S
{
}

fn map_input_zones_from_ui_nodes<S: StickIdType>(
    interaction_areas: Query<(&Transform, &Node), With<TouchStickInteractionArea>>,
    mut sticks: Query<&mut TouchStick<S>>,
) {
    for (transform, node) in &interaction_areas {
        // todo: match stick ids!

        let pos = transform.translation.truncate();
        let size = node.size();
        let interaction_area = Rect::from_center_size(pos, size);

        for mut stick in sticks.iter_mut() {
            stick.interactable_zone = interaction_area;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect]
pub enum TouchStickEventType {
    Press,
    Drag,
    Release,
}

#[derive(Event)]
pub struct TouchStickEvent<S: StickIdType> {
    id: S,
    event: TouchStickEventType,
    value: Vec2,
}

impl<S: StickIdType> TouchStickEvent<S> {
    /// Get Id of joystick throw event
    pub fn id(&self) -> S {
        self.id.clone()
    }

    /// Value of the joystick, maximum length 1
    pub fn value(&self) -> Vec2 {
        self.value
    }

    /// Return the Type of Joystick Event
    pub fn get_type(&self) -> TouchStickEventType {
        self.event
    }
}
