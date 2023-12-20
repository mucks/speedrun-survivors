use bevy::{
    asset::AssetServer,
    ecs::system::Res,
    math::{Vec2, Vec3},
    sprite::TextureAtlas,
};

use crate::plugins::health::Health;

use super::Enemy;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, strum::EnumIter)]
pub enum EnemyType {
    Spider,
    Snake,
    Golem,
    Slime,
}

impl EnemyType {
    pub fn random() -> Self {
        match rand::random::<u8>() % 4 {
            0 => EnemyType::Spider,
            1 => EnemyType::Snake,
            2 => EnemyType::Golem,
            3 => EnemyType::Slime,
            _ => unreachable!(),
        }
    }
    pub fn scale(&self) -> Vec3 {
        match self {
            EnemyType::Spider => Vec3::splat(4.),
            EnemyType::Snake => Vec3::splat(2.),
            EnemyType::Golem => Vec3::splat(4.),
            EnemyType::Slime => Vec3::splat(1.),
        }
    }

    pub fn enemy(&self) -> Enemy {
        match self {
            EnemyType::Spider => Enemy {
                speed: 100.,
                attack: 1.,
                kind: EnemyType::Spider,
            },
            EnemyType::Snake => Enemy {
                speed: 75.,
                attack: 2.,
                kind: EnemyType::Snake,
            },
            EnemyType::Golem => Enemy {
                speed: 50.,
                attack: 3.,
                kind: EnemyType::Golem,
            },
            EnemyType::Slime => Enemy {
                speed: 25.,
                attack: 4.,
                kind: EnemyType::Slime,
            },
        }
    }

    pub fn health(&self) -> Health {
        match self {
            EnemyType::Spider => Health::new(2., 2., 0., None),
            EnemyType::Snake => Health::new(3., 3., 0., None),
            EnemyType::Golem => Health::new(6., 6., 0., None),
            EnemyType::Slime => Health::new(1., 1., 0., None),
        }
    }

    pub fn frames(&self) -> usize {
        match self {
            EnemyType::Spider => 2,
            EnemyType::Snake => 2,
            EnemyType::Golem => 3,
            EnemyType::Slime => 8,
        }
    }

    pub fn texture_atlas(&self, asset_server: &Res<AssetServer>) -> TextureAtlas {
        let sprite_path = match self {
            EnemyType::Spider => "sprites/enemy/enemy-spider.png",
            EnemyType::Snake => "sprites/enemy/enemy-snake.png",
            EnemyType::Golem => "sprites/enemy/enemy-golem.png",
            EnemyType::Slime => "sprites/enemy/enemy-slime.png",
        };

        let dimensions = match self {
            EnemyType::Spider => Vec2::new(32., 32.),
            EnemyType::Snake => Vec2::new(22., 48.),
            EnemyType::Golem => Vec2::new(32., 32.),
            EnemyType::Slime => Vec2::new(63., 64.),
        };

        let texture_handle = asset_server.load(sprite_path);
        TextureAtlas::from_grid(
            texture_handle,
            dimensions,
            self.frames(),
            1,
            Some(Vec2::new(1., 1.)),
            None,
        )
    }

    pub fn get_coin_reward(&self) -> u64 {
        100
    }

    pub fn get_exp_reward(&self) -> u64 {
        77
    }
}
