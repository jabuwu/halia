use bevy::prelude::*;

/// A base set (similar to [`CoreSet`]), but for fixed timestep systems.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
#[system_set(base)]
pub enum FixedSet {
    /// Runs before [`FixedSet::Update`].
    PreUpdate,
    /// The copy of [`apply_system_buffers`] that runs immediately after `PreUpdate`.
    PreUpdateFlush,
    /// Responsible for doing most app logic.
    Update,
    /// The copy of [`apply_system_buffers`] that runs immediately after `Update`.
    UpdateFlush,
    /// Runs after [`FixedSet::Update`].
    PostUpdate,
    /// The copy of [`apply_system_buffers`] that runs immediately after `PostUpdate`.
    PostUpdateFlush,
}

pub(crate) struct FixedTimestepBaseSetPlugin;

impl Plugin for FixedTimestepBaseSetPlugin {
    fn build(&self, app: &mut App) {
        {
            let Some(schedule) = app.get_schedule_mut(CoreSchedule::FixedUpdate) else {
                warn!("halia_fixed_timestep relies on CoreSchedule::FixedUpdate, but it was not found");
                return;
            };
            schedule
                .set_default_base_set(FixedSet::Update)
                .configure_set(FixedSet::PreUpdate.before(FixedSet::PreUpdateFlush))
                .configure_set(FixedSet::PreUpdateFlush.before(FixedSet::Update))
                .configure_set(FixedSet::Update.before(FixedSet::UpdateFlush))
                .configure_set(FixedSet::UpdateFlush.before(FixedSet::PostUpdate))
                .configure_set(FixedSet::PostUpdate.before(FixedSet::PostUpdateFlush));
        }
        app.add_systems(
            (
                apply_system_buffers.in_base_set(FixedSet::PreUpdateFlush),
                apply_system_buffers.in_base_set(FixedSet::UpdateFlush),
                apply_system_buffers.in_base_set(FixedSet::PostUpdateFlush),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        );
    }
}
