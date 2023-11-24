use bevy::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// How the touch stick is positioned onscreen
#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TouchStickType {
    /// Static position
    Fixed,
    #[default]
    /// Spawn at pressed point
    Floating,
    /// Follow point on drag
    Dynamic,
}
