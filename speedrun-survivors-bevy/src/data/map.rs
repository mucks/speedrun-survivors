use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};
use bevy::prelude::*;
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

    /// Returns the dimensions of the imported map
    pub fn get_map_dimensions(&self) -> (f32, f32) {
        match self {
            MapId::Map1 => (512.0, 512.0),
            _ => (512.0, 512.0),
        }
    }

    pub fn get_map_scale(&self) -> f32 {
        match self {
            MapId::Map1 => 5.0,
            _ => 5.0,
        }
    }

    pub fn get_scaled_map_dimensions(&self) -> (f32, f32) {
        let (mut map_witdh, mut map_height) = self.get_map_dimensions();
        let map_scale = self.get_map_scale();
        map_witdh *= map_scale;
        map_height *= map_scale;
        (map_witdh, map_height)
    }
    pub fn is_at_border(&self, tr: Transform) -> bool {
        let (map_width, map_height) = self.get_scaled_map_dimensions();
        let border_offset = self.get_scaled_border_offset();

        let x = tr.translation.x;
        let y = tr.translation.y;

        let x_min = -map_width / 2. + border_offset.x;
        let x_max = map_width / 2. - border_offset.x;
        let y_min = -map_height / 2. + border_offset.y;
        let y_max = map_height / 2. - border_offset.y;

        x < x_min || x > x_max || y < y_min || y > y_max
    }

    pub fn get_border_offset(&self) -> Vec2 {
        match self {
            MapId::Map1 => Vec2::new(64., 32.),
            _ => Vec2::new(0., 0.),
        }
    }

    pub fn get_scaled_border_offset(&self) -> Vec2 {
        let mut border_offset = self.get_border_offset();
        let map_scale = self.get_map_scale();
        border_offset.x *= map_scale;
        border_offset.y *= map_scale;
        border_offset
    }

    pub fn get_map_transform(&self) -> Transform {
        let map_scale = self.get_map_scale();
        let (mut map_witdh, mut map_height) = self.get_map_dimensions();
        map_witdh *= map_scale;
        map_height *= map_scale;

        Transform {
            scale: Vec3::new(map_scale, map_scale, 0.1),
            translation: Vec3::new(-map_witdh / 2., -map_height / 2., -10.),
            ..Default::default()
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
