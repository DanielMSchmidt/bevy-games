use bevy::prelude::*;

/// Marker component for the player's ship entity.
#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct AngularVelocity(pub f32);

#[derive(Component)]
pub struct Lifetime(pub f32);

#[derive(Component)]
pub struct Cooldown(pub f32);

#[derive(Component)]
pub struct Size(pub f32);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    Lifetime,
    Input,
    Spawning,
    Physics,
}
