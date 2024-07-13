use crate::{StickIdType, TouchStick, TouchStickType};
use bevy::{
    prelude::*,
    render::{Extract, RenderApp},
    ui::{ContentSize, ExtractedUiNodes, FocusPolicy, RelativeCursorPosition, RenderUiSystem},
};
use std::marker::PhantomData;

/// Marker component for a `bevy_ui` Node area where sticks can be interacted with.
#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct TouchStickInteractionArea;

/// Marker component
#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct TouchStickUiKnob;

/// Marker component
#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct TouchStickUiOutline;

// TODO: default returns a broken bundle, should remove or fix
/// Touch stick ui bundle for easy spawning
#[derive(Bundle, Debug, Default)]
pub struct TouchStickUiBundle<S: StickIdType> {
    /// Data describing the [`TouchStick`] state
    pub stick: TouchStick<S>,
    /// Where this node will accept touch input
    pub interaction_area: TouchStickInteractionArea,
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// The calculated size based on the given image
    pub calculated_size: ContentSize,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
    /// Cursor position relative to the [`TouchStick`] in normalized logical pixels
    pub cursor_pos: RelativeCursorPosition,
}

pub(crate) struct TouchStickUiPlugin<S: StickIdType> {
    marker: PhantomData<S>,
}

impl<S: StickIdType> Default for TouchStickUiPlugin<S> {
    fn default() -> Self {
        Self { marker: default() }
    }
}

impl<S: StickIdType> Plugin for TouchStickUiPlugin<S> {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.add_systems(
            ExtractSchedule,
            patch_stick_node::<S>.after(RenderUiSystem::ExtractBackgrounds),
        );
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn patch_stick_node<S: StickIdType>(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    uinode_query: Extract<Query<(&Node, &GlobalTransform, &TouchStick<S>, &ViewVisibility)>>,
    knob_ui_query: Extract<Query<(Entity, &Parent), With<TouchStickUiKnob>>>,
    outline_ui_query: Extract<Query<(Entity, &Parent), With<TouchStickUiOutline>>>,
) {
    for (knob_entity, knob_parent) in &knob_ui_query {
        if let Ok((uinode, global_transform, stick, visibility)) = uinode_query.get(**knob_parent) {
            if visibility.get() && uinode.size().x != 0. && uinode.size().y != 0. {
                let radius = stick.radius;
                let axis_value = stick.value;
                // ui is y down, so we flip
                let pos = Vec2::new(axis_value.x, -axis_value.y) * radius;

                let base_pos = get_base_pos(uinode, stick, global_transform);
                let knob_pos = base_pos + pos.extend(0.);

                extracted_uinodes
                    .uinodes
                    .entry(knob_entity)
                    .and_modify(|node| {
                        node.transform = Mat4::from_translation(knob_pos);
                    });
            }
        }
    }

    for (outline_entity, outline_parent) in &outline_ui_query {
        if let Ok((uinode, global_transform, stick, visibility)) =
            uinode_query.get(**outline_parent)
        {
            if visibility.get() && uinode.size().x != 0. && uinode.size().y != 0. {
                let pos = get_base_pos(uinode, stick, global_transform);
                extracted_uinodes
                    .uinodes
                    .entry(outline_entity)
                    .and_modify(|node| {
                        node.transform = Mat4::from_translation(pos);
                    });
            }
        }
    }
}

fn get_base_pos<S: StickIdType>(
    uinode: &Node,
    stick: &TouchStick<S>,
    global_transform: &GlobalTransform,
) -> Vec3 {
    let container_rect = Rect {
        max: uinode.size(),
        ..default()
    };

    match stick.stick_type {
        TouchStickType::Fixed => global_transform
            .compute_matrix()
            .transform_point3((container_rect.center() - (uinode.size() / 2.)).extend(0.)),
        TouchStickType::Floating => {
            if stick.drag_id.is_none() {
                global_transform
                    .compute_matrix()
                    .transform_point3((container_rect.center() - (uinode.size() / 2.)).extend(0.))
            } else {
                stick.drag_start.extend(0.)
            }
        }
        TouchStickType::Dynamic => {
            if stick.drag_id.is_none() || stick.base_position == Vec2::ZERO {
                global_transform.translation()
            } else {
                stick.base_position.extend(0.)
            }
        }
    }
}
