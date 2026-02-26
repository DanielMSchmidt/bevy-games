use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_help_overlay);
    }
}

fn spawn_help_overlay(mut commands: Commands) {
    commands.spawn((
        Text::new(
            "Controls:\n\
             WASD / Arrows - Pan camera\n\
             Mouse Wheel   - Zoom\n\
             Q / E         - Rotate camera\n\
             Left Click    - Select ship\n\
             \n\
             Space RTS - Asset Preview",
        ),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}
