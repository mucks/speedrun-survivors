use bevy::prelude::*;

use crate::player::PlayerMovement;

const CAMERA_SPEED: f32 = 5.0;

pub fn sync_player_camera(
    player: Query<&Transform, With<PlayerMovement>>,
    mut camera: Query<(&mut Camera2d, &mut Transform), Without<PlayerMovement>>,
    time: Res<Time>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    let Ok((_, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    let target_position = Vec3::new(
        player.translation.x,
        player.translation.y,
        player.translation.z,
    );
    camera_transform.translation = camera_transform
        .translation
        .lerp(target_position, CAMERA_SPEED * time.delta_seconds());
}
