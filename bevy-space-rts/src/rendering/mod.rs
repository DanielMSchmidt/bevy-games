use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lighting);
    }
}

fn setup_lighting(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.05)));
    // Directional "sun" light
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));

    // Subtle ambient light so shadow areas aren't pitch black
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.6, 0.65, 0.8),
        brightness: 200.0,
        ..default()
    });
}
