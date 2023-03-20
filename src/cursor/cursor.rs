use bevy::{prelude::*, window::PrimaryWindow};

/// System set for updating [`Cursor`].
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CursorSystem {
    /// Updates `position` in [`Cursor`].
    Position,
}

/// Provides the [`Cursor`] resource.
///
/// Contained within [`HaliaPlugins`](`crate::HaliaPlugins`).
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>().add_system(
            cursor_position
                .in_set(CursorSystem::Position)
                .in_base_set(CoreSet::PreUpdate),
        );
    }
}

/// Helper resource for getting cursor information.
#[derive(Default, Resource, Debug)]
pub struct Cursor {
    /// The position of the cursor in the window, with 0,0 being the bottom left.
    ///
    /// Initially set to 0,0 and only updated when the cursor moves. Never updated on iOS and
    /// Android.
    ///
    /// Updated based on the [`PrimaryWindow`].
    pub window_position: Vec2,

    /// The position of the cursor in the world.
    ///
    /// Initially set to 0,0 and only updated when the cursor moves. Never updated on iOS and
    /// Android.
    ///
    /// Updated based on the [`PrimaryWindow`].
    pub world_position: Vec2,
}

fn cursor_position(
    mut cursor: ResMut<Cursor>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    cursor.window_position = cursor_position;
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };
    let Some(viewport_to_world) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    cursor.world_position = viewport_to_world.origin.truncate();
}
