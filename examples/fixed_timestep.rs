use bevy::prelude::*;
use halia::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HaliaPlugins)
        .add_systems((setup,).in_schedule(CoreSchedule::Startup))
        .add_systems((print_fixed_key_inputs,).in_schedule(CoreSchedule::FixedUpdate))
        .add_systems((send_event,))
        .add_systems((receive_event,).in_schedule(CoreSchedule::FixedUpdate))
        .add_fixed_event::<MyEvent>()
        .insert_resource(FixedTime::new_from_secs(1. / 5.))
        .run();
}

#[derive(Default)]
pub struct MyEvent;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn print_fixed_key_inputs(fixed_keys: Res<FixedInput<KeyCode>>) {
    if fixed_keys.just_pressed(KeyCode::Space) {
        println!("[fixed] space pressed!");
    }
}

fn send_event(mut events: EventWriter<MyEvent>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::E) {
        events.send_default();
    }
}

fn receive_event(mut events: EventReader<MyEvent>) {
    for _ in events.iter() {
        println!("[fixed] received MyEvent!");
    }
}
