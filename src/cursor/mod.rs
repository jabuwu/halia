//! Provides the [`Cursor`] resource.
//!
//! Feature flag: `halia_cursor`

mod cursor;
pub use cursor::*;

#[doc(hidden)]
pub mod prelude {
    pub use super::Cursor;
}
