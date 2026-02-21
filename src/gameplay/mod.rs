use crate::common::{AngularVelocity, Asteroid, Bullet, GameSystemSet, Lifetime, Size, Velocity};
use bevy::prelude::*;
use rand::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBulletEvent>();
        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            3.0,
            TimerMode::Repeating,
        )));

        app.add_systems(Update, spawn_bullet.in_set(GameSystemSet::Spawning));
        app.add_systems(Update, spawn_asteroid.in_set(GameSystemSet::Spawning));
        app.add_systems(Update, lifetime.in_set(GameSystemSet::Lifetime));
    }
}

#[derive(Event)]
pub struct SpawnBulletEvent {
    pub position: Vec3,
    pub direction: Vec3,
}

#[derive(Resource)]
struct AsteroidSpawnTimer(Timer);

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

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
        let spawn_pos = Vec3::new(angle.cos(), 0.0, angle.sin()) * 20.0;
        let velocity = -spawn_pos.normalize() * 2.0;
        let sizes = [1.0_f32, 2.0, 4.0];
        let size = sizes[rng.gen_range(0..sizes.len())];
        commands.spawn((
            Asteroid,
            Size(size),
            Velocity(velocity),
            Transform::from_translation(spawn_pos),
            AngularVelocity(rng.gen_range(-1.0..1.0)),
            Mesh3d(meshes.add(Sphere::new(size))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
            Visibility::default(),
        ));
    }
}
