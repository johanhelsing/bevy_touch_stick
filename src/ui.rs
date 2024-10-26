use crate::{StickIdType, TouchStick, TouchStickType};
use bevy::{
    prelude::*,
    ui::{ContentSize, FocusPolicy, RelativeCursorPosition},
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
    pub background_color: BackgroundColor,
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
        app.add_systems(
            Update,
            patch_stick_node::<S>, // .after(UiSystem::Stack), // .before(RenderUiSystem::ExtractBackgrounds),
        );
    }
}

// TODO: maybe better as paramter on touchstick?
const MOVE_PERCENT_RELATIVE: f32 = 0.25_f32;

#[allow(clippy::type_complexity)]
pub(crate) fn patch_stick_node<S: StickIdType>(
    uinode_query: Query<
        (&Node, &TouchStick<S>, &ViewVisibility),
        (Without<TouchStickUiKnob>, Without<TouchStickUiOutline>),
    >,
    mut knob_ui_query: Query<
        (&Parent, &mut Style),
        (With<TouchStickUiKnob>, Without<TouchStickUiOutline>),
    >,
    mut outline_ui_query: Query<
        (&Parent, &mut Style),
        (With<TouchStickUiOutline>, Without<TouchStickUiKnob>),
    >,
) {

    for (knob_parent, mut style) in &mut knob_ui_query {
        if let Ok((uinode, stick, visibility)) = uinode_query.get(**knob_parent) {
            if stick.stick_type == TouchStickType::Floating {
                if stick.value != Vec2::ZERO {
                    style.display = Display::Flex;
                    if visibility.get() && uinode.size().x != 0. && uinode.size().y != 0. {
                        let radius = stick.radius;
                        let axis_value = stick.value;
                        // ui is y down, so we flip
                        let pos = Vec2::new(axis_value.x, axis_value.y) * radius;

                        // transform.translation = pos;
                        style.left = Val::Px(pos.x);
                        style.bottom = Val::Px(pos.y);
                    }
                } else {
                    style.display = Display::None;
                }
                // show stick
            } else if stick.stick_type == TouchStickType::Dynamic {
                style.display = Display::Flex;
                if visibility.get() && uinode.size().x != 0. && uinode.size().y != 0. {
                    let radius = stick.radius;
                    let axis_value = stick.value;
                    let pos = Vec2::new(axis_value.x, axis_value.y) * radius;

                    // transform.translation = pos;
                    style.left = Val::Px(pos.x);
                    style.bottom = Val::Px(pos.y);
                }
            } else if stick.stick_type == TouchStickType::Fixed {
                style.display = Display::Flex;
                if visibility.get() && uinode.size().x != 0. && uinode.size().y != 0. {
                    // convert stick value from magnitude too percent
                    let axis_value = stick
                        .value
                        .clamp(Vec2::splat(-0.5), Vec2::splat(0.5))
                        .clamp_length(0.0, 0.5)
                        * stick.radius;

                    let pos = Vec2::new(axis_value.x, axis_value.y);

                    // transform.translation = pos;
                    style.left = Val::Px(pos.x);
                    style.bottom = Val::Px(pos.y);
                }
            }
        }
    }
    
    for (outline_parent, mut style) in &mut outline_ui_query {
        if let Ok((uinode, stick, visibility)) = uinode_query.get(**outline_parent) {
            if stick.stick_type == TouchStickType::Floating {
                if stick.value != Vec2::ZERO {
                    style.display = Display::Flex;
                } else {
                    style.display = Display::None;
                }
                // show stick
            } else if stick.stick_type == TouchStickType::Dynamic {
                style.display = Display::Flex;
                if visibility.get() && uinode.size().x != 0. && uinode.size().y != 0. {
                    let radius = stick.radius;

                    // only offset a small fraction of the knobs value
                    let axis_value = stick.value * MOVE_PERCENT_RELATIVE;
                    let pos = axis_value * radius;

                    // transform.translation = pos;
                    style.left = Val::Px(pos.x);
                    style.bottom = Val::Px(pos.y);
                }
            } else if stick.stick_type == TouchStickType::Fixed {
                // skip because position should not change
                style.display = Display::Flex;
            }
        }
    }
}

