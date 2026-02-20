use bevy::prelude::*;

/// Marker component for the player's ship entity.
#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct AngularVelocity(pub f32);
