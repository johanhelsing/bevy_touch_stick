use crate::{
    StickIdType, TouchStick, TouchStickEvent, TouchStickEventType, TouchStickNode, TouchStickType,
};
use bevy::{
    input::{mouse::MouseButtonInput, touch::TouchPhase, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Event)]
pub(crate) enum DragEvent {
    Start { id: u64, position: Vec2 },
    Drag { id: u64, position: Vec2 },
    End { id: u64 },
}

pub(crate) fn update_input<S: StickIdType>(
    mut drag_events: EventReader<DragEvent>,
    mut stick_events: EventWriter<TouchStickEvent<S>>,
    mut sticks: Query<(&TouchStickNode<S>, &mut TouchStick<S>)>,
) {
    let input_events = drag_events.read().collect::<Vec<&DragEvent>>();

    for (node, mut stick) in sticks.iter_mut() {
        for event in &input_events {
            match event {
                DragEvent::Start { id, position } => {
                    if stick.interactable_zone.contains(*position) && stick.drag_id != Some(*id) {
                        stick.drag_id = Some(*id);
                        stick.start_position = *position;
                        stick.current_position = *position;
                        stick.value = Vec2::ZERO;
                        stick_events.send(TouchStickEvent {
                            id: stick.id.clone(),
                            event: TouchStickEventType::Press,
                            value: Vec2::ZERO,
                        });
                    }
                }
                DragEvent::Drag { id, position: pos } if Some(*id) == stick.drag_id => {
                    stick.current_position = *pos;
                    let half = stick.interactable_zone.half_size();
                    if stick.stick_type == TouchStickType::Dynamic {
                        stick.base_position = *pos;
                        let to_knob = stick.current_position - stick.start_position;
                        let distance_to_knob = to_knob.length();
                        if distance_to_knob > half.x {
                            let excess_distance = distance_to_knob - half.x;
                            stick.start_position += to_knob.normalize() * excess_distance;
                        }
                    }
                    let d = (stick.current_position - stick.start_position) / half;
                    let length = d.length();
                    // input events are y positive down, so we flip it
                    stick.value = Vec2::new(d.x, -d.y) / length.max(1.);
                }
                DragEvent::End { id } if Some(*id) == stick.drag_id => {
                    stick.drag_id = None;
                    stick.base_position = Vec2::ZERO;
                    stick.start_position = Vec2::ZERO;
                    stick.current_position = Vec2::ZERO;
                    stick.value = Vec2::ZERO;
                    stick_events.send(TouchStickEvent {
                        id: node.id.clone(),
                        event: TouchStickEventType::Release,
                        value: Vec2::ZERO,
                    });
                }
                _ => {}
            }
        }

        // Send event
        if (stick.value.x.abs() >= stick.dead_zone || stick.value.y.abs() >= stick.dead_zone)
            && stick.drag_id.is_some()
        {
            stick_events.send(TouchStickEvent {
                id: node.id.clone(),
                event: TouchStickEventType::Drag,
                value: stick.value,
            });
        }
    }
}

pub(crate) fn update_sticks_from_touch(
    mut touch_events: EventReader<TouchInput>,
    mut send_values: EventWriter<DragEvent>,
) {
    let touches = touch_events
        .read()
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

    for mouse_event in mouse_events.read() {
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
