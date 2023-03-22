use bevy::{
    prelude::*,
    input::mouse::{MouseWheel, MouseScrollUnit},
};

#[derive(Component)]
pub struct MainCamera {
    pub center: Vec3,
    pub zoom: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            center: Vec3::default(),
            zoom: 10.0,
            yaw: 0.0,
            pitch: 45.0,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(control_camera)
            .add_system(handle_camera);
    }
}


fn handle_camera(mut query: Query<(&mut Transform, &MainCamera)>) {
    for (mut transform, camera) in query.iter_mut() {
        transform.translation = Vec3::new(
            camera.zoom * camera.yaw.to_radians().sin(),
            camera.zoom * camera.pitch.to_radians().sin(), 
            camera.zoom * camera.yaw.to_radians().cos(),
        );
        transform.translation += camera.center;
        transform.look_at(camera.center, Vec3::Y);
    }
}

fn control_camera(
    mut q: Query<&mut MainCamera>,
    mut scroll_evr: EventReader<MouseWheel>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let yaw_speed = 30.0;
    let zoom_speed = 1.0;
    let pitch_speed = 25.0;
    let move_speed = 5.0;
    let mut camera = q.single_mut();

    // Yaw / pitch
    if keys.pressed(KeyCode::Right) {
        camera.yaw += yaw_speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Left) {
        camera.yaw -= yaw_speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Up) {
        camera.pitch += pitch_speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Down) {
        camera.pitch -= pitch_speed * time.delta_seconds();
    }

    // Zoom in/out
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line | MouseScrollUnit::Pixel => {
                camera.zoom -= zoom_speed * ev.y;
            },
        }
    }

    // Move camera around
    if keys.pressed(KeyCode::W) {
        camera.center.z -= move_speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) {
        camera.center.z += move_speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::A) {
        camera.center.x -= move_speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D) {
        camera.center.x += move_speed * time.delta_seconds();
    }

    // Handle camera bounds
    camera.pitch = camera.pitch.clamp(0.0, 75.0);
    camera.zoom = camera.zoom.clamp(1.0, 20.0);
}
