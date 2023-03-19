//! Provides cleanup utilities.
//!
//! - [`AddStateCleanup`]
//!
//! Feature flag: `halia_cleanup`

use bevy::prelude::*;

/// Adds cleanup functionality.
///
/// Contained within [`HaliaPlugins`](`crate::HaliaPlugins`).
pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, _app: &mut App) {}
}

mod cleanup;

pub use cleanup::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::AddStateCleanup;
}
