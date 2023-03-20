use bevy::prelude::*;
use halia::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HaliaPlugins)
        .add_systems((setup,).in_schedule(CoreSchedule::Startup))
        .run();
}

fn setup(mut force_ratio: ResMut<ForceRatio>, mut commands: Commands) {
    force_ratio.enable(1920., 1080.);

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(400.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
