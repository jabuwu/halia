//! Provides the [`TimeToLive`] component.
//!
//! Feature flag: `halia_time_to_live`

mod time_to_live;
pub use time_to_live::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::TimeToLive;
}
