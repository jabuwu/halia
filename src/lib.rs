//!  Extremely opinionated Bevy plugins for 2D games.

#![deny(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
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

features!(
    ("halia_fixed_timestep", fixed_timestep, FixedTimestepPlugin),
    ("halia_transform2", transform2, Transform2Plugin)
);
