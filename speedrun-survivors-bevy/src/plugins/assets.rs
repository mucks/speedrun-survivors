use crate::enemy::enemy_type::EnemyType;
use crate::heroes::{HeroType, Levels};
use crate::weapon::weapon_animation_effect::{self, WeaponAnimationEffect};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::LdtkAsset;
use strum::IntoEnumIterator;

use crate::weapon::weapon_type::WeaponType;

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub buff_1: UiImage,
    pub checkbox_o: UiImage,
    pub checkbox_x: UiImage,
    pub weapons: Vec<(WeaponType, UiImage)>,
    pub heroes: HashMap<HeroType, UiImage>,
    pub levels: HashMap<Levels, UiImage>,
}

#[derive(Resource)]
pub struct GameAssets {
    pub heroes: HashMap<HeroType, Handle<Image>>,
    pub level: Handle<LdtkAsset>,
    pub weapons: HashMap<WeaponType, Handle<TextureAtlas>>,
    pub weapon_animation_effects: HashMap<WeaponAnimationEffect, Handle<TextureAtlas>>,
    pub enemies: HashMap<EnemyType, Handle<TextureAtlas>>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load ui image for each hero
    let mut heroes: HashMap<HeroType, UiImage> = HashMap::new();
    for hero in HeroType::into_iter() {
        heroes.insert(
            hero.clone(),
            asset_server.load(hero.get_ui_image_name()).into(),
        );
    }

    // Load ui image for each level
    let mut levels: HashMap<Levels, UiImage> = HashMap::new();
    for level in Levels::into_iter() {
        levels.insert(
            level.clone(),
            asset_server.load(level.get_ui_image_name()).into(),
        );
    }

    commands.insert_resource(UiAssets {
        font: asset_server.load("ui/expanse.otf"),
        buff_1: asset_server.load("ui/buff_1.png").into(),
        //TODO refactor this to hashmap as well with an iter() as the hero images above?
        checkbox_o: asset_server.load("ui/checkbox_o.png").into(),
        checkbox_x: asset_server.load("ui/checkbox_x.png").into(),
        weapons: vec![
            (
                WeaponType::Hammer,
                asset_server.load("ui/weapon/hammer-icon.png").into(),
            ),
            (
                WeaponType::Sword,
                asset_server.load("ui/weapon/sword-icon.png").into(),
            ),
            (
                WeaponType::Gun,
                asset_server.load("ui/weapon/gun-icon.png").into(),
            ),
            (
                WeaponType::FlameThrower,
                asset_server
                    .load("ui/weapon/flamethrower-ui-icon.png")
                    .into(),
            ),
        ],
        heroes,
        levels,
    });

    // Load sprite sheets for each hero
    let mut heroes: HashMap<HeroType, Handle<Image>> = HashMap::new();
    for hero in HeroType::into_iter() {
        heroes.insert(
            hero.clone(),
            asset_server.load(hero.get_sprite_name()).into(),
        );
    }

    let level_asset = asset_server.load("level/level.ldtk");

    let mut weapons = HashMap::new();

    for weapon in WeaponType::iter() {
        weapons.insert(
            weapon.clone(),
            texture_atlases.add(weapon.texture_atlas(&asset_server)),
        );
    }

    let mut weapon_animation_effects = HashMap::new();

    for weapon_animation_effect in WeaponAnimationEffect::iter() {
        weapon_animation_effects.insert(
            weapon_animation_effect.clone(),
            texture_atlases.add(weapon_animation_effect.texture_atlas(&asset_server)),
        );
    }

    let mut enemies = HashMap::new();

    for enemy in EnemyType::iter() {
        enemies.insert(
            enemy.clone(),
            texture_atlases.add(enemy.texture_atlas(&asset_server)),
        );
    }

    commands.insert_resource(GameAssets {
        heroes,
        level: level_asset,
        weapons,
        weapon_animation_effects,
        enemies,
    });
}
