use bevy::{
    prelude::*,
    transform::systems::{propagate_transforms, sync_simple_transforms},
};

use crate::transform2::transform2_propagate;

use super::FixedSet;

/// Fixed timestep transform propagation system set.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum FixedTransformSystem {
    /// Fixed timestep version of
    /// [`Transform2System::Transform2Propagate`](`crate::transform2::Transform2System::Transform2Propagate`).
    Transform2Propagate,
    /// Fixed timestep version of
    /// [`TransformSystem::TransformPropagate`](`bevy::transform::TransformSystem::TransformPropagate`).
    TransformPropagate,
}

pub(crate) struct FixedTimestepPropagatePlugin;

impl Plugin for FixedTimestepPropagatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            transform2_propagate
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedTransformSystem::Transform2Propagate)
                .in_base_set(FixedSet::PostUpdate)
                .before(FixedTransformSystem::TransformPropagate),
            sync_simple_transforms
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedTransformSystem::TransformPropagate)
                .in_base_set(FixedSet::PostUpdate),
            propagate_transforms
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedTransformSystem::TransformPropagate)
                .in_base_set(FixedSet::PostUpdate),
        ));
    }
}
