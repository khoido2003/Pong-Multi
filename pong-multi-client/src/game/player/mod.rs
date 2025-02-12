use bevy::app::*;
use system::spawn_player;

pub mod component;
pub mod system;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
