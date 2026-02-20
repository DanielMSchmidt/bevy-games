use bevy::prelude::*;

use crate::{common::GameSystemSet, gameplay, input, physics, rendering, ui};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            rendering::RenderingPlugin,
            input::InputPlugin,
            physics::PhysicsPlugin,
            gameplay::GameplayPlugin,
            ui::UiPlugin,
        ))
        .configure_sets(
            Update,
            (
                GameSystemSet::Lifetime,
                GameSystemSet::Input,
                GameSystemSet::Spawning,
                GameSystemSet::Physics,
            )
                .chain(),
        )
        .run();
}
