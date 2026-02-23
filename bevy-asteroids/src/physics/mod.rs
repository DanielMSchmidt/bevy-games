use crate::common::{AngularVelocity, GameSystemSet, Velocity};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement.in_set(GameSystemSet::Physics));
        app.add_systems(Update, apply_rotation.in_set(GameSystemSet::Physics));
    }
}

fn apply_movement(mut q: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in &mut q {
        // Move the entity according to its velocity
        transform.translation += velocity.0 * dt;
    }
}

fn apply_rotation(mut q: Query<(&mut Transform, &AngularVelocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut transform, angular_velocity) in &mut q {
        // Rotate the entity around the Y axis according to its angular velocity
        transform.rotate_y(angular_velocity.0 * dt);
    }
}
