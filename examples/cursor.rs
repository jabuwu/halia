use bevy::prelude::*;
use halia::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HaliaPlugins)
        .add_systems((setup,).in_schedule(CoreSchedule::Startup))
        .add_systems((cursor_position,).in_schedule(CoreSchedule::FixedUpdate))
        .run();
}

#[derive(Component)]

pub struct FollowCursor;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(20.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::default(),
        FollowCursor,
    ));
}

fn cursor_position(
    mut follow_cursor_query: Query<&mut Transform2, With<FollowCursor>>,
    cursor: Res<Cursor>,
) {
    dbg!(cursor.window_position);
    for mut follow_cursor_transform in follow_cursor_query.iter_mut() {
        follow_cursor_transform.translation = cursor.world_position;
    }
}
