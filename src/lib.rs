//!  Extremely opinionated Bevy plugins for 2D games.

#![deny(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::redundant_closure_for_method_calls,
    clippy::type_complexity
)]

use bevy::{app::PluginGroupBuilder, prelude::*};

macro_rules! features {
    ($(($feature:literal, $mod:ident, $plugin:ident)),+) => {
        $(
            #[cfg(feature = $feature)]
            pub mod $mod;
        )+

        /// Collection of all enabled Halia plugins.
        pub struct HaliaPlugins;

        impl PluginGroup for HaliaPlugins {
            fn build(self) -> PluginGroupBuilder {
                #[allow(unused_mut)]
                let mut group = PluginGroupBuilder::start::<Self>();

                $(
                    #[cfg(feature = $feature)]
                    {
                        group = group.add(crate::$mod::$plugin);
                    }
                )+

                group
            }
        }

        #[doc(hidden)]
        pub mod prelude {
            pub use super::{HaliaPlugins};
            $(
                #[cfg(feature = $feature)]
                pub use super::$mod::prelude::*;
            )+
        }
    }
}

#[rustfmt::skip]
features!(
    ("halia_cleanup", cleanup, CleanupPlugin),
    ("halia_cursor", cursor, CursorPlugin),
    ("halia_fixed_timestep", fixed_timestep, FixedTimestepPlugin),
    ("halia_force_ratio", force_ratio, ForceRatioPlugin),
    ("halia_sets", sets, SetsPlugin),
    ("halia_time_to_live", time_to_live, TimeToLivePlugin),
    ("halia_transform2", transform2, Transform2Plugin)
);

/// A marker component indicating that an entity must not be automatically despawned by state
/// transitions or other cleanup systems.
///
/// This should only be added to parentless entities since cleanup functions search for parent
/// entities and despawn recursively.
#[derive(Clone, Component, Copy, Default)]
pub struct Persistent;
