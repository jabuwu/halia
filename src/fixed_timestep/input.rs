use std::hash::Hash;

use bevy::{input::InputSystem, prelude::*, reflect::Reflect};

use super::FixedSet;

/// System set for updating fixed timestep input state.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct FixedInputSystem;

/// A trait implemented by [`App`] allowing adding more fixed timestep input types.
pub trait AddFixedInput {
    /// Add a [`FixedInput`] version of [`Input`] for T. This is called automatically for
    /// [`KeyCode`], [`ScanCode`], [`MouseButton`], and [`GamepadButton`].
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self;
}

impl AddFixedInput for App {
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self {
        self.init_resource::<FixedInput<T>>().add_systems((
            fixed_input_update::<T>
                .in_base_set(CoreSet::PreUpdate)
                .after(InputSystem),
            fixed_input_clear::<T>
                .in_schedule(CoreSchedule::FixedUpdate)
                .in_set(FixedInputSystem)
                .in_base_set(FixedSet::PostUpdate),
        ));
        self
    }
}

pub(crate) struct FixedTimestepInputPlugin;

impl Plugin for FixedTimestepInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_input::<KeyCode>()
            .add_fixed_input::<ScanCode>()
            .add_fixed_input::<MouseButton>()
            .add_fixed_input::<GamepadButton>();
    }
}

/// A fixed timestep version of [`Input`]. Implements [`Deref`] and [`DerefMut`] for the internal
/// [`Input`] member, so see that struct for documentation.
#[derive(Clone, Deref, DerefMut, Debug, Reflect, Resource)]
#[reflect(Default)]
pub struct FixedInput<T: Copy + Eq + Hash + Send + Sync + 'static>(Input<T>);

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Default for FixedInput<T> {
    fn default() -> Self {
        Self(Input::default())
    }
}

fn fixed_input_update<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    input: Res<Input<T>>,
) {
    for pressed in input.get_just_pressed() {
        fixed_input.press(*pressed);
    }
    for released in input.get_just_released() {
        fixed_input.release(*released);
    }
}

fn fixed_input_clear<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
) {
    fixed_input.clear();
}
