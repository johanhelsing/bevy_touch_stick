use bevy::{
    prelude::*,
    reflect::TypePath,
    render::RenderApp,
    ui::{RenderUiSystem, UiSystem},
};
use std::{hash::Hash, marker::PhantomData};

mod behavior;
#[cfg(feature = "gamepad_mapping")]
mod gamepad;
mod input;
mod joystick;

pub mod prelude {
    pub use crate::{
        TouchStick, TouchStickBundle, TouchStickNode, TouchStickPlugin, TouchStickType,
    };
}

#[cfg(feature = "gamepad_mapping")]
use crate::gamepad::GamepadMappingPlugin;
#[cfg(feature = "gamepad_mapping")]
pub use crate::gamepad::TouchStickGamepadMapping;
pub use crate::{
    behavior::TouchStickType,
    joystick::{
        TintColor, TouchStick, TouchStickBundle, TouchStickInteractionArea, TouchStickNode,
    },
};
use crate::{
    input::{update_input, update_sticks, update_sticks_from_mouse, DragEvent},
    joystick::extract_joystick_node,
};

pub struct TouchStickPlugin<S> {
    _marker: PhantomData<S>,
}

impl<S> Default for TouchStickPlugin<S> {
    fn default() -> Self {
        Self { _marker: default() }
    }
}

impl<S: Hash + Sync + Send + Clone + Default + Reflect + TypePath + FromReflect + 'static> Plugin
    for TouchStickPlugin<S>
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<TintColor>()
            .register_type::<TouchStickInteractionArea>()
            .register_type::<TouchStickNode<S>>()
            .register_type::<TouchStick>()
            .register_type::<TouchStickType>()
            .register_type::<TouchStickEventType>()
            .add_event::<TouchStickEvent<S>>()
            .add_event::<DragEvent>()
            .add_systems(PreUpdate, update_sticks.before(update_input::<S>))
            .add_systems(
                PreUpdate,
                update_sticks_from_mouse.before(update_input::<S>),
            )
            .add_systems(PreUpdate, update_input::<S>)
            .add_systems(
                PostUpdate,
                map_input_zones_from_ui_nodes::<S>.before(UiSystem::Layout),
            );

        #[cfg(feature = "gamepad_mapping")]
        app.add_plugins(GamepadMappingPlugin);

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };
        render_app.add_systems(
            ExtractSchedule,
            extract_joystick_node::<S>.after(RenderUiSystem::ExtractNode),
        );
    }
}

fn map_input_zones_from_ui_nodes<
    S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + 'static,
>(
    interaction_areas: Query<(&Node, With<TouchStickInteractionArea>)>,
    mut sticks: Query<(&Transform, &TouchStickNode<S>, &mut TouchStick)>,
) {
    // todo: this looks like a giant hack
    let interaction_areas = interaction_areas
        .iter()
        .map(|(node, _)| node.size())
        .collect::<Vec<Vec2>>();

    for (i, (stick_transform, stick_node, mut stick)) in sticks.iter_mut().enumerate() {
        let j_pos = stick_transform.translation.truncate();
        let Some(size) = interaction_areas.get(i) else {
            return;
        };
        let interaction_area = Rect::from_center_size(j_pos, *size);
        stick.dead_zone = stick_node.dead_zone;
        stick.interactable_zone = interaction_area;
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
pub struct TouchStickEvent<S: Hash + Sync + Send + Clone + Default + Reflect + 'static + TypePath> {
    id: S,
    event: TouchStickEventType,
    value: Vec2,
}

impl<S: Hash + Sync + Send + Clone + Default + Reflect + TypePath + 'static> TouchStickEvent<S> {
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
