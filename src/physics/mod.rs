use crate::common::{AngularVelocity, Velocity};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement);
    }
}

pub fn apply_movement(
    mut q: Query<(&mut Transform, &Velocity, &AngularVelocity)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, velocity, angular_velocity) in &mut q {
        // Move the entity according to its velocity
        transform.translation += velocity.0 * dt;

        // Rotate the entity around the Y axis according to its angular velocity
        transform.rotate_y(angular_velocity.0 * dt);
    }
}
