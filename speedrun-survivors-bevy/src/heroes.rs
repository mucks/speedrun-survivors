use bevy::prelude::*;

use crate::{animation::Animation, plugins::assets::GameAssets, weapon::weapon_type::WeaponType};

#[derive(Clone, Debug, Eq, Hash, Default, PartialEq)]
pub enum HeroType {
    Pepe,
    #[default]
    BonkInu,
    PudgyPenguin,
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
            _ => "ui/heroes/mystery.png",
        }
    }

    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        //TODO each hero needs his own complete set of ABS type stats
        vec![]
    }

    pub fn into_iter() -> core::array::IntoIter<HeroType, 6> {
        [
            HeroType::Pepe,
            HeroType::BonkInu,
            HeroType::PudgyPenguin,
            HeroType::MadLad,
            HeroType::MysteryHero1,
            HeroType::MysteryHero2,
        ]
        .into_iter()
    }
}

//TODO doesnt belong here REFACTOR
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Levels {
    Level1,
    Level2,
    Level3,
    Level4,
}

impl Levels {
    pub fn get_level_path(&self) -> &str {
        match self {
            _ => "level/level.ldtk",
        }
    }

    pub fn get_ui_image_name(&self) -> &str {
        match self {
            Levels::Level1 => "ui/level/level_1.png",
            _ => "ui/level/level_unk.png",
        }
    }

    pub fn get_gameplay_effects(&self) -> Vec<GameplayEffect> {
        //TODO each level can have a set of game play effects (such as faster spawn rates)
        vec![]
    }

    pub fn into_iter() -> core::array::IntoIter<Levels, 4> {
        [
            Levels::Level1,
            Levels::Level2,
            Levels::Level3,
            Levels::Level4,
        ]
        .into_iter()
    }
}

//TODO doesnt belong here REFACTOR
//TODO not sure how to do these - need some calculators to ADD hero base stats + level base stats and then multiply with item based multipliers or so
// hero has base stats
// level has base stats
// items are multipliers
// implement fn modify_with(mod: Modifier)

#[derive(PartialEq)]
enum GameplayOp {
    Abs,
    Add,
    Sub,
    Mul,
    Div,
}
enum GameplayStat {
    MovementSpeed,
    AttackRate,
    Health,
    HealthRegen,
    SpawnRate,
    Damage,
}
struct GameplayEffect {
    pub stat: GameplayStat,
    pub op: GameplayOp,
    pub val: f64,
}

struct GameplayEffectContainer {
    hero: Vec<GameplayEffect>,
    level: Vec<GameplayEffect>,
    items: Vec<GameplayEffect>,
    //TODO need this to be (Entity, GameplayEffect) or so, so that we can remove and recalculate if it becomes necessary (LATER)
    // probably need this for weapons... since they can be switched
}

impl GameplayEffectContainer {
    fn flatten(&self) -> GameplayStatsFlatPacked {
        let mut flat = GameplayStatsFlatPacked::default();

        for effect in &self.hero {
            if effect.op != GameplayOp::Abs {
                panic!("GameplayStat error: Heroes Gameplay stats must only use ABS operator as they are the base for all calculations");
            }

            flat.modify(&effect.stat, &effect.op, effect.val);
        }

        if !flat.are_valid_base_stats() {
            panic!("GameplayStat error: Hero stats are not properly configured");
        }

        for effect in &self.level {
            flat.modify(&effect.stat, &effect.op, effect.val);
        }

        for effect in &self.items {
            flat.modify(&effect.stat, &effect.op, effect.val);
        }

        flat
    }
}

#[derive(Default)]
struct GameplayStatsFlatPacked {
    movement_speed: f64,
    attack_rate: f64,
    health: f64,
    health_regen: f64,
    spawn_rate: f64,
    damage: f64,
}

impl GameplayStatsFlatPacked {
    pub fn are_valid_base_stats(&self) -> bool {
        self.movement_speed > 0.
            && self.attack_rate > 0.
            && self.attack_rate > 0.
            && self.health > 0.
            && self.health_regen > 0.
    }
    pub fn modify(&mut self, stat: &GameplayStat, op: &GameplayOp, val: f64) {
        match stat {
            GameplayStat::MovementSpeed => {
                self.movement_speed = Self::calculate(op, self.movement_speed, val)
            }
            GameplayStat::AttackRate => {
                self.attack_rate = Self::calculate(op, self.movement_speed, val)
            }

            GameplayStat::Health => self.health = Self::calculate(op, self.movement_speed, val),
            GameplayStat::HealthRegen => {
                self.health_regen = Self::calculate(op, self.movement_speed, val)
            }

            GameplayStat::SpawnRate => {
                self.spawn_rate = Self::calculate(op, self.movement_speed, val)
            }
            GameplayStat::Damage => self.damage = Self::calculate(op, self.movement_speed, val),
        }
    }

    fn calculate(op: &GameplayOp, operand: f64, operator: f64) -> f64 {
        match op {
            GameplayOp::Abs => operator,
            GameplayOp::Add => operand + operator,
            GameplayOp::Sub => operand - operator,
            GameplayOp::Mul => operand * operator,
            GameplayOp::Div => operand / operator,
        }
    }
}
