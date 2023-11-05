use crate::{TouchStick, TouchStickType};
use bevy::{
    prelude::*,
    ui::{ContentSize, FocusPolicy, RelativeCursorPosition},
};
use std::hash::Hash;

/// The tint color of the image
///
/// When combined with [`VirtualJoystickNode`], tints the provided texture, while still
/// respecting transparent areas.
#[derive(Component, Copy, Clone, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct TintColor(pub Color);

impl TintColor {
    pub const DEFAULT: Self = Self(Color::WHITE);
}

impl Default for TintColor {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<Color> for TintColor {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

/// Marker component for a bevy_ui Node area where sticks can be interacted with.
#[derive(Component, Copy, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct TouchStickInteractionArea;

// TODO: default returns a broken bundle, should remove or fix
#[derive(Bundle, Debug, Default)]
pub struct TouchStickBundle<
    S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + 'static,
> {
    /// Indicates that this node may be interacted with
    pub(crate) interaction_area: TouchStickInteractionArea,
    /// Describes the size of the node
    pub(crate) node: Node,
    /// Describes the style including flexbox settings
    pub(crate) style: Style,
    /// The calculated size based on the given image
    pub(crate) calculated_size: ContentSize,
    /// The tint color of the image
    pub(crate) color: TintColor,
    pub(crate) joystick: TouchStickNode<S>,
    /// Whether this node should block interaction with lower nodes
    pub(crate) focus_policy: FocusPolicy,
    /// The transform of the node
    pub(crate) transform: Transform,
    /// The global transform of the node
    pub(crate) global_transform: GlobalTransform,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub(crate) z_index: ZIndex,
    pub(crate) stick: TouchStick,
    pub(crate) cursor_pos: RelativeCursorPosition,
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
    /// Define the behavior of joystick
    pub behavior: TouchStickType,
}

impl<S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + 'static>
    TouchStickBundle<S>
{
    pub fn new(joystick: TouchStickNode<S>) -> Self {
        Self {
            joystick,
            ..default()
        }
    }

    pub fn set_node(mut self, node: Node) -> Self {
        self.node = node;
        self
    }

    pub fn set_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn set_color(mut self, color: TintColor) -> Self {
        self.color = color;
        self
    }

    pub fn set_focus_policy(mut self, focus_policy: FocusPolicy) -> Self {
        self.focus_policy = focus_policy;
        self
    }

    pub fn set_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn set_global_transform(mut self, global_transform: GlobalTransform) -> Self {
        self.global_transform = global_transform;
        self
    }

    pub fn set_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn set_z_index(mut self, z_index: ZIndex) -> Self {
        self.z_index = z_index;
        self
    }
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
