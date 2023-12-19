use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Level {
    level: u64,
}

impl Level {
    /// Each level can add effects
    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        match self.level {
            2 => {
                vec![GameplayEffect::new_mul(GameplayStat::Health, 1.1)]
            }
            _ => {
                vec![GameplayEffect::new_mul(GameplayStat::Health, 1.2)]
            }
        }
    }
}
