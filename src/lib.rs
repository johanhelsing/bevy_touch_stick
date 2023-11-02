use bevy::{
    prelude::*,
    reflect::TypePath,
    render::RenderApp,
    ui::{RenderUiSystem, UiSystem},
};
use std::{hash::Hash, marker::PhantomData};

mod behaviour;
mod input;
mod joystick;

pub mod prelude {
    pub use crate::{TouchStickBundle, TouchStickNode, TouchStickPlugin, TouchStickType};
}

pub use crate::{
    behaviour::TouchStickType,
    joystick::{TintColor, TouchStickBundle, TouchStickInteractionArea, TouchStickNode},
};
use crate::{
    input::{update_input, update_joystick, update_joystick_by_mouse, DragEvent},
    joystick::{extract_joystick_node, VirtualJoystickKnob},
};

#[derive(Default)]
pub struct TouchStickPlugin<S> {
    _marker: PhantomData<S>,
}

impl<S: Hash + Sync + Send + Clone + Default + Reflect + TypePath + FromReflect + 'static> Plugin
    for TouchStickPlugin<S>
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<TintColor>()
            .register_type::<TouchStickInteractionArea>()
            .register_type::<TouchStickNode<S>>()
            .register_type::<VirtualJoystickKnob>()
            .register_type::<TouchStickType>()
            .register_type::<VirtualJoystickEventType>()
            .add_event::<TouchStickEvent<S>>()
            .add_event::<DragEvent>()
            .add_systems(PreUpdate, update_joystick.before(update_input::<S>))
            .add_systems(
                PreUpdate,
                update_joystick_by_mouse.before(update_input::<S>),
            )
            .add_systems(PreUpdate, update_input::<S>)
            .add_systems(
                PostUpdate,
                joystick_image_node_system::<S>.before(UiSystem::Layout),
            );

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

fn joystick_image_node_system<
    S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + 'static,
>(
    interaction_area: Query<(&Node, With<TouchStickInteractionArea>)>,
    mut joystick: Query<(&Transform, &TouchStickNode<S>, &mut VirtualJoystickKnob)>,
) {
    let interaction_area = interaction_area
        .iter()
        .map(|(node, _)| node.size())
        .collect::<Vec<Vec2>>();

    for (i, (j_pos, data, mut knob)) in joystick.iter_mut().enumerate() {
        let j_pos = j_pos.translation.truncate();
        let Some(size) = interaction_area.get(i) else {
            return;
        };
        let interaction_area = Rect::from_center_size(j_pos, *size);
        knob.dead_zone = data.dead_zone;
        knob.interactable_zone_rect = interaction_area;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect]
pub enum VirtualJoystickEventType {
    Press,
    Drag,
    Up,
}

#[derive(Event)]
pub struct TouchStickEvent<S: Hash + Sync + Send + Clone + Default + Reflect + 'static + TypePath> {
    id: S,
    event: VirtualJoystickEventType,
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
    pub fn get_type(&self) -> VirtualJoystickEventType {
        self.event
    }
}
