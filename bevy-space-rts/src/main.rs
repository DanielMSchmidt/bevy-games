use bevy::prelude::*;

mod app;
mod assets;
mod camera;
mod common;
mod rendering;
mod selection;
mod ui;

fn main() {
    App::new()
        .add_plugins(app::SpaceRtsPlugin)
        .run();
}
