use bevy::prelude::*;

use crate::common::{AngularVelocity, Cooldown, Ship, Velocity};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene);
    }
}

fn spawn_scene(mut commands: Commands) {
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
}
