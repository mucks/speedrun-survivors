use crate::state::AppState;
use bevy::prelude::*;
use rand::Rng;

pub struct CameraShakePlugin;

impl Plugin for CameraShakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::GameRunning), on_enter_game_running)
            .add_systems(OnExit(AppState::GameRunning), on_exit_game_running)
            .add_systems(Update, (on_update).run_if(in_state(AppState::GameRunning)))
            .add_event::<CameraImpact>()
            .insert_resource(Shake::create_shake(&CameraImpactStrength::Idle));
    }
}

fn on_enter_game_running(mut shake: ResMut<Shake>) {
    shake.trauma = 0f32;
}

fn on_exit_game_running(mut shake: ResMut<Shake>) {
    shake.trauma = 0f32;
}

fn on_update(
    mut shake: ResMut<Shake>,
    mut query_camera: Query<(&Camera, &mut Transform)>,
    time: Res<Time>,
    mut impact_events: EventReader<CameraImpact>,
) {
    // If there is a new impact event, reset our data from that, will cancel out the current shake
    if let Some(impact) = impact_events.iter().last() {
        *shake = Shake::create_shake(&impact.strength);
    }

    // Return if trauma ran out
    if shake.trauma <= 0f32 {
        return;
    }

    // Get the camera
    let Ok((_, mut camera_transform)) = query_camera.get_single_mut() else {
        return;
    };

    // Update shake
    let mut rng = rand::thread_rng();

    shake.trauma = f32::max(shake.trauma - shake.decay * time.delta_seconds(), 0.0);

    let trauma_amount = f32::powf(shake.trauma, shake.trauma_power);
    if trauma_amount > 0.0 {
        let offset = shake.max_offset * trauma_amount * Vec2::new(rng.gen(), rng.gen());

        let shake_translation = Vec3::new(offset.x, offset.y, 0.0);

        let shake_rotation = Quat::from_euler(
            EulerRot::YXZ,
            0.0,
            0.0,
            shake.max_roll * trauma_amount * rng.gen::<f32>(),
        );

        camera_transform.translation = shake_translation;
        camera_transform.rotation = shake_rotation;
    } else {
        camera_transform.translation = Vec3::default();
        camera_transform.rotation = Quat::default();
    }
}

#[derive(Event)]
pub struct CameraImpact {
    pub strength: CameraImpactStrength,
}

pub enum CameraImpactStrength {
    Idle,
    Light,
    Medium,
    Heavy,
    Absurd,
}

#[derive(Resource)]
struct Shake {
    /// The maximum amount of offset in the X and Y dimensions.
    /// Defaults to `Vec2::new(100.0, 100.0)`.
    max_offset: Vec2,
    /// The maximum amount of roll allowed in radians.
    /// Defaults to `0.1`.
    max_roll: f32,
    /// The starting trauma when created.
    /// Defaults to `0.0`.
    trauma: f32,
    /// The exponent of the trauma used when calculating offset and rotational shakiness.
    /// Should likely be set to a value between `2.0` and `3.0`.
    /// Defaults to `2.0`.
    trauma_power: f32,
    /// The percentage to decrease trauma per second.
    /// If set to 1, there will be no trauma after 1 second. If set to 0, trauma will not decrease over time.
    /// If set below 0, trauma will *increase* over time, and if set above 1, trauma will decrease very quickly.
    /// Defaults to `0.8`.
    decay: f32,
}

impl Shake {
    fn create_shake(strength: &CameraImpactStrength) -> Self {
        let mut shake = Self {
            max_offset: Vec2::new(100.0, 100.0),
            max_roll: 0.1,
            trauma: 1.0,
            trauma_power: 2.0,
            decay: 0.8,
        };

        match strength {
            CameraImpactStrength::Light => {
                shake.trauma = 0.6;
            }
            CameraImpactStrength::Medium => {
            }
            CameraImpactStrength::Heavy => {
                shake.trauma = 1.4;
                shake.max_roll = 0.15;
            }
            CameraImpactStrength::Absurd => {
                shake.trauma = 1.8;
                shake.max_roll = 0.2;
            }
            _ => {}
        }

        shake
    }
}
