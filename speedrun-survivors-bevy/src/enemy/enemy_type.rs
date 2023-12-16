use bevy::{
    asset::AssetServer,
    ecs::system::Res,
    math::{Vec2, Vec3},
    sprite::TextureAtlas,
};

use crate::plugins::health::Health;

use super::Enemy;

pub enum EnemyType {
    Spider,
    Snake,
}

impl EnemyType {
    pub fn random() -> Self {
        match rand::random::<u8>() % 2 {
            0 => EnemyType::Spider,
            _ => EnemyType::Snake,
        }
    }
    pub fn scale(&self) -> Vec3 {
        match self {
            EnemyType::Spider => Vec3::splat(4.),
            EnemyType::Snake => Vec3::splat(3.),
        }
    }

    pub fn enemy(&self) -> Enemy {
        match self {
            EnemyType::Spider => Enemy {
                speed: 100.,
                attack: 1.,
            },
            EnemyType::Snake => Enemy {
                speed: 75.,
                attack: 2.,
            },
        }
    }

    pub fn health(&self) -> Health {
        match self {
            EnemyType::Spider => Health::new(2., 2., 0., None),
            EnemyType::Snake => Health::new(4., 4., 0., None),
        }
    }

    pub fn texture_atlas(&self, asset_server: &Res<AssetServer>) -> TextureAtlas {
        let sprite_path = match self {
            EnemyType::Spider => "sprites/enemy/enemy-spider.png",
            EnemyType::Snake => "sprites/enemy/enemy-snake.png",
        };

        let dimensions = match self {
            EnemyType::Spider => Vec2::new(32., 32.),
            EnemyType::Snake => Vec2::new(22., 48.),
        };

        let texture_handle = asset_server.load(sprite_path);
        TextureAtlas::from_grid(
            texture_handle,
            dimensions,
            2,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        )
    }
}
