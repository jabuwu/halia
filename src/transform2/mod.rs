//! Provides components for managing entity transforms in a 2D world.
//!
//! - [`Transform2`]
//! - [`Depth`]
//!
//! Please note that this module **does not** replace the need for Bevy's internal
//! [`Transform`](`bevy::prelude::Transform`) and entities that wish to use 2D transform components
//! must also have [`Transform`](`bevy::prelude::Transform`) and
//! [`GlobalTransform`](`bevy::prelude::GlobalTransform`).
//!
//! Feature flag: `halia_transform2`

mod transform2;

pub use transform2::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::{Depth, Transform2};
}
