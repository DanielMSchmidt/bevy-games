use crate::common::{Bullet, GameSystemSet, Lifetime, Velocity};
use bevy::prelude::*;
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBulletEvent>();
        app.add_systems(Update, spawn_bullet.in_set(GameSystemSet::Spawning));
        app.add_systems(Update, lifetime.in_set(GameSystemSet::Lifetime));
    }
}

#[derive(Event)]
pub struct SpawnBulletEvent {
    pub position: Vec3,
    pub direction: Vec3,
}

fn spawn_bullet(
    mut commands: Commands,
    mut events: EventReader<SpawnBulletEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        commands.spawn((
            Bullet,
            Transform::from_translation(event.position),
            Velocity(event.direction),
            Lifetime(2.0), // Bullets last for 2 seconds
            Mesh3d(meshes.add(Sphere::new(0.1))),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
            Visibility::default(),
        ));
    }
}

fn lifetime(mut commands: Commands, mut query: Query<(Entity, &mut Lifetime)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (entity, mut lifetime) in &mut query {
        lifetime.0 -= dt;
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
