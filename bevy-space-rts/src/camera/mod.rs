use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (camera_pan, camera_zoom, camera_rotate));
    }
}

/// Marker + state for the RTS camera.
#[derive(Component)]
pub struct RtsCamera {
    /// World position the camera orbits around (on the gameplay plane).
    pub focus: Vec3,
    /// Distance from focus point.
    pub distance: f32,
    /// Yaw angle in radians.
    pub yaw: f32,
    /// Pitch angle in radians (measured from vertical).
    pub pitch: f32,
}

impl Default for RtsCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            distance: 400.0,
            yaw: 0.0,
            // ~60 degrees from vertical gives a nice top-down-ish angle
            pitch: std::f32::consts::FRAC_PI_3,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let cam = RtsCamera::default();
    let transform = compute_camera_transform(&cam);

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scale: 1.0,
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        transform,
        cam,
    ));
}

fn compute_camera_transform(cam: &RtsCamera) -> Transform {
    let offset = Vec3::new(
        cam.distance * cam.pitch.sin() * cam.yaw.sin(),
        cam.distance * cam.pitch.cos(),
        cam.distance * cam.pitch.sin() * cam.yaw.cos(),
    );
    Transform::from_translation(cam.focus + offset).looking_at(cam.focus, Vec3::Y)
}

fn camera_pan(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut RtsCamera, &mut Transform)>,
) {
    let Ok((mut cam, mut transform)) = query.single_mut() else {
        return;
    };

    let speed = 200.0 * time.delta_secs();
    let mut delta = Vec3::ZERO;

    // Compute forward/right on the XZ plane based on yaw
    let forward = Vec3::new(-cam.yaw.sin(), 0.0, -cam.yaw.cos());
    let right = Vec3::new(cam.yaw.cos(), 0.0, -cam.yaw.sin());

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        delta += forward;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        delta -= forward;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        delta -= right;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        delta += right;
    }

    if delta != Vec3::ZERO {
        cam.focus += delta.normalize() * speed;
        *transform = compute_camera_transform(&cam);
    }
}

fn camera_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    mut query: Query<(&mut Projection, &RtsCamera)>,
) {
    let Ok((mut projection, _cam)) = query.single_mut() else {
        return;
    };

    for event in scroll_events.read() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale -= event.y * 0.5;
            ortho.scale = ortho.scale.clamp(0.3, 5.0);
        }
    }
}

fn camera_rotate(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut RtsCamera, &mut Transform)>,
) {
    let Ok((mut cam, mut transform)) = query.single_mut() else {
        return;
    };

    let speed = 2.0 * time.delta_secs();

    if keys.pressed(KeyCode::KeyQ) {
        cam.yaw -= speed;
    }
    if keys.pressed(KeyCode::KeyE) {
        cam.yaw += speed;
    }

    if keys.pressed(KeyCode::KeyQ) || keys.pressed(KeyCode::KeyE) {
        *transform = compute_camera_transform(&cam);
    }
}
