//! Provides helper sets.
//!
//! - [`EventSet`]
//!
//! Feature flag: `halia_sets`

use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::prelude::*;

/// A generic set meant to be used with events. `T` represents the event.
///
/// ```
/// # use bevy::prelude::*;
/// # use halia::prelude::*;
/// #[derive(Default)]
/// pub struct MyEvent;
///
/// // Order two systems based on event sending and receiving.
/// App::new()
///     .add_system(my_event_sender.in_set(EventSet::<MyEvent>::Sender))
///     .add_system(my_event_receiver.after(EventSet::<MyEvent>::Sender))
///     .run();
/// # fn my_event_sender() {}
/// # fn my_event_receiver() {}
/// ```
#[derive(Copy, SystemSet)]
pub enum EventSet<T: Send + Sync + 'static> {
    /// A set indicating that a system sends event `T`.
    ///
    /// Add systems which send `T` into this set, and systems which receive `T` after this set.
    Sender,
    #[doc(hidden)]
    #[system_set(ignore_field)]
    _Data(PhantomData<T>),
}

impl<T: Send + Sync + 'static> Clone for EventSet<T> {
    fn clone(&self) -> Self {
        Self::Sender
    }
}

impl<T: Send + Sync + 'static> Debug for EventSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sender => {
                f.write_str("Sender")?;
            }
            Self::_Data(..) => unreachable!(),
        }
        Ok(())
    }
}

impl<T: Send + Sync + 'static> Hash for EventSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Sender => {
                state.write_u32(0);
            }
            Self::_Data(..) => unreachable!(),
        }
    }
}

impl<T: Send + Sync + 'static> PartialEq for EventSet<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Sender => match other {
                Self::Sender => true,
                Self::_Data(..) => unreachable!(),
            },
            Self::_Data(..) => unreachable!(),
        }
    }
}

impl<T: Send + Sync + 'static> Eq for EventSet<T> {}

/// Configures helper sets.
///
/// Contained within [`HaliaPlugins`](`crate::HaliaPlugins`).
pub struct SetsPlugin;

impl Plugin for SetsPlugin {
    fn build(&self, _app: &mut App) {}
}

#[doc(hidden)]
pub mod prelude {
    pub use super::EventSet;
}
