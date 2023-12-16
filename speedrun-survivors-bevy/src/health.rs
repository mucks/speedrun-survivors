use crate::player::Player;
use crate::state::{AppState, ForState};
use bevy::prelude::*;

const HEALTH_BAR_WIDTH: f32 = 100.0;
const HEALTH_BAR_HEIGHT: f32 = 20.0;
const HEALTH_BAR_OFFSET_Y: f32 = 50.0;

#[derive(Debug, Component)]
pub struct Health {
    pub active_health: f32,
    pub max_health: f32,
    pub regen: f32,
    pub health_bar: Option<Entity>,
}

#[derive(Component)]
pub struct HealthBar {
    pub offset: Vec2,
}

#[derive(Component)]
pub struct EmptyBar;

pub fn add_health_bar(commands: &mut Commands, translation: Vec3, z: f32) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        translation.x + 0.,
                        translation.y + HEALTH_BAR_OFFSET_Y,
                        z,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            ForState {
                states: vec![AppState::GameRunning],
            },
        ))
        .insert(HealthBar {
            offset: Vec2::new(0., HEALTH_BAR_OFFSET_Y),
        })
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -10.),
                    ..Default::default()
                },

                ..Default::default()
            });
        })
        .id()
}

pub fn update_health_bar(
    mut next_state: ResMut<NextState<AppState>>,
    mut healthbar_query: Query<
        (&HealthBar, &mut Sprite, &mut Transform, Entity),
        (With<HealthBar>, Without<Health>),
    >,
    mut health_query: Query<
        (&Health, Entity, &Transform, Option<&Player>),
        (With<Health>, Without<HealthBar>),
    >,
    mut commands: Commands,
) {
    for (health, mut entity, mut transform, player) in health_query.iter_mut() {
        let health_percentage = health.active_health / health.max_health;

        if health_percentage <= 0.0 {
            if player.is_some() {
                next_state.set(AppState::GameOver);
            }
            commands.entity(entity).despawn_recursive();
        }
        //TODO refactored that a bit; but I think we should move the healthbar into the UI.. no need to update this bar as a separate component
        if let Some(health_bar_entity) = health.health_bar {
            if let Ok((health_bar, mut bar_sprite, mut bar_tr, br_entity)) =
                healthbar_query.get_mut(health_bar_entity)
            {
                if health_percentage <= 0.0 {
                    commands.entity(br_entity).despawn_recursive();
                    return;
                }

                // Got a health bar
                bar_tr.translation = Vec3::new(
                    transform.translation.x + health_bar.offset.x,
                    transform.translation.y + health_bar.offset.y,
                    bar_tr.translation.z,
                );

                let new_width = health_percentage * HEALTH_BAR_WIDTH; // Change 200.0 to the width of your health bar
                if let Some(ref mut size) = bar_sprite.custom_size {
                    size.x = new_width.clamp(0.0, HEALTH_BAR_WIDTH); // Clamp the width to ensure it doesn't go beyond the bar's boundaries
                }
            };
        }
    }
}
