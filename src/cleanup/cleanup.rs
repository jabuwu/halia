use bevy::prelude::*;

use crate::Persistent;

/// A trait implemented by [`App`] to provide state cleanup.
pub trait AddStateCleanup {
    /// Adds [`cleanup_nonpersistent_entities`] to each state's [`OnExit`] schedule.
    fn add_state_cleanup<T: States>(&mut self) -> &mut Self;
}

impl AddStateCleanup for App {
    fn add_state_cleanup<T: States>(&mut self) -> &mut Self {
        for state in T::variants() {
            self.add_system(cleanup_nonpersistent_entities.in_schedule(OnExit(state)));
        }
        self
    }
}

/// Recursively despawn parentless entities that are not marked [`Persistent`].
pub fn cleanup_nonpersistent_entities(
    mut commands: Commands,
    entity_query: Query<Entity, (Without<Parent>, Without<Persistent>, Without<Window>)>,
) {
    for entity in entity_query.iter() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
