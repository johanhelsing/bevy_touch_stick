use std::hash::Hash;

use bevy::{
    input::{mouse::MouseButtonInput, touch::TouchPhase, ButtonState},
    prelude::*,
    reflect::TypePath,
    window::PrimaryWindow,
};

use crate::{
    joystick::TouchStick, TouchStickEvent, TouchStickEventType, TouchStickNode, TouchStickType,
};

#[derive(Event)]
pub(crate) enum DragEvent {
    Start { id: u64, position: Vec2 },
    Drag { id: u64, position: Vec2 },
    End { id: u64 },
}

pub(crate) fn update_input<
    S: Hash + Sync + Send + Clone + Default + Reflect + TypePath + FromReflect + 'static,
>(
    mut drag_events: EventReader<DragEvent>,
    mut stick_events: EventWriter<TouchStickEvent<S>>,
    mut sticks: Query<(&TouchStickNode<S>, &mut TouchStick)>,
) {
    let input_events = drag_events.iter().collect::<Vec<&DragEvent>>();

    for (node, mut knob) in sticks.iter_mut() {
        for event in &input_events {
            match event {
                DragEvent::Start { id, position } => {
                    if knob.interactable_zone.contains(*position) && knob.drag_id != Some(*id) {
                        knob.drag_id = Some(*id);
                        knob.start_position = *position;
                        knob.current_position = *position;
                        knob.value = Vec2::ZERO;
                        stick_events.send(TouchStickEvent {
                            id: node.id.clone(),
                            event: TouchStickEventType::Press,
                            value: Vec2::ZERO,
                        });
                    }
                }
                DragEvent::Drag { id, position: pos } if Some(*id) == knob.drag_id => {
                    knob.current_position = *pos;
                    let half = knob.interactable_zone.half_size();
                    if node.behavior == TouchStickType::Dynamic {
                        knob.base_position = *pos;
                        let to_knob = knob.current_position - knob.start_position;
                        let distance_to_knob = to_knob.length();
                        if distance_to_knob > half.x {
                            let excess_distance = distance_to_knob - half.x;
                            knob.start_position += to_knob.normalize() * excess_distance;
                        }
                    }
                    let d = (knob.current_position - knob.start_position) / half;
                    let length = d.length();
                    // input events are y positive down, so we flip it
                    knob.value = Vec2::new(d.x, -d.y) / length.max(1.);
                }
                DragEvent::End { id } if Some(*id) == knob.drag_id => {
                    knob.drag_id = None;
                    knob.base_position = Vec2::ZERO;
                    knob.start_position = Vec2::ZERO;
                    knob.current_position = Vec2::ZERO;
                    knob.value = Vec2::ZERO;
                    stick_events.send(TouchStickEvent {
                        id: node.id.clone(),
                        event: TouchStickEventType::Up,
                        value: Vec2::ZERO,
                    });
                }
                _ => {}
            }
        }

        // Send event
        if (knob.value.x.abs() >= knob.dead_zone || knob.value.y.abs() >= knob.dead_zone)
            && knob.drag_id.is_some()
        {
            stick_events.send(TouchStickEvent {
                id: node.id.clone(),
                event: TouchStickEventType::Drag,
                value: knob.value,
            });
        }
    }
}

pub(crate) fn update_sticks(
    mut touch_events: EventReader<TouchInput>,
    mut send_values: EventWriter<DragEvent>,
) {
    let touches = touch_events
        .iter()
        .map(|e| (e.id, e.phase, e.position))
        .collect::<Vec<(u64, TouchPhase, Vec2)>>();

    for (id, phase, position) in &touches {
        match phase {
            TouchPhase::Started => {
                send_values.send(DragEvent::Start {
                    id: *id,
                    position: *position,
                });
            }
            TouchPhase::Moved => {
                send_values.send(DragEvent::Drag {
                    id: *id,
                    position: *position,
                });
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                send_values.send(DragEvent::End { id: *id });
            }
        }
    }
}

pub(crate) fn update_sticks_from_mouse(
    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_events: EventReader<MouseButtonInput>,
    mut drag_events: EventWriter<DragEvent>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = primary_window.single();
    let position = primary_window.cursor_position().unwrap_or(Vec2::ZERO);

    for mouse_event in mouse_events.iter() {
        if mouse_event.button == MouseButton::Left && mouse_event.state == ButtonState::Released {
            drag_events.send(DragEvent::End { id: 0 });
        }

        if mouse_event.button == MouseButton::Left && mouse_event.state == ButtonState::Pressed {
            drag_events.send(DragEvent::Start { id: 0, position });
        }
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        drag_events.send(DragEvent::Drag { id: 0, position });
    }
}
