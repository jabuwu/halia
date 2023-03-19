use std::time::Duration;

use bevy::prelude::*;

use crate::fixed_timestep::FixedSet;

/// Time to live system set.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum TimeToLiveSystem {
    /// Set for the time to live update system which is responsible for decrementing the time
    /// remaining and despawning the entity when it reaches 0.
    Update,
}

/// Adds time to live functionality.
///
/// Contained within [`HaliaPlugins`](`crate::HaliaPlugins`).
pub struct TimeToLivePlugin;

impl Plugin for TimeToLivePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            time_to_live_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(TimeToLiveSystem::Update)
                .in_base_set(FixedSet::PostUpdate),
        );
    }
}

/// A component which will recursively despawn the entity its attached to after a specified amount
/// of time.
#[derive(Component, Debug, Clone, Copy)]
pub struct TimeToLive {
    /// The amount of time left before the entity is despawned.
    pub time_remaining: f32,
}

impl TimeToLive {
    /// Instantiate a new [`TimeToLive`] component with time remaining set to `duration`.
    pub fn new(duration: Duration) -> Self {
        Self {
            time_remaining: duration.as_secs_f32(),
        }
    }
}

fn time_to_live_update(
    mut time_to_live_query: Query<(Entity, &mut TimeToLive)>,
    mut commands: Commands,
    time: Res<FixedTime>,
) {
    for (time_to_live_entity, mut time_to_live) in time_to_live_query.iter_mut() {
        if time_to_live.time_remaining <= 0. {
            if let Some(entity_commands) = commands.get_entity(time_to_live_entity) {
                entity_commands.despawn_recursive();
            }
        }
        time_to_live.time_remaining -= time.period.as_secs_f32();
    }
}
