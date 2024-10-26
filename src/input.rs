use crate::{StickIdType, TouchStick, TouchStickEvent, TouchStickEventType, TouchStickType};
use bevy::{
    input::{mouse::MouseButtonInput, touch::TouchPhase, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Event)]
/// Actual device input passed too [`TouchStick`]
pub(crate) enum DragEvent {
    Start { id: u64, position: Vec2 },
    Drag { id: u64, position: Vec2 },
    End { id: u64 },
}

pub(crate) fn update_sticks_from_drag_events<S: StickIdType>(
    mut drag_events: EventReader<DragEvent>,
    mut stick_events: EventWriter<TouchStickEvent<S>>,
    mut sticks: Query<&mut TouchStick<S>>,
) {
    let input_events = drag_events.read().collect::<Vec<&DragEvent>>();

    for mut stick in &mut sticks {
        for event in &input_events {
            match event {
                DragEvent::Start { id, position } => {
                    if stick.interactable_zone.contains(*position) && stick.drag_id != Some(*id) {
                        stick.drag_id = Some(*id);
                        stick.drag_start = *position;
                        stick.drag_position = *position;
                        stick.value = Vec2::ZERO;
                        stick_events.send(TouchStickEvent {
                            id: stick.id.clone(),
                            event: TouchStickEventType::Press,
                            value: Vec2::ZERO,
                        });
                    }
                }
                DragEvent::Drag { id, position: pos } if Some(*id) == stick.drag_id => {
                    stick.drag_position = *pos;
                    let radius = stick.radius;
                    if stick.stick_type == TouchStickType::Dynamic {
                        stick.base_position = *pos;
                        let to_knob = stick.drag_position - stick.drag_start;
                        let distance_to_knob = to_knob.length();
                        if distance_to_knob > radius {
                            let excess_distance = distance_to_knob - radius;
                            stick.drag_start += to_knob.normalize() * excess_distance;
                        }
                    }
                    let d = (stick.drag_position - stick.drag_start) / radius;
                    let length = d.length();
                    // input events are y positive down, so we flip it
                    stick.value = Vec2::new(d.x, -d.y) / length.max(1.);
                }
                DragEvent::End { id } if Some(*id) == stick.drag_id => {
                    stick.drag_id = None;
                    stick.base_position = Vec2::ZERO;
                    stick.drag_start = Vec2::ZERO;
                    stick.drag_position = Vec2::ZERO;
                    stick.value = Vec2::ZERO;
                    stick_events.send(TouchStickEvent {
                        id: stick.id.clone(),
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
                id: stick.id.clone(),
                event: TouchStickEventType::Drag,
                value: stick.value,
            });
        }
    }
}

pub(crate) fn send_drag_events_from_touch(
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

pub(crate) fn send_drag_events_from_mouse(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_events: EventReader<MouseButtonInput>,
    mut drag_events: EventWriter<DragEvent>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(primary_window) = primary_window.get_single() else {
        return
    };

    let position = primary_window.cursor_position();

    for mouse_event in mouse_events.read() {
        if mouse_event.button == MouseButton::Left && mouse_event.state == ButtonState::Released {
            drag_events.send(DragEvent::End { id: 0 });
        }

        if mouse_event.button == MouseButton::Left && mouse_event.state == ButtonState::Pressed {
            drag_events.send(DragEvent::Start {
                id: 0,
                position: position.unwrap_or_default(),
            });
        }
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        // if the mouse is outside the window, we'll still get pressed state,
        // but we won't get the position. So in that case, we stop sending drag
        // events.
        if let Some(position) = position {
            drag_events.send(DragEvent::Drag { id: 0, position });
        }
    }
}
