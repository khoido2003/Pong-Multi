use bevy::{
    prelude::*,
    text::FontSmoothing,
    window::{PresentMode, WindowMode},
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use game::world::WorldPlugin;

pub mod game;
pub mod user_interface;

struct OverlayColor;
impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

fn main() {
    let mut app = App::new();

    app
        // Defaul plugins
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    //mode: WindowMode::Fullscreen(MonitorSelection::Primary),
                    mode: WindowMode::Windowed,
                    title: "Pong the Game".into(),
                    name: Some("bevy.app".into()),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    enabled_buttons: bevy::window::EnabledButtons {
                        ..Default::default()
                    },
                    visible: true,
                    ..default()
                }),

                ..Default::default()
            }),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 42.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },

                    text_color: OverlayColor::GREEN,
                    enabled: true,
                },
            },
        ))
        // Game plugins
        .add_plugins(WorldPlugin)
        .run();
}
