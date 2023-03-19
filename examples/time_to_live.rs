use std::{f32::consts::TAU, time::Duration};

use bevy::prelude::*;
use halia::prelude::*;
use rand::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum ExampleSystem {
    Setup,
    Spawn,
    Update,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HaliaPlugins)
        .add_systems((setup,).in_schedule(CoreSchedule::Startup))
        .add_systems(
            (
                spawn.in_set(ExampleSystem::Spawn),
                update
                    .in_set(ExampleSystem::Update)
                    .after(ExampleSystem::Spawn),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .run();
}

#[derive(Component)]
pub struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::default(),
        Velocity(Vec2::from_angle(thread_rng().gen_range(0.0..TAU)) * 300.),
        TimeToLive::new(Duration::from_secs_f32(1.)),
    ));
}

fn update(mut query: Query<(&mut Transform2, &Velocity)>, time: Res<FixedTime>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.period.as_secs_f32();
    }
}
