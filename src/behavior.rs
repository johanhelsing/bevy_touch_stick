use bevy::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// how the touch stick is positioned onscreen
///
/// - `Fixed`: Static position onscreen
/// - `Floating`: spawn at pressed point
/// - `Dynamic`: follow point on drag
#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TouchStickType {
    /// Static position
    Fixed,
    #[default]
    /// Spawn at point click
    Floating,
    /// Follow point on drag
    Dynamic,
}
