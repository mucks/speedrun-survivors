use bevy::prelude::*;

use crate::state::AppState;

use super::{
    health::{Health, HealthChangeEvent, HealthChangeTargetType},
    status_effect::StatusEffectController,
};

pub struct CombatTextPlugin;

impl Plugin for CombatTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_health_change_event, float_combat_text).run_if(in_state(AppState::GameRunning)),
        );
    }
}

fn on_status_effect_event(
    mut health_change: EventReader<HealthChangeEvent>,
    mut health_query: Query<&StatusEffectController, With<StatusEffectController>>,
    mut commands: Commands,
) {
}

fn on_health_change_event(
    mut health_change: EventReader<HealthChangeEvent>,
    mut health_query: Query<&Transform, With<Health>>,
    mut commands: Commands,
) {
    for ev in health_change.iter() {
        let Ok(health_tr) = health_query.get_mut(ev.entity) else {
            return;
        };

        let mut color = match ev.target_type {
            HealthChangeTargetType::Player => Color::RED,
            HealthChangeTargetType::Enemy => Color::YELLOW,
        };

        if ev.health_change > 0.0 {
            color = Color::GREEN;
        }

        spawn_combat_text(
            &mut commands,
            &format!("{}", ev.health_change),
            health_tr.translation,
            color,
        );
    }
}

#[derive(Component)]
pub struct CombatText {
    timer: Timer,
}

pub fn spawn_combat_text(commands: &mut Commands, text: &str, position: Vec3, color: Color) {
    let transform = Transform {
        translation: Vec3::new(position.x, position.y, 1.0),
        scale: Vec3::new(0.5, 0.5, 1.0),
        ..Default::default()
    };

    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text.to_string(),
                    style: TextStyle {
                        font_size: 50.0,
                        color,
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            transform,
            ..Default::default()
        })
        .insert(CombatText {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        });
}

pub fn float_combat_text(
    mut query: Query<(&mut Transform, &mut Text, &mut CombatText), With<CombatText>>,
    time: Res<Time>,
) {
    for (mut transform, mut text, mut ct) in query.iter_mut() {
        transform.translation.y += 1.0 * time.delta_seconds();
        text.sections[0].style.color.set_a(ct.timer.percent());
        ct.timer.tick(time.delta());
        if ct.timer.finished() {
            text.sections[0].style.color.set_a(0.0);
        }
    }
}