use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};
use strum::EnumIter;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, EnumIter)]
pub enum MapId {
    #[default]
    Map1,
    Map2,
    Map3,
    Map4,
}

impl MapId {
    /// Returns the path for this maps ldtk path
    pub fn get_map_path(&self) -> &str {
        match self {
            _ => "maps/map_1.ldtk",
        }
    }

    /// Returns the path for this maps ui image
    pub fn get_ui_image_name(&self) -> &str {
        match self {
            MapId::Map1 => "ui/map/map_1.png",
            _ => "ui/map/map_default.png",
        }
    }

    /// Each maps can have a set of game play effects (such as faster spawn rates)
    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        match self {
            MapId::Map1 => {
                vec![GameplayEffect::new_abs(GameplayStat::SpawnRate, 1.0)]
            }
            _ => {
                vec![
                    GameplayEffect::new_abs(GameplayStat::SpawnRate, 3.0),
                    GameplayEffect::new_mul(GameplayStat::Health, 1.5),
                ]
            }
        }
    }
}
