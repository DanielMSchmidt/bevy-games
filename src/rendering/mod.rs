use bevy::prelude::*;

use crate::common::{AngularVelocity, Cooldown, Ship, Velocity};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene);
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera — pulled back and slightly above, looking at the origin
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 30.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    // Directional light angled from above-right
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let ship = commands
        .spawn((
            Ship,
            Transform::default(),
            Visibility::default(),
            Velocity(Vec3::ZERO),
            AngularVelocity(0.0),
            Cooldown(0.0),
        ))
        .id();
    commands
        .spawn((
            Mesh3d(meshes.add(Cone::new(0.5, 2.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.2, 0.9, 0.4))),
            Transform::default().with_rotation(Quat::from_rotation_x(f32::to_radians(-90.0))),
        ))
        .set_parent(ship);
}
