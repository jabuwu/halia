use bevy::prelude::*;
use halia::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HaliaPlugins)
        .add_systems((setup,).in_schedule(CoreSchedule::Startup))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
