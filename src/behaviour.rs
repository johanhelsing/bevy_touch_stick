use bevy::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
