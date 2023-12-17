use crate::heroes::{HeroType, Levels};
use crate::plugins::assets::UiAssets;
use crate::state::{AppState, ForState};
use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::app::AppExit;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SplashScreen), menu_splash_screen)
            .add_systems(OnEnter(AppState::GameCreate), menu_game_create)
            .add_systems(OnExit(AppState::GameCreate), menu_game_create_complete)
            .add_systems(OnEnter(AppState::GameOver), menu_game_over)
            .add_systems(
                Update,
                (
                    mouse_scroll,
                    on_checkbox_interaction,
                    on_button_interaction,
                    menu_input_system,
                    menu_blink_system,
                ),
            )
            .add_systems(Startup, setup)
            .insert_resource(GameConfigState::default());
    }
}

fn setup(mut _commands: Commands) {
    //TODO
}

fn menu_splash_screen(mut commands: Commands, assets: ResMut<UiAssets>) {
    let font_color = Color::rgb_u8(0x92, 0xA6, 0x8A);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::SplashScreen],
            },
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Speedrun Survivors",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: font_color,
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: font_color,
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
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
struct HeroButton {
    hero_type: HeroType,
}

fn on_checkbox_interaction(
    mut query: Query<(&Interaction, &mut CheckBox, &mut UiImage), Changed<Interaction>>,
    assets: Res<UiAssets>,
) {
    for (interaction, mut checkbox, mut image) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                eprintln!("Checkbox clicked {}, {:?}", checkbox.nft_id, image.texture);

                checkbox.checked = !checkbox.checked;

                //TODO doesnt work
                if checkbox.checked {
                    *image = assets.checkbox_x.clone();
                } else {
                    *image = assets.checkbox_o.clone();
                }
            }
            _ => {}
        }
    }
}

fn on_button_interaction(
    mut query_action_button: Query<(&Interaction, &mut MenuButtonAction), Changed<Interaction>>,
    mut query_hero_button: Query<
        (&Interaction, &mut BorderColor, &mut HeroButton),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut state: ResMut<GameConfigState>,
) {
    for (interaction, mut action) in query_action_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => match *action {
                MenuButtonAction::Play => next_state.set(AppState::GameRunning),
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
            },
            _ => {}
        }
    }

    for (interaction, mut border, mut hero) in query_hero_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                state.hero = hero.hero_type.clone();
                border.0 = Color::RED; //TODO doesnt really work as hover will immediatly overwrite and even if thats checked against, would need to be flipped back if another is selected
            }
            Interaction::Hovered => {
                border.0 = Color::PINK;
            }
            Interaction::None => {
                border.0 = Color::INDIGO;
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
    mut state: ResMut<GameConfigState>,
) {
    // Reset state
    state.hero = HeroType::Pepe;
    state.level = 1;
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
                states: vec![AppState::GameCreate],
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
fn menu_game_create_complete(nft_list: Query<&CheckBox>, mut state: ResMut<GameConfigState>) {
    for CheckBox { nft_id, checked } in nft_list.iter() {
        if !checked {
            continue;
        }
        state.nft_list.push(nft_id.clone());
    }

    eprintln!("Configured GameState:: {:?}", state);
}

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

            // Wrapper for level selection
            wrapper_level_selector(parent, assets);
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
            background_color: Color::CRIMSON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Wrapper for the list of owned NFTs
            wrapper_nft_list(parent, assets);
        });
}

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
                    for hero in HeroType::into_iter() {
                        let ui_img = assets.heroes.get(&hero).unwrap();
                        spawn_hero_select_box(parent, ui_img.clone(), &hero);
                    }
                });
        });
}

fn wrapper_level_selector(parent: &mut ChildBuilder, assets: &UiAssets) {
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
                    "Level Selection",
                    TextStyle {
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
                    for level in Levels::into_iter() {
                        let ui_img = assets.levels.get(&level).unwrap();
                        spawn_level_select_box(parent, ui_img.clone(), &level);
                    }
                });
        });
}

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
                        padding: UiRect::all(Val::Px(2.)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::INDIGO),
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

fn spawn_equipment_row(parent: &mut ChildBuilder, assets: &UiAssets, slots: [u32; 3]) {
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
            for slot in slots {
                let ui_img = assets.buff_1.clone();
                spawn_equipment_selected_box(parent, ui_img.clone(), slot);
            }
        });
}

fn spawn_equipment_selected_box(parent: &mut ChildBuilder, ui_img: UiImage, slot: u32) -> Entity {
    let mut node = parent.spawn(ButtonBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(64f32),
            height: Val::Px(64f32),
            margin: UiRect::all(Val::Px(5.)),
            padding: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::INDIGO),
        ..Default::default()
    });

    node.with_children(|parent| {
        spawn_nested_icon(parent, Color::GOLD, ui_img.clone(), 56.0);
    })
    .id()
}

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
                    parent.spawn(TextBundle::from_section("Play!", button_text_style));
                });
        });
}

fn spawn_hero_select_box(parent: &mut ChildBuilder, ui_img: UiImage, hero_type: &HeroType) {
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
        .insert(HeroButton {
            hero_type: hero_type.clone(),
        })
        .with_children(|parent| {
            spawn_nested_icon(parent, Color::GOLD, ui_img.clone(), 56.0);
        });
}

fn spawn_level_select_box(
    parent: &mut ChildBuilder,
    ui_img: UiImage,
    level_type: &Levels,
) -> Entity {
    let mut node = parent.spawn(ButtonBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(64f32),
            height: Val::Px(64f32),
            margin: UiRect::all(Val::Px(5.)),
            padding: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::INDIGO),
        ..Default::default()
    });

    node.with_children(|parent| {
        spawn_nested_icon(parent, Color::GOLD, ui_img.clone(), 56.0);
    })
    .id()
}

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

fn menu_game_over(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![AppState::GameOver],
            },
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100.0,
                        color: Color::rgb_u8(0xAA, 0x22, 0x22),
                    },
                ),
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            ));
        });
}

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

fn menu_input_system(
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    //if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Comma) {
    if state.get() != &AppState::SplashScreen && keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::SplashScreen);
    } else {
        match state.get() {
            AppState::SplashScreen => {
                if keys.just_pressed(KeyCode::Return) {
                    next_state.set(AppState::GameCreate);
                }
                if keys.just_pressed(KeyCode::Escape) {
                    app_exit_events.send(AppExit);
                }
            }
            AppState::GameCreate => {
                if keys.just_pressed(KeyCode::Return) {
                    next_state.set(AppState::GameRunning);
                }
            }
            AppState::GameOver => {
                if keys.just_pressed(KeyCode::Return) {
                    next_state.set(AppState::SplashScreen);
                }
            }
            AppState::GameRunning => {}
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
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

#[derive(Debug, Resource)]
pub struct GameConfigState {
    pub hero: HeroType,
    pub level: u64,
    pub nft_list: Vec<String>,
}

impl Default for GameConfigState {
    fn default() -> Self {
        Self {
            hero: HeroType::BonkInu,
            level: 0,
            nft_list: vec![],
        }
    }
}
