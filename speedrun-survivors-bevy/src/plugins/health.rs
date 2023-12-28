use crate::enemy::enemy_type::EnemyType;
use crate::enemy::EnemyEvent;
use crate::player::{Player, PlayerEvent};
use crate::state::{for_game_states, AppState};
use bevy::prelude::*;

const HEALTH_BAR_WIDTH: f32 = 100.0;
const HEALTH_BAR_HEIGHT: f32 = 10.0;
const HEALTH_BAR_OFFSET_Y: f32 = -60.0;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthUpdateEvent>();
        app.add_systems(
            Update,
            (on_health_change_event, regenerate_health, update_health_bar)
                .run_if(in_state(AppState::GameRunning)),
        );
    }
}

#[derive(Debug, Component)]
pub struct Health {
    pub current: f32,
    maximum: f32,
    regen: f32,
    pub health_bar: Option<Entity>,
}

impl Health {
    pub fn new(current: f32, maximum: f32, regen: f32, health_bar: Option<Entity>) -> Self {
        Health {
            current,
            maximum,
            regen,
            health_bar,
        }
    }

    pub fn set_cap(&mut self, cap: f32) {
        self.maximum = cap;
    }

    pub fn set_regen(&mut self, regen: f32) {
        self.regen = regen;
    }

    pub fn set_health_half(&mut self) {
        self.current = self.maximum / 2.0;
    }

    pub fn apply_regen(&mut self, delta: f32) {
        if self.regen <= 0.0 {
            return;
        }

        // Increment health
        self.current += self.regen * delta;

        // Cap to max health
        if self.current > self.maximum {
            self.current = self.maximum;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetType {
    Player,
    Enemy(EnemyType),
}

#[derive(Debug, Event)]
pub struct HealthUpdateEvent {
    /// The entity who's health changed
    pub entity: Entity,
    /// The amount the health changed by. Negative for damage, positive for healing
    pub health_change: f32,
    /// What type of
    pub target_type: TargetType,
}

fn on_health_change_event(
    mut rx_health: EventReader<HealthUpdateEvent>,
    mut health_query: Query<&mut Health>,
    mut tx_player: EventWriter<PlayerEvent>,
    mut tx_enemy: EventWriter<EnemyEvent>,
) {
    for ev in rx_health.iter() {
        let Ok(mut health) = health_query.get_mut(ev.entity) else {
            continue;
        };
        health.current += ev.health_change;
        if health.current <= 0. {
            match ev.target_type {
                TargetType::Player => {
                    tx_player.send(PlayerEvent::Died);
                }
                TargetType::Enemy(kind) => {
                    tx_enemy.send(EnemyEvent::Died(ev.entity, kind));
                    tx_player.send(PlayerEvent::EnemyDefeated);
                }
            }
        }
    }
}

/// Regenerate health, if applicable
pub fn regenerate_health(time: Res<Time>, mut health: Query<&mut Health>) {
    for mut health in health.iter_mut() {
        health.apply_regen(time.delta_seconds());
    }
}

#[derive(Component)]
pub struct HealthBar {
    pub offset: Vec2,
}

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
            for_game_states(),
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
    for (health, _entity, transform, _player) in health_query.iter_mut() {
        let health_percentage = health.current / health.maximum;

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
