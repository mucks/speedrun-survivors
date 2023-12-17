use crate::heroes::HeroType;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::weapon::weapon_type::WeaponType;

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub buff_1: UiImage,
    pub weapons: Vec<(WeaponType, UiImage)>,
    pub heroes: HashMap<HeroType, UiImage>,
}

#[derive(Resource)]
pub struct GameAssets {
    pub heroes: HashMap<HeroType, Handle<Image>>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load ui image for each hero
    let mut heroes: HashMap<HeroType, UiImage> = HashMap::new();
    for hero in HeroType::into_iter() {
        heroes.insert(
            hero.clone(),
            asset_server.load(hero.get_ui_image_name()).into(),
        );
    }

    commands.insert_resource(UiAssets {
        font: asset_server.load("ui/expanse.otf"),
        buff_1: asset_server.load("ui/buff_1.png").into(),
        //TODO refactor this to hashmap as well with an iter() as the hero images above?
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
        ],
        heroes,
    });

    // Load sprite sheets for each hero
    let mut heroes: HashMap<HeroType, Handle<Image>> = HashMap::new();
    for hero in HeroType::into_iter() {
        heroes.insert(
            hero.clone(),
            asset_server.load(hero.get_sprite_name()).into(),
        );
    }

    commands.insert_resource(GameAssets { heroes });
}
