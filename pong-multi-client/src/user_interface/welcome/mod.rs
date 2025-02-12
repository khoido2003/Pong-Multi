use bevy::prelude::*;
use system::{button_system, exit_button_system, generate_random_name, spawn_welcome_screen};

pub mod components;
pub mod styles;
pub mod system;

pub struct WelcomePlugin;

#[derive(Resource, Default)]
pub struct PlayerName(String);

impl Plugin for WelcomePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerName>()
            .add_systems(
                Startup,
                (generate_random_name, spawn_welcome_screen).chain(),
            )
            .add_systems(Update, (button_system, exit_button_system));
    }
}
