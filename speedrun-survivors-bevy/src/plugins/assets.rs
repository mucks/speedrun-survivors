use crate::data::hero::HeroType;
use crate::data::map::MapId;
use crate::enemy::enemy_type::EnemyType;
use crate::weapon::weapon_animation_effect::WeaponAnimationEffect;
use crate::weapon::weapon_type::WeaponType;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::LdtkAsset;
use strum::IntoEnumIterator;

#[derive(Resource)]
pub struct UiAssets {
    pub font_expanse: Handle<Font>,
    pub buff_1: UiImage,
    pub checkbox_o: UiImage,
    pub checkbox_x: UiImage,
    pub weapons: HashMap<WeaponType, UiImage>,
    pub heroes: HashMap<HeroType, UiImage>,
    pub maps: HashMap<MapId, UiImage>,
}

#[derive(Resource)]
pub struct GameAssets {
    pub heroes: HashMap<HeroType, Handle<Image>>,
    pub map: (MapId, Handle<LdtkAsset>),
    pub weapons: HashMap<WeaponType, Handle<TextureAtlas>>,
    pub weapon_animation_effects: HashMap<WeaponAnimationEffect, Handle<TextureAtlas>>,
    pub enemies: HashMap<EnemyType, Handle<TextureAtlas>>,
    pub skull: Handle<TextureAtlas>,
    pub orca: Handle<Image>,
    pub whale: Handle<Image>,
    pub shitcoin: Handle<Image>,
    pub pickup_exp: Handle<Image>,
    pub pickup_coin: Handle<Image>,
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
    let heroes: HashMap<HeroType, UiImage> = HeroType::iter()
        .map(|hero| (hero, asset_server.load(hero.get_ui_image_name()).into()))
        .collect();

    // Load ui image for each map
    let maps: HashMap<MapId, UiImage> = MapId::iter()
        .map(|map| (map, asset_server.load(map.get_ui_image_name()).into()))
        .collect();

    // Load ui image for each weapon
    let weapons: HashMap<WeaponType, UiImage> = WeaponType::iter()
        .map(|weapon| (weapon, asset_server.load(weapon.get_ui_image_name()).into()))
        .collect();

    commands.insert_resource(UiAssets {
        font_expanse: asset_server.load("ui/expanse.otf"),
        buff_1: asset_server.load("ui/buff_1.png").into(),
        checkbox_o: asset_server.load("ui/checkbox_o.png").into(),
        checkbox_x: asset_server.load("ui/checkbox_x.png").into(),
        weapons,
        heroes,
        maps,
    });

    // Load sprite sheets for each hero
    let heroes: HashMap<HeroType, Handle<Image>> = HeroType::iter()
        .map(|hero| (hero, asset_server.load(hero.get_sprite_name()).into()))
        .collect();

    let map_id = MapId::Map1;
    let map_asset = asset_server.load(map_id.get_map_path());

    // Load texture atlases for each weapons
    let weapons: HashMap<WeaponType, Handle<TextureAtlas>> = WeaponType::iter()
        .map(|weapon| {
            (
                weapon,
                texture_atlases.add(weapon.texture_atlas(&asset_server)),
            )
        })
        .collect();

    // Load weapon texture atlases
    let weapon_animation_effects: HashMap<WeaponAnimationEffect, Handle<TextureAtlas>> =
        WeaponAnimationEffect::iter()
            .map(|anim| (anim, texture_atlases.add(anim.texture_atlas(&asset_server))))
            .collect();

    // Load enemy texture atlases
    let enemies: HashMap<EnemyType, Handle<TextureAtlas>> = EnemyType::iter()
        .map(|enemy| {
            (
                enemy,
                texture_atlases.add(enemy.texture_atlas(&asset_server)),
            )
        })
        .collect();

    let skull = TextureAtlas::from_grid(
        asset_server.load("sprites/misc/skull.png"),
        Vec2::new(64., 64.),
        1,
        1,
        Some(Vec2::new(1., 1.)),
        None,
    );

    commands.insert_resource(GameAssets {
        heroes,
        map: (map_id, map_asset),
        weapons,
        weapon_animation_effects,
        enemies,
        skull: texture_atlases.add(skull),
        orca: asset_server.load("sprites/passives/orca.png"),
        whale: asset_server.load("sprites/passives/whale.png"),
        shitcoin: asset_server.load("sprites/passives/shitcoin.png"),
        pickup_exp: asset_server.load("sprites/misc/exp.png"),
        pickup_coin: asset_server.load("sprites/misc/coin.png"),
    });
}
