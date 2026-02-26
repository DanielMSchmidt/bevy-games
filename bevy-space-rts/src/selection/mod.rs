use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::*;

use crate::common::{Selectable, Selected, SelectionRing};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .add_systems(Update, (handle_click_selection, update_selection_rings))
            .add_systems(PostStartup, attach_selection_rings);
    }
}

/// After ships are spawned, attach a selection ring child to each selectable entity.
fn attach_selection_rings(
    mut commands: Commands,
    query: Query<Entity, With<Selectable>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ring_mesh = meshes.add(Annulus::new(0.8, 1.0));
    let ring_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 0.3, 0.6),
        emissive: LinearRgba::new(0.0, 4.0, 1.0, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    for entity in &query {
        let ring = commands
            .spawn((
                Mesh3d(ring_mesh.clone()),
                MeshMaterial3d(ring_material.clone()),
                Transform::from_xyz(0.0, 0.1, 0.0)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                Visibility::Hidden,
                SelectionRing,
            ))
            .id();

        commands.entity(entity).add_child(ring);
    }
}

/// Detect clicks on ships using raycasting against the gameplay plane,
/// then pick the nearest selectable entity.
fn handle_click_selection(
    mouse: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
    selected_q: Query<Entity, With<Selected>>,
    selectable_q: Query<(Entity, &GlobalTransform), With<Selectable>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Cast ray from cursor through camera
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return;
    };

    // Intersect with Y=0 plane
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let world_pos = ray.get_point(distance);

    // Find nearest selectable entity within pick radius
    let pick_radius = 2.5;
    let mut best: Option<(Entity, f32)> = None;

    for (entity, global_transform) in &selectable_q {
        let entity_pos = global_transform.translation();
        let dist = (Vec2::new(entity_pos.x, entity_pos.z)
            - Vec2::new(world_pos.x, world_pos.z))
        .length();

        if dist < pick_radius && (best.is_none() || dist < best.unwrap().1) {
            best = Some((entity, dist));
        }
    }

    // Deselect all first
    for entity in &selected_q {
        commands.entity(entity).remove::<Selected>();
    }

    // Select the picked entity
    if let Some((entity, _)) = best {
        commands.entity(entity).insert(Selected);
    }
}

/// Show/hide selection rings based on the `Selected` component.
fn update_selection_rings(
    selected_q: Query<Entity, With<Selected>>,
    selectable_q: Query<(Entity, &Children), With<Selectable>>,
    mut ring_q: Query<&mut Visibility, With<SelectionRing>>,
    time: Res<Time>,
    mut ring_transforms: Query<&mut Transform, With<SelectionRing>>,
) {
    // Build set of selected entities
    let selected: Vec<Entity> = selected_q.iter().collect();

    for (entity, children) in &selectable_q {
        let is_selected = selected.contains(&entity);

        for child in children.iter() {
            if let Ok(mut visibility) = ring_q.get_mut(child) {
                *visibility = if is_selected {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }

            // Gentle rotation on the ring
            if is_selected {
                if let Ok(mut ring_transform) = ring_transforms.get_mut(child) {
                    ring_transform.rotate_local_z(time.delta_secs() * 1.5);
                }
            }
        }
    }
}
