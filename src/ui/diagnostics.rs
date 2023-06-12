use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::prelude::*;

#[derive(Component)]
struct FpsText;

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default()
        }),
        FpsText,
    ));
}

fn update_fps_system(diagnostics: Res<Diagnostics>, mut text: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut text {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}

pub struct DiagnosticsMenuPlugin;

impl Plugin for DiagnosticsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup)
            .add_system(update_fps_system);
    }
}
