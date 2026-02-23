use std::{collections::HashSet, f32::consts::FRAC_PI_4};

use crate::common::{
    AngularVelocity, Asteroid, Bullet, GameSystemSet, Lifetime, Ship, Size, Velocity,
};
use bevy::prelude::*;
use rand::prelude::*;

const BULLET_SIZE: f32 = 0.2;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBulletEvent>();
        app.add_event::<BulletAsteroidCollisionEvent>();
        app.add_event::<AsteroidShipCollisionEvent>();

        app.insert_resource(AsteroidSpawnTimer(Timer::from_seconds(
            3.0,
            TimerMode::Repeating,
        )));

        app.add_systems(Update, spawn_bullet.in_set(GameSystemSet::Spawning));
        app.add_systems(Update, spawn_asteroid.in_set(GameSystemSet::Spawning));
        app.add_systems(Update, lifetime.in_set(GameSystemSet::Lifetime));
        app.add_systems(
            Update,
            detect_collisions.in_set(GameSystemSet::CollisionDetection),
        );
        app.add_systems(
            Update,
            handle_bullet_asteroid_collision.in_set(GameSystemSet::CollisionResponse),
        );
        app.add_systems(
            Update,
            handle_ship_asteroid_collision.in_set(GameSystemSet::CollisionResponse),
        );
    }
}

#[derive(Event)]
pub struct SpawnBulletEvent {
    pub position: Vec3,
    pub direction: Vec3,
}

#[derive(Event)]
pub struct BulletAsteroidCollisionEvent {
    pub bullet: Entity,
    pub asteroid: Entity,
}

#[derive(Event)]
pub struct AsteroidShipCollisionEvent {}

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
            Mesh3d(meshes.add(Sphere::new(BULLET_SIZE))),
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

fn detect_collisions(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &Size), With<Asteroid>>,
    ship: Query<(Entity, &Transform), With<Ship>>,
    mut bullet_hit: EventWriter<BulletAsteroidCollisionEvent>,
    mut ship_hit: EventWriter<AsteroidShipCollisionEvent>,
) {
    for (bullet_entity, bullet_transform) in &bullets {
        for (asteroid_entity, asteroid_transform, asteroid_size) in &asteroids {
            let distance = bullet_transform
                .translation
                .distance(asteroid_transform.translation);
            if distance < asteroid_size.0 + BULLET_SIZE {
                bullet_hit.send(BulletAsteroidCollisionEvent {
                    bullet: bullet_entity,
                    asteroid: asteroid_entity,
                });
            }
        }
    }

    for (_, asteroid_transform, asteroid_size) in &asteroids {
        for (_, ship_transform) in &ship {
            let distance = asteroid_transform
                .translation
                .distance(ship_transform.translation);
            // This is inaccurate since the ship is a cone
            if distance < asteroid_size.0 + 0.5 {
                ship_hit.send(AsteroidShipCollisionEvent {});
            }
        }
    }
}

fn handle_bullet_asteroid_collision(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<BulletAsteroidCollisionEvent>,
    asteroid_query: Query<(&Transform, &Size, &Velocity), With<Asteroid>>,
) {
    let mut despawned = HashSet::new();
    for event in events.read() {
        if despawned.contains(&event.bullet) || despawned.contains(&event.asteroid) {
            continue;
        }
        if let Ok((asteroid_transform, asteroid_size, asteroid_velocity)) =
            asteroid_query.get(event.asteroid)
        {
            // Despawn the bullet and the hit asteroid
            commands.entity(event.bullet).despawn();
            despawned.insert(event.bullet);

            commands.entity(event.asteroid).despawn();
            despawned.insert(event.asteroid);

            // If the asteroid is large enough, split it into two smaller ones
            if asteroid_size.0 > 1.0 {
                let new_size = asteroid_size.0 / 2.0;
                for i in 0..2 {
                    let foresign: f32 = if i == 0 { 1.0 } else { -1.0 };
                    let velocity =
                        Quat::from_rotation_y(foresign * FRAC_PI_4).mul_vec3(asteroid_velocity.0);

                    commands.spawn((
                        Asteroid,
                        Size(new_size),
                        Velocity(velocity),
                        Transform::from_translation(asteroid_transform.translation),
                        AngularVelocity(rand::thread_rng().gen_range(-1.0..1.0)),
                        Mesh3d(mesh.add(Sphere::new(new_size))),
                        MeshMaterial3d(material.add(Color::srgb(0.6, 0.6, 0.6))),
                        Visibility::default(),
                    ));
                }
            }
        }
    }
}

fn handle_ship_asteroid_collision(
    mut commands: Commands,
    ship_query: Query<Entity, With<Ship>>,
    mut events: EventReader<AsteroidShipCollisionEvent>,
) {
    for _ in events.read() {
        for ship_entity in &ship_query {
            // For simplicity, just despawn the ship on collision
            commands.entity(ship_entity).despawn_recursive();
        }
    }
}
