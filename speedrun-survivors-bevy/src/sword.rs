use bevy::prelude::*;

use crate::{animation::Animator, enemy::Enemy};

#[derive(Component)]
pub struct SwordController {
    pub hitbox: f32,
    pub swing_time: f32,
}

pub fn sword_controls(
    mut sword_query: Query<(&mut SwordController, &mut Transform, &mut Animator)>,
    buttons: Res<Input<MouseButton>>,
) {
    for (mut sword_controller, mut transform, mut animator) in sword_query.iter_mut() {
        if sword_controller.swing_time > 0. {
            sword_controller.swing_time -= 0.1;
            animator.current_animation = "Swing".to_string();
        } else {
            animator.current_animation = "Idle".to_string();
        }

        if sword_controller.swing_time <= 0. {
            if buttons.pressed(MouseButton::Left) {
                sword_controller.swing_time = 1.;
            }
        }
    }
}

pub fn update_sword_hits(
    sword_query: Query<(&Transform, Entity), (With<SwordController>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform), Without<SwordController>>,
) {
    if let Some(sword) = sword_query.iter().next() {
        let s = Vec2::new(sword.0.translation.x, sword.0.translation.y);
        for (mut enemy, transform) in enemy_query.iter_mut() {
            if Vec2::distance(
                s,
                Vec2::new(transform.translation.x, transform.translation.y),
            ) <= 32.
            {
                enemy.health -= 1.;
            }
        }
    }
}
