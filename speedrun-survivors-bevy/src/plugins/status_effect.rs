use bevy::prelude::*;

pub struct StatusEffectPlugin;

impl Plugin for StatusEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StatusEffectEvent>()
            .add_systems(Update, (apply_status_effects, on_status_effect_event));
    }
}

#[derive(Debug)]
pub enum StatusEffectEventType {
    Apply,
    Remove,
}

#[derive(Event)]
pub struct StatusEffectEvent {
    pub effect: StatusEffect,
    pub entity: Entity,
    pub event_type: StatusEffectEventType,
}

#[derive(Component, Debug, Clone)]
pub struct StatusEffectController {
    pub effects: Vec<StatusEffect>,
}

#[derive(Debug, Clone, Copy)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub current_duration: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusEffectType {
    Knockback(Vec3),
    DeathIsTemporary,
}

fn on_status_effect_event(
    mut rx_status: EventReader<StatusEffectEvent>,
    mut status_effect_query: Query<&mut StatusEffectController>,
) {
    for ev in rx_status.iter() {
        let Ok(mut status_effect_cont) = status_effect_query.get_mut(ev.entity) else {
            return;
        };

        match ev.event_type {
            StatusEffectEventType::Apply => {
                status_effect_cont.effects.push(ev.effect);
            }
            StatusEffectEventType::Remove => {
                status_effect_cont
                    .effects
                    .retain(|effect| effect.effect_type != ev.effect.effect_type);
            }
        }
    }
}

pub fn apply_status_effects(
    time: Res<Time>,
    mut query: Query<(&mut StatusEffectController, &mut Transform, Entity)>,
    mut tx_status: EventWriter<StatusEffectEvent>,
) {
    for (mut status_effect_cont, mut transform, ent) in query.iter_mut() {
        for status_effect in status_effect_cont.effects.iter_mut() {
            match status_effect.effect_type {
                StatusEffectType::Knockback(knockback) => {
                    transform.translation +=
                        knockback / status_effect.duration * time.delta_seconds();
                    transform.translation.z = 0.;

                    status_effect.current_duration -= time.delta_seconds();

                    if status_effect.current_duration <= 0. {
                        tx_status.send(StatusEffectEvent {
                            effect: *status_effect,
                            entity: ent,
                            event_type: StatusEffectEventType::Remove,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
