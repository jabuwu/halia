use std::marker::PhantomData;

use bevy::prelude::*;

/// A trait implemented by [`App`], similar to [`App::add_event`], which works properly with fixed
/// timestep systems. Events will continue to work properly in non-fixed timestep systems as well.
///
/// ```
/// // before:
/// App::new().add_event::<MyEvent>();
///
/// // after:
/// App::new().add_fixed_event::<MyEvent>();
/// ```
pub trait AddFixedEvent {
    /// Setup the application to manage events of type T (similar to [`App::add_event`]), but which
    /// also work in fixed timestep systems.
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self;
}

impl AddFixedEvent for App {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self {
        self.init_resource::<EventClearFlag<T>>()
            .init_resource::<Events<T>>()
            .add_systems((fixed_events_clear_flag::<T>,).in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((fixed_events_clear::<T>,).in_base_set(CoreSet::Last));
        self
    }
}

#[derive(Resource)]
struct EventClearFlag<T: Event> {
    clear: bool,
    _marker: PhantomData<T>,
}

impl<T: Event> Default for EventClearFlag<T> {
    fn default() -> Self {
        Self {
            clear: false,
            _marker: PhantomData,
        }
    }
}

fn fixed_events_clear_flag<T: Event>(mut event_clear_flag: ResMut<EventClearFlag<T>>) {
    event_clear_flag.clear = true;
}

fn fixed_events_clear<T: Event>(
    mut event_clear_flag: ResMut<EventClearFlag<T>>,
    mut fixed_events: ResMut<Events<T>>,
) {
    if event_clear_flag.clear {
        fixed_events.update();
        event_clear_flag.clear = false;
    }
}
