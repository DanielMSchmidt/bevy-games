use bevy::prelude::*;

use crate::assets::AssetsPlugin;
use crate::camera::CameraPlugin;
use crate::common::CommonPlugin;
use crate::rendering::RenderingPlugin;
use crate::selection::SelectionPlugin;
use crate::ui::UiPlugin;

pub struct SpaceRtsPlugin;

impl Plugin for SpaceRtsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space RTS".into(),
                    ..default()
                }),
                ..default()
            }),
            CommonPlugin,
            RenderingPlugin,
            CameraPlugin,
            AssetsPlugin,
            SelectionPlugin,
            UiPlugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
}
