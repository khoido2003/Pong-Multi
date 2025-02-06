use bevy::{prelude::*, text::FontSmoothing};
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
            DefaultPlugins,
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
