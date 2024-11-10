use bevy::prelude::*;
use bevy::window::{Window, WindowResolution};

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

fn window_resolution() -> WindowResolution {
    WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT).with_scale_factor_override(1.0)
}

pub fn window_plugins(w_title: &str) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: w_title.to_string(),
            resolution: window_resolution(),
            transparent: true,
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }
}
