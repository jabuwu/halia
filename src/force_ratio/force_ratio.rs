use bevy::{prelude::*, transform::TransformSystem};

use crate::Persistent;

const RATIO_BAR_SIZE: f32 = 100_000.;

/// System set for force ratio systems.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ForceRatioSystem {
    /// A [`CoreSchedule::Startup`] system that creates black bars ([`ForceRatioBar`]) on the edges
    /// of the screen.
    Setup,
    /// A [`CoreSet::PostUpdate`] system that updates the camera's scale and adjusts
    /// [`ForceRatioBar`] entity positions.
    Update,
}

/// Adds force ratio functionality, configurable with the [`ForceRatio`] resource.
///
/// Contained within [`HaliaPlugins`](`crate::HaliaPlugins`).
pub struct ForceRatioPlugin;

impl Plugin for ForceRatioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ForceRatio>()
            .add_startup_system(force_ratio_setup.in_set(ForceRatioSystem::Setup))
            .add_system(
                force_ratio_update
                    .in_set(ForceRatioSystem::Update)
                    .in_base_set(CoreSet::PostUpdate)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

/// A utility to force the screen to appear at a specified resolution.
///
/// Forcing the screen ratio allows a game to be authored at only one resolution, either upscaling
/// or downscaling if drawn at a different resolution. Black bars ([`ForceRatioBar`]) are placed
/// around the screens to block out view if the screen ratio is not the same as the desired
/// resolution.
#[derive(Default, Resource, Copy, Clone, PartialEq)]
pub enum ForceRatio {
    /// Force ratio is disabled.
    #[default]
    Disabled,
    /// Force ratio is enabled with the specified resolution.
    Enabled {
        /// The resolution's width
        width: f32,
        /// The resolution's height
        height: f32,
    },
}

impl ForceRatio {
    /// Disable forced ratio.
    ///
    /// This will not "reset" the camera's scale, so it may be in an invalid state after disabling
    /// forced ratio.
    pub fn disable(&mut self) {
        *self = Self::Disabled;
    }

    /// Enable forced ratio with the given resolution.
    pub fn enable(&mut self, width: f32, height: f32) {
        *self = Self::Enabled { width, height };
    }
}

/// Black bar entities created at the edges of the screen to block out view when the screen's
/// resolution is not the desired resolution.
///
/// These entities are not visible if force ratio is disabled.
#[derive(Component, Clone, Copy)]
pub enum ForceRatioBar {
    /// The bar at the top of the screen.
    Top,
    /// The bar at the bottom of the screen.
    Bottom,
    /// The bar at the left of the screen.
    Left,
    /// The bar at the right of the screen.
    Right,
}

impl ForceRatioBar {
    fn visibility(force_ratio: &ForceRatio) -> Visibility {
        if matches!(force_ratio, &ForceRatio::Disabled) {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        }
    }
    fn translation(self, force_ratio: &ForceRatio) -> Vec3 {
        match force_ratio {
            ForceRatio::Disabled => Vec3::ZERO,
            ForceRatio::Enabled { width, height } => match self {
                ForceRatioBar::Top => Vec3::new(0., height * 0.5 + RATIO_BAR_SIZE * 0.5, 1.),
                ForceRatioBar::Bottom => Vec3::new(0., height * -0.5 - RATIO_BAR_SIZE * 0.5, 1.),
                ForceRatioBar::Left => Vec3::new(width * -0.5 - RATIO_BAR_SIZE * 0.5, 0., 1.),
                ForceRatioBar::Right => Vec3::new(width * 0.5 + RATIO_BAR_SIZE * 0.5, 0., 1.),
            },
        }
    }
}

fn force_ratio_setup(mut commands: Commands, force_ratio: Res<ForceRatio>) {
    for side in [
        ForceRatioBar::Top,
        ForceRatioBar::Bottom,
        ForceRatioBar::Left,
        ForceRatioBar::Right,
    ] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(RATIO_BAR_SIZE)),
                    color: Color::BLACK,
                    ..Default::default()
                },
                visibility: ForceRatioBar::visibility(force_ratio.as_ref()),
                transform: Transform::from_translation(side.translation(force_ratio.as_ref())),
                ..Default::default()
            },
            Persistent,
            side,
        ));
    }
}

fn force_ratio_update(
    mut transform_query: Query<&mut Transform>,
    mut visibility_query: Query<&mut Visibility>,
    camera_query: Query<Entity, With<Camera>>,
    bar_query: Query<(Entity, &ForceRatioBar)>,
    window_query: Query<&Window>,
    force_ratio: Res<ForceRatio>,
) {
    if let ForceRatio::Enabled { width, height } = force_ratio.as_ref() {
        if let Ok(window) = window_query.get_single() {
            for camera_entity in camera_query.iter() {
                if let Ok(mut camera_transform) = transform_query.get_mut(camera_entity) {
                    let ratio = window.width() / window.height();
                    let mut desired_width = *width;
                    let mut desired_height = *height;
                    let desired_ratio = desired_width / desired_height;
                    if ratio > desired_ratio {
                        desired_width *= ratio / desired_ratio;
                    } else {
                        desired_height *= desired_ratio / ratio;
                    }
                    camera_transform.scale.x = desired_width / window.width();
                    camera_transform.scale.y = desired_height / window.height();
                }
            }
        }
    }
    for (bar_entity, bar) in bar_query.iter() {
        if let Ok(mut bar_transform) = transform_query.get_mut(bar_entity) {
            bar_transform.translation = bar.translation(force_ratio.as_ref());
        }
        if let Ok(mut bar_visibility) = visibility_query.get_mut(bar_entity) {
            *bar_visibility = ForceRatioBar::visibility(force_ratio.as_ref());
        }
    }
}
