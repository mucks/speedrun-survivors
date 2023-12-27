use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Copy)]
pub struct Level(pub u64);

impl Level {
    /// Each level can add effects
    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        match self.0 {
            _ => {
                vec![
                    GameplayEffect::new_mul(GameplayStat::MovementSpeed, 1.1),
                    GameplayEffect::new_add(GameplayStat::HealthCap, 5.0),
                    GameplayEffect::new_add(GameplayStat::HealthRegen, 1.0),
                    GameplayEffect::new_add(GameplayStat::OrcaCount, 1.0),
                    GameplayEffect::new_add(GameplayStat::OrcaSpeed, 50.0),
                    GameplayEffect::new_add(GameplayStat::OrcaDamage, -0.1),
                    GameplayEffect::new_add(GameplayStat::PickupDistance, 50.0),
                    GameplayEffect::new_mul(GameplayStat::WhaleDamage, 1.1),
                    GameplayEffect::new_div(GameplayStat::WhaleInterval, 1.05),
                    GameplayEffect::new_mul(GameplayStat::WhaleArea, 1.1),
                    GameplayEffect::new_sub(GameplayStat::ShitcoinInterval, 0.1),
                    GameplayEffect::new_add(GameplayStat::ShitcoinMunitions, 1.0),
                    GameplayEffect::new_add(GameplayStat::ShitcoinDamage, 0.1),
                ]
            }
        }
    }

    /// Amount of experience required to gain the next level
    fn exp_required_for_next_level(current_level: &Level) -> u64 {
        (300. * (current_level.0 as f64).powi(2)) as u64
    }

    /// Returns the next Level if the total experience is sufficient, None otherwise
    pub fn has_leveled_up(&self, total_xp: u64) -> Option<Level> {
        if total_xp >= Self::exp_required_for_next_level(self) {
            Some(Level(self.0 + 1))
        } else {
            None
        }
    }

    /// Returns the progress towards the next level
    pub fn percent_to_level_up(&self, total_xp: u64) -> f32 {
        let lvl_current = Self::exp_required_for_next_level(&Level(self.0 - 1));
        let lvl_next = Self::exp_required_for_next_level(self);

        (total_xp - lvl_current) as f32 / (lvl_next - lvl_current) as f32
    }
}
