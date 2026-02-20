use bevy::prelude::*;

use crate::common::Ship;

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
        Transform::from_xyz(0.0, 15.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    // Ship — cone primitive as a placeholder, tip pointing up (+Y)
    commands.spawn((
        Ship,
        Mesh3d(meshes.add(Cone::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.9, 0.4))),
        Transform::default(),
    ));
}
