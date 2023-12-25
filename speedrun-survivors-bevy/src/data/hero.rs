use bevy::prelude::*;
use strum::EnumIter;

use crate::plugins::gameplay_effects::{GameplayEffect, GameplayStat};
use crate::{animation::Animation, plugins::assets::GameAssets, weapon::weapon_type::WeaponType};

#[derive(Clone, Copy, Debug, Eq, Hash, Default, PartialEq, EnumIter)]
pub enum HeroType {
    Pepe,
    #[default]
    BonkInu,
    Orca,
    MadLad,
    MysteryHero1,
    MysteryHero2,
}

impl HeroType {
    pub fn walk_animation(&self) -> Animation {
        match self {
            HeroType::Pepe => Animation {
                start: 1,
                end: 4,
                looping: true,
                cooldown: 0.1,
            },
            HeroType::BonkInu => Animation {
                start: 1,
                end: 5,
                looping: true,
                cooldown: 0.1,
            },
            _ => Animation {
                start: 1,
                end: 4,
                looping: true,
                cooldown: 0.1,
            },
        }
    }

    pub fn weapon_offset(&self, weapon_type: WeaponType) -> Vec2 {
        match self {
            HeroType::Pepe => match weapon_type {
                WeaponType::Hammer => Vec2::new(50., 30.),
                WeaponType::Sword => Vec2::new(35., 15.),
                WeaponType::Gun => Vec2::new(0., 0.),
                WeaponType::FlameThrower => Vec2::new(50., 20.),
            },
            HeroType::BonkInu => match weapon_type {
                WeaponType::Hammer => Vec2::new(50., 30.),
                WeaponType::Sword => Vec2::new(45., 15.),
                WeaponType::Gun => Vec2::new(0., 0.),
                WeaponType::FlameThrower => Vec2::new(50., -10.),
            },
            _ => match weapon_type {
                WeaponType::Hammer => Vec2::new(0., 0.),
                WeaponType::Sword => Vec2::new(0., 0.),
                WeaponType::Gun => Vec2::new(0., 0.),
                WeaponType::FlameThrower => Vec2::new(0., 0.),
            },
        }
    }

    pub fn splat_scale(&self) -> f32 {
        match self {
            HeroType::Pepe => 2.,
            HeroType::BonkInu => 1.4,
            _ => 3.5,
        }
    }

    pub fn texture_atlas(&self, game_assets: &Res<GameAssets>) -> TextureAtlas {
        let texture_handle = game_assets.heroes.get(self).unwrap().clone();

        let (size, frames) = match self {
            HeroType::Pepe => (Vec2::new(32., 56.), 4),
            HeroType::BonkInu => (Vec2::new(57., 64.), 5),
            _ => (Vec2::new(32., 56.), 4),
        };

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            size,
            frames,
            1,
            Some(Vec2::new(1., 1.)),
            None,
        );

        texture_atlas
    }

    pub fn get_sprite_name(&self) -> &str {
        match self {
            HeroType::Pepe => "sprites/player/pepe.png",
            HeroType::BonkInu => "sprites/player/bonk-walking.png",
            _ => "sprites/player/pepe.png",
        }
    }

    pub fn get_ui_image_name(&self) -> &str {
        match self {
            HeroType::Pepe => "ui/heroes/pepe.png",
            HeroType::BonkInu => "ui/heroes/bonk_inu.png",
            HeroType::Orca => "ui/heroes/orca.png",
            HeroType::MadLad => "ui/heroes/madlad.png",
            _ => "ui/heroes/mystery.png",
        }
    }

    /// Each hero needs its own complete set of ABS type stats
    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        match self {
            HeroType::Pepe => {
                vec![
                    GameplayEffect::new_abs(GameplayStat::HealthCap, 100.0),
                    GameplayEffect::new_abs(GameplayStat::HealthRegen, 10.0),
                    GameplayEffect::new_abs(GameplayStat::Damage, 5.0),
                    GameplayEffect::new_abs(GameplayStat::AttackRate, 5.0),
                    GameplayEffect::new_abs(GameplayStat::MovementSpeed, 100.0),
                    GameplayEffect::new_abs(GameplayStat::OrcaSpeed, 400.0),
                    GameplayEffect::new_abs(GameplayStat::PickupDistance, 40.0),
                    GameplayEffect::new_abs(GameplayStat::WhaleDamage, 1.0),
                    GameplayEffect::new_abs(GameplayStat::WhaleInterval, 5.0),
                    GameplayEffect::new_abs(GameplayStat::WhaleArea, 70.0),
                ]
            }
            _ => {
                vec![
                    GameplayEffect::new_abs(GameplayStat::HealthCap, 120.0),
                    GameplayEffect::new_abs(GameplayStat::HealthRegen, 5.0),
                    GameplayEffect::new_abs(GameplayStat::Damage, 5.0),
                    GameplayEffect::new_abs(GameplayStat::AttackRate, 5.0),
                    GameplayEffect::new_abs(GameplayStat::MovementSpeed, 120.0),
                    GameplayEffect::new_abs(GameplayStat::OrcaSpeed, 400.0),
                    GameplayEffect::new_abs(GameplayStat::PickupDistance, 40.0),
                    GameplayEffect::new_abs(GameplayStat::WhaleDamage, 1.0),
                    GameplayEffect::new_abs(GameplayStat::WhaleInterval, 5.0),
                    GameplayEffect::new_abs(GameplayStat::WhaleArea, 70.0),
                ]
            }
        }
    }
}
