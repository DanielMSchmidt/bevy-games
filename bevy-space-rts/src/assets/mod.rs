use bevy::prelude::*;

use crate::common::{Asteroid, Selectable, Ship, Station};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scene);
    }
}

fn spawn_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- Gameplay plane (subtle grid reference) ---
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.02, 0.02, 0.05, 0.3),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.01, 0.0),
    ));

    // --- Ships ---
    let ship_positions = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(30.0, 0.0, 10.0),
        Vec3::new(-20.0, 0.0, 20.0),
        Vec3::new(15.0, 0.0, -25.0),
        Vec3::new(-35.0, 0.0, -10.0),
    ];

    for pos in &ship_positions {
        commands.spawn((
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("models/craft_speederA.glb")),
            ),
            Transform::from_translation(*pos).with_scale(Vec3::splat(40.0)),
            Ship,
            Selectable,
        ));
    }

    // --- Asteroids ---
    let asteroid_configs: Vec<(Vec3, f32, &str)> = vec![
        (Vec3::new(150.0, 0.0, 100.0), 40.0, "models/rock_largeA.glb"),
        (Vec3::new(180.0, 0.0, 80.0), 25.0, "models/rocks_smallA.glb"),
        (Vec3::new(-120.0, 0.0, 140.0), 50.0, "models/rock_largeA.glb"),
        (Vec3::new(200.0, 0.0, 120.0), 30.0, "models/rocks_smallA.glb"),
        (Vec3::new(-100.0, 0.0, -150.0), 40.0, "models/rock_largeA.glb"),
        (Vec3::new(80.0, 0.0, -180.0), 20.0, "models/rocks_smallA.glb"),
    ];

    for (pos, scale, model) in &asteroid_configs {
        commands.spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(*model))),
            Transform::from_translation(*pos).with_scale(Vec3::splat(*scale)),
            Asteroid,
        ));
    }

    // --- HQ Station (near an asteroid cluster) ---
    commands.spawn((
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("models/hangar_roundA.glb")),
        ),
        Transform::from_translation(Vec3::new(160.0, 0.0, 110.0))
            .with_scale(Vec3::splat(50.0)),
        Station,
    ));

    // --- Starfield background (simple scattered particles via meshes) ---
    spawn_starfield(&mut commands, &mut meshes, &mut materials);
}

fn spawn_starfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let star_mesh = meshes.add(Sphere::new(0.05));
    let star_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(10.0, 10.0, 10.0, 1.0),
        unlit: true,
        ..default()
    });

    // Pseudo-random starfield using deterministic positions
    let seeds: [(f32, f32); 40] = [
        (0.1, 0.2), (0.3, 0.7), (0.5, 0.1), (0.8, 0.9), (0.2, 0.5),
        (0.9, 0.3), (0.4, 0.8), (0.6, 0.4), (0.7, 0.6), (0.15, 0.85),
        (0.35, 0.15), (0.55, 0.65), (0.75, 0.35), (0.95, 0.55), (0.25, 0.95),
        (0.45, 0.25), (0.65, 0.75), (0.85, 0.45), (0.05, 0.05), (0.12, 0.62),
        (0.32, 0.42), (0.52, 0.82), (0.72, 0.22), (0.92, 0.72), (0.22, 0.32),
        (0.42, 0.52), (0.62, 0.92), (0.82, 0.12), (0.02, 0.42), (0.18, 0.78),
        (0.38, 0.58), (0.58, 0.38), (0.78, 0.18), (0.98, 0.98), (0.28, 0.08),
        (0.48, 0.48), (0.68, 0.68), (0.88, 0.88), (0.08, 0.28), (0.33, 0.93),
    ];

    for (sx, sz) in seeds {
        let x = (sx - 0.5) * 200.0;
        let z = (sz - 0.5) * 200.0;
        let y = -5.0 + sx * sz * 2.0; // Slight depth variation

        commands.spawn((
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(star_material.clone()),
            Transform::from_xyz(x, y, z),
        ));
    }
}
