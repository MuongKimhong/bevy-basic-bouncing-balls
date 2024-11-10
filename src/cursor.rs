use crate::ball;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

fn cursor_position(windows: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    windows.single().cursor_position().unwrap()
}

pub fn screen_coord_to_world_coord(
    camera: &Camera,
    windows: &Query<&Window, With<PrimaryWindow>>,
    camera_transform: &GlobalTransform,
) -> (f32, f32) {
    let world_pos = camera
        .viewport_to_world(camera_transform, cursor_position(windows))
        .unwrap();

    (world_pos.origin.x, world_pos.origin.y)
}

pub fn mouse_button_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut total_balls: ResMut<ball::TotalBallCount>,
    windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    for ev in mousebtn_evr.read() {
        if let Some((camera, camera_transform)) = q_camera.iter().next() {
            match ev.state {
                ButtonState::Pressed => {
                    let (x, y) = screen_coord_to_world_coord(camera, &windows, camera_transform);
                    commands.spawn((
                        ball::create_ball(&mut meshes, &mut materials, &mut total_balls, x, y),
                        RigidBody::Dynamic,
                        Collider::ball(ball::BALL_RADIUS),
                        Restitution::coefficient(0.8), // Bounciness
                    ));
                }
                ButtonState::Released => {
                    // println!("Mouse button release: {:?}", ev.button);
                }
            }
        }
    }
}
