//! Makes fixed timestep systems more usable.
//!
//! - Fixes events being dropped (see [`AddFixedEvent`]).
//! - Fixes inputs being dropped or double counted (see [`FixedInput`]).
//! - Adds a base set (see [`FixedSet`]).
//! - Fixes [`GlobalTransform`] not being updated.
//!
//! Fixed timestep systems are any system added to the [`CoreSchedule::FixedUpdate`] schedule.
//!
//! Feature flag: `halia_fixed_timestep`

use bevy::prelude::*;

/// Adds fixed timestep functionality.
///
/// Contained within [`HaliaPlugins`](`crate::HaliaPlugins`).
pub struct FixedTimestepPlugin;

impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FixedTimestepBaseSetPlugin)
            .add_plugin(FixedTimestepInputPlugin)
            .add_plugin(FixedTimestepPropagatePlugin);
    }
}

mod base_set;
mod events;
mod input;
mod transform;

pub use base_set::*;
pub use events::*;
pub use input::*;
pub use transform::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::{AddFixedEvent, FixedInput};
}
