use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};
use strum::EnumIter;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum AbilityType {
    OrcaChopper,
    RugPull,
    ShitcoinCluster,
    WhaleDump,
}

impl AbilityType {
    pub fn select_randomly(amount: usize) -> Vec<AbilityType> {
        eprintln!("TODO: Select abilities randomly");

        vec![
            AbilityType::OrcaChopper,
            AbilityType::RugPull,
            AbilityType::ShitcoinCluster,
            AbilityType::WhaleDump,
        ]
    }

    pub fn get_gameplay_effects(&self, level: u8) -> Vec<GameplayEffect> {
        match self {
            AbilityType::OrcaChopper => Self::orca_chopper_effects(level),
            AbilityType::RugPull => Self::rug_pull_effects(level),
            AbilityType::ShitcoinCluster => Self::shitcoin_cluster_effects(level),
            AbilityType::WhaleDump => Self::whale_dump_effects(level),
        }
    }

    fn orca_chopper_effects(level: u8) -> Vec<GameplayEffect> {
        if level == 1 {
            return vec![
                GameplayEffect::new_abs(GameplayStat::OrcaCount, 1.0),
                GameplayEffect::new_abs(GameplayStat::OrcaSpeed, 400.0),
                GameplayEffect::new_abs(GameplayStat::OrcaDamage, -0.1),
            ];
        }

        vec![
            GameplayEffect::new_add(GameplayStat::OrcaCount, 1.0),
            GameplayEffect::new_add(GameplayStat::OrcaSpeed, 50.0),
            GameplayEffect::new_add(GameplayStat::OrcaDamage, -0.1),
        ]
    }

    fn rug_pull_effects(level: u8) -> Vec<GameplayEffect> {
        if level == 1 {
            return vec![
                GameplayEffect::new_abs(GameplayStat::RugPullInterval, 6.0),
                GameplayEffect::new_abs(GameplayStat::RugPullSpeed, 280.0),
                GameplayEffect::new_abs(GameplayStat::RugPullTTL, 0.8),
            ];
        }

        vec![
            GameplayEffect::new_sub(GameplayStat::RugPullInterval, 0.15),
            GameplayEffect::new_add(GameplayStat::RugPullSpeed, 5.0),
            GameplayEffect::new_add(GameplayStat::RugPullTTL, 0.08),
        ]
    }

    fn shitcoin_cluster_effects(level: u8) -> Vec<GameplayEffect> {
        if level == 1 {
            return vec![
                GameplayEffect::new_abs(GameplayStat::ShitcoinInterval, 5.0),
                GameplayEffect::new_abs(GameplayStat::ShitcoinMunitions, 2.0),
                GameplayEffect::new_abs(GameplayStat::ShitcoinDamage, 0.3),
            ];
        }

        vec![
            GameplayEffect::new_sub(GameplayStat::ShitcoinInterval, 0.15),
            GameplayEffect::new_add(GameplayStat::ShitcoinMunitions, 1.0),
            GameplayEffect::new_add(GameplayStat::ShitcoinDamage, 0.1),
        ]
    }

    fn whale_dump_effects(level: u8) -> Vec<GameplayEffect> {
        if level == 1 {
            return vec![
                GameplayEffect::new_abs(GameplayStat::WhaleDamage, 1.0),
                GameplayEffect::new_abs(GameplayStat::WhaleInterval, 5.0),
                GameplayEffect::new_abs(GameplayStat::WhaleArea, 70.0),
            ];
        }

        vec![
            GameplayEffect::new_mul(GameplayStat::WhaleDamage, 1.1),
            GameplayEffect::new_div(GameplayStat::WhaleInterval, 1.05),
            GameplayEffect::new_mul(GameplayStat::WhaleArea, 1.1),
        ]
    }
}

impl std::fmt::Display for AbilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbilityType::OrcaChopper => write!(f, "Orca Chopper"),
            AbilityType::RugPull => write!(f, "Rug Pull"),
            AbilityType::ShitcoinCluster => write!(f, "Shitcoin Cluster"),
            AbilityType::WhaleDump => write!(f, "Whale Dump"),
        }
    }
}
