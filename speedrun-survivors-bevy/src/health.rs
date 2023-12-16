use crate::state::{AppState, ForState};
use bevy::prelude::*;

const HEALTH_BAR_WIDTH: f32 = 100.0;
const HEALTH_BAR_HEIGHT: f32 = 20.0;
const HEALTH_BAR_OFFSET_Y: f32 = 50.0;

#[derive(Debug, Component)]
pub struct Health {
    pub active_health: f32,
    pub max_health: f32,
    pub is_player: bool,
}

#[derive(Component)]
pub struct HealthBar {
    pub entity: Entity,
    pub offset: Vec2,
}

#[derive(Component)]
pub struct EmptyBar;

pub fn add_health_bar(commands: &mut Commands, entity: Entity, translation: Vec3, z: f32) {
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
            entity,
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
        });
}

pub fn update_health_bar(
    mut next_state: ResMut<NextState<AppState>>,
    mut healthbar_query: Query<
        (&HealthBar, &mut Sprite, &mut Transform, Entity),
        (With<HealthBar>, Without<Health>),
    >,
    mut health_query: Query<(&Health, Entity, &Transform), (With<Health>, Without<HealthBar>)>,
    mut commands: Commands,
) {
    for (health_bar, mut sprite, mut health_bar_tr, health_bar_entity) in healthbar_query.iter_mut()
    {
        let Ok((health, entity_with_health, tr)) = health_query.get_mut(health_bar.entity) else {
            continue;
        };

        health_bar_tr.translation = Vec3::new(
            tr.translation.x + health_bar.offset.x,
            tr.translation.y + health_bar.offset.y,
            health_bar_tr.translation.z,
        );

        let health_percentage = health.active_health / health.max_health;

        if health_percentage <= 0.0 {
            if health.is_player {
                next_state.set(AppState::GameOver);
            }
            commands.entity(health_bar_entity).despawn_recursive();
            commands.entity(entity_with_health).despawn_recursive();
        }

        let new_width = health_percentage * HEALTH_BAR_WIDTH; // Change 200.0 to the width of your health bar
        if let Some(ref mut size) = sprite.custom_size {
            size.x = new_width.clamp(0.0, HEALTH_BAR_WIDTH); // Clamp the width to ensure it doesn't go beyond the bar's boundaries
        }
    }
}
