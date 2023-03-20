//! Provides the [`SubAssets`] resource.
//!
//! Feature flag: `halia_sub_assets`

mod sub_assets;
pub use sub_assets::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::SubAssets;
}
