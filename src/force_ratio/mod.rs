//! Provides the [`ForceRatio`] resource.
//!
//! Feature flag: `halia_force_ratio`

mod force_ratio;
pub use force_ratio::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::{ForceRatio, ForceRatioSystem};
}
