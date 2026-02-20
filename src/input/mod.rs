use crate::common::{AngularVelocity, Ship, Velocity};
use crate::physics::apply_movement;
use bevy::prelude::*;
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_movement_input.before(apply_movement));
    }
}

const THRUST: f32 = 10.0;
const BRAKE_FORCE: f32 = 3.0;
const ROTATION_SPEED: f32 = std::f32::consts::PI; // radians per second

fn read_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity, &mut AngularVelocity), With<Ship>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let up = keys.pressed(KeyCode::ArrowUp);
    let down = keys.pressed(KeyCode::ArrowDown);
    let left = keys.pressed(KeyCode::ArrowLeft);
    let right = keys.pressed(KeyCode::ArrowRight);

    for (transform, mut velocity, mut angular_velocity) in &mut query {
        if up {
            velocity.0 += transform.forward() * (THRUST * dt);
        }
        if down {
            // Breaking is slower than accelerating, so we apply half the thrust in reverse
            velocity.0 *= 1.0 - (BRAKE_FORCE * dt);
        }

        if left {
            angular_velocity.0 += ROTATION_SPEED * dt;
        }
        if right {
            angular_velocity.0 -= ROTATION_SPEED * dt;
        }
    }
}
