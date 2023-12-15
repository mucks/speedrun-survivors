use bevy::prelude::*;

use crate::player::PlayerMovement;

pub fn sync_player_camera(
    player: Query<&Transform, With<PlayerMovement>>,
    mut camera: Query<(&mut Camera2d, &mut Transform), Without<PlayerMovement>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    let Ok((_, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    camera_transform.translation = player.translation;
}
