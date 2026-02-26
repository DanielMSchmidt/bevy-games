use bevy::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Marker for entities that can be selected (ships, etc.)
#[derive(Component, Default)]
pub struct Selectable;

/// Marker for the currently selected entity.
#[derive(Component, Default)]
pub struct Selected;

/// Marker for ship entities.
#[derive(Component, Default)]
pub struct Ship;

/// Marker for asteroid entities.
#[derive(Component, Default)]
pub struct Asteroid;

/// Marker for the HQ station entity.
#[derive(Component, Default)]
pub struct Station;

/// Marker for the selection ring visual.
#[derive(Component)]
pub struct SelectionRing;
