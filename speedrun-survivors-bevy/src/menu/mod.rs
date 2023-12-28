mod game_over;
mod level_up;
mod pause;
mod splash_screen;

use crate::data::hero::HeroType;
use crate::data::item::ItemType;
use crate::data::map::MapId;
use crate::menu::game_over::menu_game_over;
use crate::menu::level_up::{menu_level_up, on_level_up_menu_button_action};
use crate::menu::pause::menu_pause;
use crate::menu::splash_screen::menu_splash_screen;
use crate::plugins::assets::UiAssets;
use crate::plugins::gameplay_effects::GameplayEffectEvent;
use crate::state::{AppState, ForState};
use crate::GameAction;
use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::app::AppExit;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use strum::IntoEnumIterator;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const BTN_BORDER_DEFAULT: Color = Color::INDIGO;
const BTN_BORDER_HOVER: Color = Color::PINK;
const BTN_BORDER_SELECTED: Color = Color::RED;
const MAX_NUM_NFT: usize = 6;

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SplashScreen), menu_splash_screen)
            .add_systems(OnEnter(AppState::GameMenuMain), menu_game_create)
            .add_systems(OnExit(AppState::GameMenuMain), menu_game_create_complete)
            .add_systems(OnEnter(AppState::GameLevelUp), menu_level_up)
            .add_systems(OnEnter(AppState::GamePaused), menu_pause)
            .add_systems(OnEnter(AppState::GameOver), menu_game_over)
            .add_systems(Update, menu_input_system)
            .add_systems(
                Update,
                (on_main_menu_scroll, on_main_menu_button_action)
                    .run_if(in_state(AppState::GameMenuMain)),
            )
            .add_systems(
                Update,
                menu_blink_system.run_if(in_state(AppState::SplashScreen)),
            )
            .add_systems(
                Update,
                menu_blink_system.run_if(in_state(AppState::GamePaused)),
            )
            .add_systems(
                Update,
                menu_blink_system.run_if(in_state(AppState::GameOver)),
            )
            .add_systems(
                Update,
                on_level_up_menu_button_action.run_if(in_state(AppState::GameLevelUp)),
            )
            .insert_resource(MenuGameConfig::default());
    }
}

#[derive(Resource, Debug, Default)]
pub struct MenuGameConfig {
    pub hero: HeroType,
    pub map: MapId,
    pub nft_list: Vec<String>,
}

#[derive(Component)]
struct CheckBox {
    pub nft_id: String,
    pub checked: bool,
}
impl CheckBox {
    fn make_checkbox(nft_id: String) -> Self {
        Self {
            nft_id,
            checked: false,
        }
    }
}

#[derive(Component)]
struct HeroSelectButton {
    hero_type: HeroType,
}

#[derive(Component)]
struct MapSelectButton {
    map_id: MapId,
}

#[derive(Component)]
struct SelectedElement {}

fn on_main_menu_button_action(
    mut commands: Commands,
    mut query_action_button: Query<(&Interaction, &mut MenuButtonAction), Changed<Interaction>>,
    mut query_btn: Query<
        (
            &Interaction,
            Entity,
            &mut BorderColor,
            Option<&mut MapSelectButton>,
            Option<&mut HeroSelectButton>,
            Option<&mut CheckBox>,
        ),
        (Changed<Interaction>, Without<SelectedElement>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut tx_exit: EventWriter<AppExit>,
    mut state: ResMut<MenuGameConfig>,
    mut selected_hero: Query<
        (Entity, &mut BorderColor),
        (
            With<SelectedElement>,
            With<HeroSelectButton>,
            Without<MapSelectButton>,
        ),
    >,
    mut selected_map: Query<
        (Entity, &mut BorderColor),
        (
            With<SelectedElement>,
            With<MapSelectButton>,
            Without<HeroSelectButton>,
        ),
    >,
    mut tx_gameplay: EventWriter<GameplayEffectEvent>,
) {
    // Check for Quit & Play button interaction
    for (interaction, mut action) in query_action_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => match *action {
                MenuButtonAction::Play => next_state.set(AppState::GameInitializing),
                MenuButtonAction::Quit => tx_exit.send(AppExit),
            },
            _ => {}
        }
    }

    for (interaction, entity, mut border, mut map, mut hero, mut checkbox) in query_btn.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Some(map) = map {
                    for (entity, mut border) in selected_map.iter_mut() {
                        border.0 = BTN_BORDER_DEFAULT;
                        commands.entity(entity).remove::<SelectedElement>();
                    }
                    state.map = map.map_id.clone();
                    tx_gameplay.send(GameplayEffectEvent::MapSelected(map.map_id.clone()));
                    commands.entity(entity).insert(SelectedElement {});
                }
                if let Some(hero) = hero {
                    for (entity, mut border) in selected_hero.iter_mut() {
                        border.0 = BTN_BORDER_DEFAULT;
                        commands.entity(entity).remove::<SelectedElement>();
                    }
                    state.hero = hero.hero_type.clone();
                    tx_gameplay.send(GameplayEffectEvent::HeroSelected(hero.hero_type.clone()));
                    commands.entity(entity).insert(SelectedElement {});
                }
                if let Some(mut checkbox) = checkbox {
                    // Limit the number of wearable NFTs
                    if state.nft_list.len() > (MAX_NUM_NFT - 1) && !checkbox.checked {
                        eprintln!(
                            "CAN NOT EQUIP ANY MORE NFTs. Current count: {}",
                            state.nft_list.len()
                        );
                    } else {
                        //TODO too many indentations, refactor this
                        checkbox.checked = !checkbox.checked;
                        if checkbox.checked {
                            tx_gameplay.send(GameplayEffectEvent::NFTEquipped(
                                checkbox.nft_id.clone(),
                                ItemType::BonkInuBattleBracers, //TODO depends on NFT
                            ));
                            state.nft_list.push(checkbox.nft_id.clone());
                        } else {
                            tx_gameplay
                                .send(GameplayEffectEvent::NFTUnEquipped(checkbox.nft_id.clone()));
                            state.nft_list.retain(|id| id != &checkbox.nft_id)
                        }
                    }
                }

                // Attach or remove the SelectedElement tag from this entity
                border.0 = BTN_BORDER_SELECTED;
            }
            Interaction::Hovered => {
                if !checkbox.is_some() || !checkbox.unwrap().checked {
                    border.0 = BTN_BORDER_HOVER;
                }
            }
            Interaction::None => {
                if !checkbox.is_some() || !checkbox.unwrap().checked {
                    border.0 = BTN_BORDER_DEFAULT
                }
            }
        }
    }
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn menu_game_create(
    mut commands: Commands,
    assets: Res<UiAssets>,
    mut state: ResMut<MenuGameConfig>,
) {
    // Reset state
    state.hero = HeroType::Pepe;
    state.map = MapId::Map1;
    state.nft_list = vec![];

    // Screen wrapper
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::GameMenuMain],
            },
        ))
        .with_children(|parent| {
            // Wrapper for the main content
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(85.),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| wrapper_content(parent, &assets));

            // Wrapper for the footer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(15.),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    background_color: Color::OLIVE.into(),
                    ..Default::default()
                })
                .with_children(|parent| wrapper_footer(parent, &assets));
        });
}

/// Process the UI data so we can send it to the game setup
fn menu_game_create_complete(nft_list: Query<&CheckBox>, mut state: ResMut<MenuGameConfig>) {
    state.nft_list = get_equipped_nfts(&nft_list);

    eprintln!("Configured GameState:: {:?}", state);
}

/// A query to find all the NFTs that have been equipped
fn get_equipped_nfts(nft_list: &Query<&CheckBox>) -> Vec<String> {
    let mut res = vec![];
    for CheckBox { nft_id, checked } in nft_list.iter() {
        if !checked {
            continue;
        }
        res.push(nft_id.clone());
    }

    res
}

/// Wrapper for the game menu content, this is split into two sides, on the left the hero and map are selected and on the right the NFTs can be equipped
fn wrapper_content(parent: &mut ChildBuilder, assets: &UiAssets) {
    // Wrapper for the left side
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::DARK_GREEN.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Wrapper for hero selection
            wrapper_hero_selector(parent, assets);

            // Wrapper for map selection
            wrapper_map_selector(parent, assets);
        });

    // Wrapper for the right side
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::TEAL.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Wrapper for the list of owned NFTs
            wrapper_nft_list(parent, assets);
        });
}

/// This section is about choosing a hero
fn wrapper_hero_selector(parent: &mut ChildBuilder, assets: &UiAssets) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Hero Selection",
                    TextStyle {
                        font: assets.font_primary.clone(),
                        font_size: 30.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    for hero in HeroType::iter() {
                        let ui_img = assets.heroes.get(&hero).unwrap();
                        spawn_bordered_button_with_bundle(
                            parent,
                            ui_img.clone(),
                            HeroSelectButton { hero_type: hero },
                        );
                    }
                });
        });
}

/// This section is about selecting a map
fn wrapper_map_selector(parent: &mut ChildBuilder, assets: &UiAssets) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::YELLOW_GREEN.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Map Selection",
                    TextStyle {
                        font: assets.font_primary.clone(),
                        font_size: 30.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    for map in MapId::iter() {
                        let ui_img = assets.maps.get(&map).unwrap();
                        spawn_bordered_button_with_bundle(
                            parent,
                            ui_img.clone(),
                            MapSelectButton { map_id: map },
                        );
                    }
                });
        });
}

/// This section is about equipping cNFTs
fn wrapper_nft_list(parent: &mut ChildBuilder, assets: &UiAssets) {
    let nfts_from_api_todo: Vec<(&str, &str)> = vec![
        (
            "7hgJd62T7j1KarSLU7vsr9tGdKZ1Q2qUXWZsg3pVkPjb",
            "BonkInu's Battle Gloves (+10% Attack Speed)",
        ),
        (
            "2k4GvRj2zrYx9DWuT5haXxPpoQa1kp3UDWuX9gqJN9Wj",
            "Pepe's Crypto Wings (+8% Movement Speed)",
        ),
        (
            "8EHJpX2PpGqFvqBkCfz6FjWpvvDqYqLUrVpPZj5H1DPw",
            "MadLads Market Mayhem (+15% Damage Boost)",
        ),
        (
            "3gF5g6GJ2PZpVgJ8ZyUqufuB5vG2aJMr5Njh6P6J3KTm",
            "CryptoKitty Fever (+12% Critical Hit Chance)",
        ),
        (
            "4iWJVp7HPgqKYbNZDv3XYgZaQsZ9K6q8jq9hJjr3XzYo",
            "BonkInu's Blockchain Brawl (+20% Token Harvesting Rate)",
        ),
        (
            "9u2aVuRjaG69paRgE2Z1wzZV9N2UP8mXSWPjq2tV3vzv",
            "Pepe's Pixelated Portfolio (+5% Defense)",
        ),
        (
            "5rv2aSJf2zbdXjW9jFqK9vKYvr8PrwrG9FpHb5t4d9jS",
            "MadLads Moonshot Madness (+18% Dodge Chance)",
        ),
        (
            "6pJ4gRzvqNZquHZG1ZaS42GXDqCkPZyUTyHXaqgVTXfD",
            "CryptoKitties Wisdom Tonic (+7% Experience Gain)",
        ),
        (
            "1gVyaVSK87y8yvNWa8WVP8yZavGqU6W1Kv1uF2zYvVZ9",
            "BonkInu's DeFi Derby (+25% DeFi Yield)",
        ),
        (
            "2zPvTm2jaY4P4zJy2ZwJ2a8hj7JwF5eXy2D5r5wF5u7z",
            "Pepe's NFT Nexus (+10% Rarity Find)",
        ),
        (
            "8PmRJmP4rYquqGvzSj3m7yZAyqn3YqpWy2myqSguZwyo",
            "MadLads MetaVerse Mayhem (+15% Stamina Regeneration)",
        ),
        (
            "3fSvZVqW9Jyo1ZfzYbJgquB9mBvzaPjW2JgZ2y9JkzP6",
            "CryptoKitties Caper (+10% Catnip Collection)",
        ),
        (
            "4rH9ZygvGqzVT7Mz2gU1Zjy8Tn8njh4n1pW1uWZqfUtu",
            "BonkInu's Bonanza Blitz (+12% Resource Gathering)",
        ),
        (
            "9V4ZvgUwVgJ8aTzo87zvq8MfJvnYXZpJ1J3VgZYgXYc",
            "Pepe's Precious Tokens (+15% Gold Discovery)",
        ),
        (
            "5XbGQvQ1zZoXHhPKbGUQ6iZfHHzWXe7Jg9muvzqJpSV",
            "MadLads Meme Minefield (+8% Energy Regeneration)",
        ),
        (
            "1cNaZY2ZXnqw9jJcqjuaJqCzJ7G5nzqm4V4UZMgPwYX",
            "CryptoKitties Crypto Carnival (+10% Kitty Breeding Speed)",
        ),
        (
            "2RiGzny4vY3qvKXV9aR1mH8FyUJ4v89UxZ6TqGhXaE2M",
            "BonkInu's Bullish Battle (+20% Investment Returns)",
        ),
        (
            "8ZR3rX8zynWj59nvqZYgSK9ZMqPQ2v9hVnVjZAqzAVZN",
            "Pepe's Profits Parade (+5% Market Influence)",
        ),
        (
            "3J2vXV9u2JqyWZgKXY2UuZfaSWzV2JjKrWUvTqSnJyNW",
            "MadLads Market Mingle (+15% Social Interaction Bonus)",
        ),
        (
            "4u1KZJqKJZq1v3nTgrGhFm7j5jtmvZr2wZ5mVaSP2qys",
            "CryptoKitties Kitty Kingdom (+10% Kingdom Prosperity)",
        ),
    ];

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Display the game name
            parent.spawn(
                TextBundle::from_section(
                    "Equip your cNFTs",
                    TextStyle {
                        font: assets.font_primary.clone(),
                        font_size: 30.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        height: Val::Percent(100.),
                        overflow: Overflow::clip_y(),
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Moving panel
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Start,
                                    ..default()
                                },
                                ..default()
                            },
                            ScrollingList::default(),
                            AccessibilityNode(NodeBuilder::new(Role::List)),
                        ))
                        .with_children(|parent| {
                            for (id, text) in nfts_from_api_todo {
                                list_item_selectable(parent, assets, id, text);
                            }
                        });
                });
        });
}

/// This is a select box used for equipping a particular cNFT
fn list_item_selectable(parent: &mut ChildBuilder, assets: &UiAssets, id: &str, text: &str) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(32f32),
                        height: Val::Px(32f32),
                        margin: UiRect::all(Val::Px(2.)),
                        border: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::INDIGO),
                    ..Default::default()
                })
                .insert(CheckBox::make_checkbox(id.to_string()))
                .with_children(|parent| {
                    spawn_nested_icon(parent, Color::GOLD, assets.checkbox_o.clone(), 28.0);
                });

            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 16.,
                        ..default()
                    },
                ),
                Label,
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ));
        });
}

/// This wrapper contains the quit and play buttons
fn wrapper_footer(parent: &mut ChildBuilder, assets: &UiAssets) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font: assets.font_primary.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    let icon = assets.buff_1.clone();
                    parent.spawn(ImageBundle {
                        style: button_icon_style.clone(),
                        image: icon,
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::End,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    let icon = assets.buff_1.clone();
                    parent.spawn(ImageBundle {
                        style: button_icon_style,
                        image: icon,
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Play", button_text_style));
                });
        });
}

/// These buttons are used for the hero and map selection
fn spawn_bordered_button_with_bundle(
    parent: &mut ChildBuilder,
    ui_img: UiImage,
    bundle: impl Bundle,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Px(64f32),
                height: Val::Px(64f32),
                margin: UiRect::all(Val::Px(5.)),
                border: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            border_color: BorderColor(Color::INDIGO),
            ..Default::default()
        })
        .insert(bundle)
        .with_children(|parent| {
            spawn_nested_icon(parent, Color::GOLD, ui_img.clone(), 56.0);
        });
}

/// Spawns an icon for some button
fn spawn_nested_icon(
    parent: &mut ChildBuilder,
    background_color: Color,
    ui_img: UiImage,
    size: f32,
) {
    parent
        .spawn(NodeBundle {
            background_color: BackgroundColor(background_color),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                ui_img,
            ));
        });
}

/// Flashes some text at a fixed interval
fn menu_blink_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DrawBlinkTimer, &Visibility)>,
) {
    for (entity, mut timer, visibility) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let new_visibility = if visibility == Visibility::Visible {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
            commands.entity(entity).insert(new_visibility);
        }
    }
}

/// Handle input specific to the menu
fn menu_input_system(
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tx_exit: EventWriter<AppExit>,
    actions: Query<&ActionState<GameAction>>,
) {
    let action = actions.single();

    if state.get() != &AppState::SplashScreen && action.just_pressed(GameAction::Cancel) {
        next_state.set(AppState::SplashScreen);
    } else {
        match state.get() {
            AppState::SplashScreen => {
                if action.just_pressed(GameAction::Confirm) {
                    next_state.set(AppState::GameMenuMain);
                }
                if action.just_pressed(GameAction::Cancel) {
                    tx_exit.send(AppExit);
                }
            }
            AppState::GameMenuMain => {
                if action.just_pressed(GameAction::Confirm) {
                    next_state.set(AppState::GameInitializing);
                }
            }
            AppState::GameInitializing => {
                next_state.set(AppState::GameRunning);
            }
            AppState::GameRunning => {
                if action.just_pressed(GameAction::Pause) {
                    next_state.set(AppState::GamePaused);
                }
            }
            AppState::GamePaused => {
                if action.just_pressed(GameAction::Pause) {
                    next_state.set(AppState::GameRunning);
                }
            }
            AppState::GameLevelUp => {}
            AppState::GameOver => {
                if action.just_pressed(GameAction::Confirm) {
                    next_state.set(AppState::SplashScreen);
                }
            }
        }
    }
}

/// Scroll handler for the cNFT list
fn on_main_menu_scroll(
    mut rx_mouse: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in rx_mouse.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
