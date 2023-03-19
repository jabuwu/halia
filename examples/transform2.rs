use bevy::prelude::*;
use halia::prelude::*;
use lerp::Lerp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HaliaPlugins)
        .add_systems((setup,).in_schedule(CoreSchedule::Startup))
        .add_systems((player_update, y_order))
        .run();
}

#[derive(Clone, Copy)]
pub enum DepthLayer {
    Inherit(f32),
    Below,
    YOrder(f32),
    Above,
}

impl From<DepthLayer> for Depth {
    fn from(value: DepthLayer) -> Self {
        match value {
            DepthLayer::Inherit(x) => Depth::Inherit(x),
            DepthLayer::Below => Depth::Exact(0.1),
            DepthLayer::YOrder(x) => Depth::Exact(0.2_f32.lerp(0.8, x)),
            DepthLayer::Above => Depth::Exact(1.),
        }
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
pub struct YOrder;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::default(),
        Depth::from(DepthLayer::YOrder(0.)),
        YOrder,
        Player { speed: 300. },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(50.)),
                color: Color::BLUE,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::from_xy(100., -75.),
        Depth::from(DepthLayer::YOrder(0.)),
        YOrder,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                color: Color::GREEN,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::from_xy(-200., -75.),
        Depth::from(DepthLayer::Below),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                color: Color::PURPLE,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::from_xy(-200., 75.),
        Depth::from(DepthLayer::Above),
    ));
}

fn player_update(
    mut player_query: Query<(&mut Transform2, &Player)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut movement = Vec2::ZERO;

    if keys.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if keys.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if keys.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if keys.pressed(KeyCode::D) {
        movement.x += 1.;
    }

    for (mut player_transform, player) in player_query.iter_mut() {
        player_transform.translation +=
            movement.normalize_or_zero() * time.delta_seconds() * player.speed;
    }
}

fn y_order(mut y_order_query: Query<(&mut Depth, &Transform2), With<YOrder>>) {
    for (mut y_order_depth, y_order_transform) in y_order_query.iter_mut() {
        *y_order_depth = DepthLayer::YOrder(y_order_transform.translation.y / -10000. + 0.5).into();
    }
}
