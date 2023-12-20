use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Copy)]
pub struct Level(pub u64);

impl Level {
    /// Each level can add effects
    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        match self.0 {
            _ => {
                vec![GameplayEffect::new_mul(GameplayStat::MovementSpeed, 1.2)]
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

    pub fn percent_to_level_up(&self, total_xp: u64) -> f32 {
        let exp_required_for_current_level = Self::exp_required_for_next_level(&Level(self.0 - 1));
        let exp_required_for_next_level = Self::exp_required_for_next_level(self);

        let progress = (total_xp - exp_required_for_current_level) as f32
            / (exp_required_for_next_level - exp_required_for_current_level) as f32;

        progress
    }
}
