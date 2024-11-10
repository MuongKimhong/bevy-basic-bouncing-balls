use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;

const FPS_TEXT_SIZE: f32 = 20.0;

fn fps_text(asset_server: &Res<AssetServer>) -> TextSection {
    TextSection::new(
        " FPS: ",
        TextStyle {
            font: asset_server.load("Menlo-Regular.ttf"),
            font_size: FPS_TEXT_SIZE,
            ..default()
        },
    )
}

fn fps_count_text(asset_server: &Res<AssetServer>) -> TextSection {
    TextSection::new(
        "0",
        TextStyle {
            font: asset_server.load("Menlo-Regular.ttf"),
            font_size: FPS_TEXT_SIZE,
            ..default()
        },
    )
}

pub fn fps_text_bundle(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_sections([fps_text(asset_server), fps_count_text(asset_server)])
}

pub fn update_fps_count_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
