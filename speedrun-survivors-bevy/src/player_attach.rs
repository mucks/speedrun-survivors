use bevy::prelude::*;

use crate::{animation::Animator, player::PlayerMovement};

#[derive(Component)]
pub struct PlayerAttach {
    pub offset: Vec2,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl PlayerAttach {
    pub fn new(offset: Vec2) -> Self {
        Self {
            offset,
            flip_x: false,
            flip_y: false,
        }
    }
}

pub fn attach_objects(
    player_query: Query<(&PlayerMovement, &mut Transform), Without<PlayerAttach>>,
    mut objects_query: Query<(&PlayerAttach, &mut Transform), Without<PlayerMovement>>,
) {
    if let Ok((_movement_data, player_transform)) = player_query.get_single() {
        for (attach, mut transform) in objects_query.iter_mut() {
            let mut offset_x = attach.offset.x;
            let mut offset_y = attach.offset.y;

            if attach.flip_x {
                offset_x *= -1.;
            }

            if attach.flip_y {
                offset_y *= -1.;
            }

            transform.translation =
                player_transform.translation + Vec3::new(offset_x, offset_y, 0.);
        }
    }
}
