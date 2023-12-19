use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};
use strum::EnumIter;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, EnumIter)]
pub enum ItemType {
    #[default]
    RingOfPower,
    BonkInuBattleBracers,
    WindForce,
}

impl ItemType {
    /// The path to the ui image for this item
    pub fn get_ui_image_name(&self) -> &str {
        match self {
            _ => "ui/buff_1.png",
        }
    }

    /// Gameplay Effects for this item
    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        match self {
            ItemType::RingOfPower => {
                vec![GameplayEffect::new_add(GameplayStat::Health, 44.0)]
            }
            _ => {
                vec![
                    GameplayEffect::new_add(GameplayStat::Damage, 3.0),
                    GameplayEffect::new_add(GameplayStat::Health, 13.5),
                ]
            }
        }
    }
}
