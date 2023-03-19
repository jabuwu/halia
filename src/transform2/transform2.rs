use bevy::{prelude::*, transform::TransformSystem};

/// System set for [`transform2`](`crate::transform2`) systems.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum Transform2System {
    /// Set for [`transform2_propagate`].
    Transform2Propagate,
}

/// Collection of all enabled Halia plugins.
pub struct Transform2Plugin;

impl Plugin for Transform2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (transform2_propagate,)
                .in_base_set(CoreSet::PostUpdate)
                .in_set(Transform2System::Transform2Propagate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

/// A 2D transform similar to Bevy's [`Transform`].
///
/// This component depends on Bevy's [`Transform`] and [`GlobalTransform`] and will only work if an
/// entity contains those components as well.
///
/// Unlike the regular [`Transform`], this transform separates depth into its own component and will
/// not adjust the z value of Bevy's [`Transform`]. To apply depth to an entity, use the [`Depth`]
/// component.
///
/// Ensures the final z scale is 1.0.
#[derive(Clone, Component, Copy, Debug)]
pub struct Transform2 {
    /// The 2D translational data (position) for this transform.
    pub translation: Vec2,
    /// The 2D rotational data for this transform.
    pub rotation: f32,
    /// The 2D scale data for this transform.
    pub scale: Vec2,
}

impl Transform2 {
    /// An identity [`Transform2`] with no translation, rotation, and a scale of 1 on all axes.
    pub const IDENTITY: Self = Transform2 {
        translation: Vec2::ZERO,
        rotation: 0.,
        scale: Vec2::ONE,
    };

    /// Creates a new [`Transform2`] at the position `(x, y)`.
    #[inline]
    #[must_use]
    pub const fn from_xy(x: f32, y: f32) -> Self {
        Self::from_translation(Vec2::new(x, y))
    }

    /// Creates a new [`Transform2`], with `translation`. Rotation will be 0 and scale 1 on all
    /// axes.
    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vec2) -> Self {
        Transform2 {
            translation,
            ..Self::IDENTITY
        }
    }

    /// Creates a new [`Transform2`], with `rotation`. Translation will be 0 and scale 1 on all
    /// axes.
    #[inline]
    #[must_use]
    pub const fn from_rotation(rotation: f32) -> Self {
        Transform2 {
            rotation,
            ..Self::IDENTITY
        }
    }

    /// Creates a new [`Transform2`], with `scale`. Translation will be 0 on all axes and rotation
    /// will be 0.
    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vec2) -> Self {
        Transform2 {
            scale,
            ..Self::IDENTITY
        }
    }

    /// Returns this [`Transform2`] with a new translation.
    #[inline]
    #[must_use]
    pub const fn with_translation(mut self, translation: Vec2) -> Self {
        self.translation = translation;
        self
    }

    /// Returns this [`Transform2`] with a new rotation.
    #[inline]
    #[must_use]
    pub const fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Returns this [`Transform2`] with a new scale.
    #[inline]
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

impl Default for Transform2 {
    fn default() -> Self {
        Transform2::IDENTITY
    }
}

/// A component intended to aid in depth sorting.
///
/// This component depends on Bevy's [`Transform`] and [`GlobalTransform`] and will only work if an
/// entity contains those components as well.
///
/// The Bevy [`Transform`]'s z translation is overridden to the value in this component.
#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub enum Depth {
    /// Use an exact depth value to apply to z.
    Exact(f32),
    /// Base the depth of this entity's depth on its parent. The value specified will be applied as
    /// an offset to the parent's calculated depth.
    Inherit(f32),
}

/// Propagates [`Transform2`] and [`Depth`] values to [`Transform`].
///
/// Set: [`Transform2System::Transform2Propagate`]
pub fn transform2_propagate(
    mut transform_query: Query<(&mut Transform, Option<&Transform2>, Option<&Depth>)>,
    root_query: Query<Entity, Without<Parent>>,
    children_query: Query<&Children>,
) {
    // TODO: only use changed transform2 and depths?
    for root_entity in root_query.iter() {
        update_transform2_recursive(root_entity, &children_query, &mut transform_query, 0.);
    }
}

fn update_transform2_recursive(
    entity: Entity,
    children_query: &Query<&Children>,
    transform_query: &mut Query<(&mut Transform, Option<&Transform2>, Option<&Depth>)>,
    mut cumulative_depth: f32,
) {
    if let Ok((mut transform, transform2, depth)) = transform_query.get_mut(entity) {
        if let Some(transform2) = transform2 {
            transform.translation.x = transform2.translation.x;
            transform.translation.y = transform2.translation.y;
            transform.scale = transform2.scale.extend(1.);
            transform.rotation = Quat::from_rotation_z(transform2.rotation);
        }
        if let Some(depth) = depth {
            transform.translation.z = match depth {
                Depth::Exact(depth_value) => *depth_value - cumulative_depth,
                Depth::Inherit(depth_value) => *depth_value,
            };
        }
        cumulative_depth += transform.translation.z;
    }
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            update_transform2_recursive(*child, children_query, transform_query, cumulative_depth);
        }
    }
}
