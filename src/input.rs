use std::hash::Hash;

use bevy::{
    input::{mouse::MouseButtonInput, touch::TouchPhase, ButtonState},
    prelude::*,
    reflect::TypePath,
    window::PrimaryWindow,
};

use crate::{
    joystick::TouchStickKnob, TouchStickEvent, TouchStickEventType, TouchStickNode, TouchStickType,
};

#[derive(Event)]
pub(crate) enum DragEvent {
    StartDrag { id: u64, pos: Vec2 },
    Dragging { id: u64, pos: Vec2 },
    EndDrag { id: u64 },
}

fn is_some_and<T>(opt: Option<T>, cb: impl FnOnce(T) -> bool) -> bool {
    if let Some(v) = opt {
        return cb(v);
    }
    false
}

pub(crate) fn update_input<
    S: Hash + Sync + Send + Clone + Default + Reflect + TypePath + FromReflect + 'static,
>(
    mut input_events: EventReader<DragEvent>,
    mut stick_events: EventWriter<TouchStickEvent<S>>,
    mut sticks: Query<(&TouchStickNode<S>, &mut TouchStickKnob)>,
) {
    let input_events = input_events.iter().collect::<Vec<&DragEvent>>();

    for (node, mut knob) in sticks.iter_mut() {
        for event in &input_events {
            match event {
                DragEvent::StartDrag { id, pos } => {
                    if knob.interactable_zone.contains(*pos) && knob.id_drag.is_none()
                        || is_some_and(knob.id_drag, |i| i != *id)
                            && knob.interactable_zone.contains(*pos)
                    {
                        knob.id_drag = Some(*id);
                        knob.start_pos = *pos;
                        knob.current_pos = *pos;
                        knob.value = Vec2::ZERO;
                        stick_events.send(TouchStickEvent {
                            id: node.id.clone(),
                            event: TouchStickEventType::Press,
                            value: Vec2::ZERO,
                        });
                    }
                }
                DragEvent::Dragging { id, pos } => {
                    if !is_some_and(knob.id_drag, |i| i == *id) {
                        continue;
                    }
                    knob.current_pos = *pos;
                    let half = knob.interactable_zone.half_size();
                    if node.behavior == TouchStickType::Dynamic {
                        knob.base_pos = *pos;
                        let to_knob = knob.current_pos - knob.start_pos;
                        let distance_to_knob = to_knob.length();
                        if distance_to_knob > half.x {
                            let excess_distance = distance_to_knob - half.x;
                            knob.start_pos += to_knob.normalize() * excess_distance;
                        }
                    }
                    let d = (knob.current_pos - knob.start_pos) / half;
                    let length = d.length();
                    // input events are y positive down, so we flip it
                    knob.value = Vec2::new(d.x, -d.y) / length.max(1.);
                }
                DragEvent::EndDrag { id } => {
                    if !is_some_and(knob.id_drag, |i| i == *id) {
                        continue;
                    }
                    knob.id_drag = None;
                    knob.base_pos = Vec2::ZERO;
                    knob.start_pos = Vec2::ZERO;
                    knob.current_pos = Vec2::ZERO;
                    knob.value = Vec2::ZERO;
                    stick_events.send(TouchStickEvent {
                        id: node.id.clone(),
                        event: TouchStickEventType::Up,
                        value: Vec2::ZERO,
                    });
                }
            }
        }

        // Send event
        if (knob.value.x.abs() >= knob.dead_zone || knob.value.y.abs() >= knob.dead_zone)
            && knob.id_drag.is_some()
        {
            stick_events.send(TouchStickEvent {
                id: node.id.clone(),
                event: TouchStickEventType::Drag,
                value: knob.value,
            });
        }
    }
}

pub(crate) fn update_joystick(
    mut touch_events: EventReader<TouchInput>,
    mut send_values: EventWriter<DragEvent>,
) {
    let touches = touch_events
        .iter()
        .map(|e| (e.id, e.phase, e.position))
        .collect::<Vec<(u64, TouchPhase, Vec2)>>();

    for (id, phase, pos) in &touches {
        match phase {
            TouchPhase::Started => {
                send_values.send(DragEvent::StartDrag { id: *id, pos: *pos });
            }
            TouchPhase::Moved => {
                send_values.send(DragEvent::Dragging { id: *id, pos: *pos });
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                send_values.send(DragEvent::EndDrag { id: *id });
            }
        }
    }
}

pub(crate) fn update_joystick_by_mouse(
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_events: EventReader<MouseButtonInput>,
    mut send_values: EventWriter<DragEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let pos = window.cursor_position().unwrap_or(Vec2::ZERO);

    for mouse_event in mouse_events.iter() {
        if mouse_event.button == MouseButton::Left && mouse_event.state == ButtonState::Released {
            send_values.send(DragEvent::EndDrag { id: 0 });
        }

        if mouse_event.button == MouseButton::Left && mouse_event.state == ButtonState::Pressed {
            send_values.send(DragEvent::StartDrag { id: 0, pos });
        }
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        send_values.send(DragEvent::Dragging { id: 0, pos });
    }
}
