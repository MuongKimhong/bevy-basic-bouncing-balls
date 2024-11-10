pub mod ball;
pub mod cursor;
pub mod fps;
pub mod window;

use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn camera_2d_bundle() -> Camera2dBundle {
    Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    }
}

fn ground() -> (RigidBody, Collider, TransformBundle) {
    (
        RigidBody::Fixed,
        Collider::cuboid(window::WINDOW_WIDTH, 1.0),
        TransformBundle::from(Transform::from_xyz(0.0, -window::WINDOW_HEIGHT / 2.0, 0.0)),
    )
}

fn left_wall() -> (RigidBody, Collider, TransformBundle) {
    (
        RigidBody::Fixed,
        Collider::cuboid(1.0, window::WINDOW_HEIGHT),
        TransformBundle::from(Transform::from_xyz(-window::WINDOW_WIDTH / 2.0, 0.0, 0.0)),
    )
}

fn right_wall() -> (RigidBody, Collider, TransformBundle) {
    let wall_width = 1.0;
    (
        RigidBody::Fixed,
        Collider::cuboid(wall_width, window::WINDOW_HEIGHT),
        TransformBundle::from(Transform::from_xyz(
            window::WINDOW_WIDTH / 2.0 - wall_width,
            0.0,
            0.0,
        )),
    )
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ball::TotalBallCount(0));

    commands.spawn(ground());
    commands.spawn(left_wall());
    commands.spawn(right_wall());
    commands.spawn((camera_2d_bundle(), BloomSettings::default()));
    commands.spawn((fps::fps_text_bundle(&asset_server), fps::FpsText));
    commands.spawn((ball::ball_text_bundle(&asset_server), ball::BallCountText));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window::window_plugins("Bouncing Ball")))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, cursor::mouse_button_events)
        .add_systems(Update, fps::update_fps_count_system)
        .add_systems(Update, ball::update_ball_count_system)
        .run();
}
