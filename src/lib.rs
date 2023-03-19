use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct HaliaPlugins;

impl PluginGroup for HaliaPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}

pub mod prelude {
    pub use crate::HaliaPlugins;
}
