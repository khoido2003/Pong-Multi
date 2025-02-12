use bevy::{prelude::*, window::PrimaryWindow};

use super::component::Player;

const PADDLE_SIZE: f32 = 32.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        let window_width = window.width();
        let window_height = window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/block_narrow.png"),
                ..default()
            },
            Transform::from_xyz(window_width / 2.0 - PADDLE_SIZE, 0.0, 0.0),
            Player {},
        ));
    }
}
