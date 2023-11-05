use crate::{StickIdType, TouchStick};
use bevy::{
    prelude::*,
    ui::{ContentSize, FocusPolicy, RelativeCursorPosition},
};
use std::hash::Hash;

/// Marker component for a bevy_ui Node area where sticks can be interacted with.
#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct TouchStickInteractionArea;

// TODO: default returns a broken bundle, should remove or fix
#[derive(Bundle, Debug, Default)]
pub struct TouchStickUiBundle<S: StickIdType> {
    pub stick: TouchStick<S>,

    pub stick_node: TouchStickNode<S>,

    /// Indicates that this node may be interacted with
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

    pub cursor_pos: RelativeCursorPosition,
}

// todo: deriving Default for this is a mistake
/// bevy ui config for a stick
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct TouchStickNode<S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + 'static>
{
    /// Identifier of joystick
    pub id: S,
    /// Radius for knob on joystick
    pub knob_radius: f32,
    /// Radius for ring around the stick knob
    pub outline_radius: f32,
}

pub(crate) fn update_stick_ui() {}

// #[allow(clippy::type_complexity)]
// pub fn extract_joystick_node<
//     S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + 'static,
// >(
//     mut extracted_uinodes: ResMut<ExtractedUiNodes>,
//     images: Extract<Res<Assets<Image>>>,
//     ui_stack: Extract<Res<UiStack>>,
//     uinode_query: Extract<
//         Query<(
//             Entity,
//             &Node,
//             &GlobalTransform,
//             &TintColor,
//             &TouchStickNode<S>,
//             &ViewVisibility,
//             &TouchStick,
//         )>,
//     >,
// ) {
//     for (stack_index, entity) in ui_stack.uinodes.iter().enumerate() {
//         let stack_index = stack_index as u32;

//         if let Ok((entity, uinode, global_transform, color, joystick_node, visibility, data)) =
//             uinode_query.get(*entity)
//         {
//             if !visibility.get()
//                 || uinode.size().x == 0.
//                 || uinode.size().y == 0.
//                 || color.0.a() == 0.0
//                 || !images.contains(&joystick_node.border_image)
//                 || !images.contains(&joystick_node.knob_image)
//                 || data.drag_id.is_none() && joystick_node.behavior == TouchStickType::Dynamic
//             {
//                 continue;
//             }
//             let container_rect = Rect {
//                 max: uinode.size(),
//                 ..default()
//             };

//             let border_pos = match joystick_node.behavior {
//                 TouchStickType::Fixed => global_transform
//                     .compute_matrix()
//                     .transform_point3((container_rect.center() - (uinode.size() / 2.)).extend(0.)),
//                 TouchStickType::Floating => {
//                     if data.drag_id.is_none() {
//                         global_transform.compute_matrix().transform_point3(
//                             (container_rect.center() - (uinode.size() / 2.)).extend(0.),
//                         )
//                     } else {
//                         data.start_position.extend(0.)
//                     }
//                 }
//                 TouchStickType::Dynamic => data.base_position.extend(0.),
//             };

//             extracted_uinodes.uinodes.insert(
//                 entity,
//                 ExtractedUiNode {
//                     stack_index,
//                     transform: Mat4::from_translation(border_pos),
//                     color: color.0,
//                     rect: container_rect,
//                     image: joystick_node.border_image.id(),
//                     atlas_size: None,
//                     clip: None,
//                     flip_x: false,
//                     flip_y: false,
//                 },
//             );

//             let rect = Rect {
//                 max: joystick_node.knob_size,
//                 ..default()
//             };

//             let radius = uinode.size().x / 2.;
//             let axis_value = data.value;
//             // ui is y down, so we flip
//             let pos = Vec2::new(axis_value.x, -axis_value.y) * radius;

//             let knob_pos = match joystick_node.behavior {
//                 TouchStickType::Fixed => global_transform.compute_matrix().transform_point3(
//                     (container_rect.center() - (uinode.size() / 2.) + pos).extend(0.),
//                 ),
//                 TouchStickType::Floating => {
//                     if data.drag_id.is_none() {
//                         global_transform.compute_matrix().transform_point3(
//                             (container_rect.center() - (uinode.size() / 2.)).extend(0.),
//                         )
//                     } else {
//                         (data.start_position + pos).extend(0.)
//                     }
//                 }
//                 TouchStickType::Dynamic => (data.base_position + pos).extend(0.),
//             };

//             extracted_uinodes.uinodes.insert(
//                 entity,
//                 ExtractedUiNode {
//                     rect,
//                     stack_index,
//                     transform: Mat4::from_translation(knob_pos),
//                     color: color.0,
//                     image: joystick_node.knob_image.id(),
//                     atlas_size: None,
//                     clip: None,
//                     flip_x: false,
//                     flip_y: false,
//                 },
//             );
//         }
//     }
// }
